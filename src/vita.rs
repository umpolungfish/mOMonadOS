//! vita — the trunk's mouth and gate, native on mOMonadOS.
//!
//! The SIXTEEN_3 Logic Lattice transformer (vae_vita's 26-step protocol
//! forward pass) reimplemented in pure no_std + alloc + libm, loading the
//! host-trained weights baked by `vita-bake` (include_bytes at build). The
//! alphabet is DERIVED from imasm_core exactly as vita_native derives it —
//! same ids, same faces, one kernel. The gate is imasm_core itself: the
//! classic close condition (check::word_verdict) and, when the word carries
//! tri tokens, the SIXTEEN_3 register machine's tri-ancestral verdict.
//!
//! One certified turn per call: ⊢⟨word⟩◇⊞⟨⟩●⟨readout⟩⊣ — the query arm
//! sampled under the fork discipline, the μ arm the trunk's self-readout,
//! and the verdict spoken by the kernel that lives on this machine.
//!
//! Build with the `vita` feature after baking weights to vita_weights.bin.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use imasm_core::check;
use imasm_core::classic::Token as CTok;
use imasm_core::imasm16_3::{self, Token16_3, ALL_TOKENS};

static WEIGHTS: &[u8] = include_bytes!("../vita_weights.bin");

// ── the derived alphabet (identical derivation to vita_native) ───────────

fn vocab() -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for t in ALL_TOKENS {
        if t == Token16_3::Fsplit3 {
            v.push(CTok::Fsplit.code().to_string());
            v.push(CTok::Ffuse.code().to_string());
        }
        v.push(t.glyph().to_string());
    }
    for s in imasm_core::state_order() {
        v.push(s.glyph().to_string());
    }
    v.push("⊤".to_string());
    for p in ["Ð", "Þ", "Ř", "Φ", "ƒ", "Ç", "Γ", "ɢ", "Ħ", "Σ", "Ω"] {
        v.push(p.to_string());
    }
    for c in 0x10450..=0x1047F_u32 {
        v.push(char::from_u32(c).unwrap().to_string());
    }
    for s in ["⟨", "⟩", "·", "⋈", "|", " ", "\n", "?"] {
        v.push(s.to_string());
    }
    v
}

fn id_of(voc: &[String], g: &str) -> u32 {
    voc.iter().position(|e| e == g).unwrap_or(voc.len() - 1) as u32
}

fn decode(voc: &[String], ids: &[u32]) -> String {
    ids.iter().map(|&i| voc.get(i as usize).map(|s| s.as_str()).unwrap_or("?")).collect()
}

// ── the baked tensor store ────────────────────────────────────────────────

struct Store {
    tensors: BTreeMap<String, (Vec<usize>, Vec<f32>)>,
}

impl Store {
    fn load() -> Option<Store> {
        let b = WEIGHTS;
        if b.len() < 8 || &b[0..4] != b"VITA" {
            return None;
        }
        let mut off = 4;
        let rd_u32 = |b: &[u8], o: &mut usize| -> u32 {
            let v = u32::from_le_bytes([b[*o], b[*o + 1], b[*o + 2], b[*o + 3]]);
            *o += 4;
            v
        };
        let count = rd_u32(b, &mut off) as usize;
        let mut tensors = BTreeMap::new();
        for _ in 0..count {
            let nl = rd_u32(b, &mut off) as usize;
            let name = core::str::from_utf8(&b[off..off + nl]).ok()?.to_string();
            off += nl;
            let nd = rd_u32(b, &mut off) as usize;
            let mut dims = Vec::with_capacity(nd);
            for _ in 0..nd {
                dims.push(rd_u32(b, &mut off) as usize);
            }
            let n: usize = dims.iter().product();
            let mut data = Vec::with_capacity(n);
            for _ in 0..n {
                data.push(f32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]]));
                off += 4;
            }
            tensors.insert(name, (dims, data));
        }
        Some(Store { tensors })
    }

    fn t(&self, name: &str) -> &(Vec<usize>, Vec<f32>) {
        self.tensors.get(name).expect("baked tensor missing")
    }
    fn has(&self, name: &str) -> bool {
        self.tensors.contains_key(name)
    }
}

// ── minimal f32 math (rows = t, cols = d, row-major) ─────────────────────

fn expf(x: f32) -> f32 {
    libm::expf(x)
}

/// y = x · Wᵀ (+ bias). W is (out, in) as candle's Linear stores it.
fn linear(x: &[f32], t: usize, din: usize, w: &[f32], dout: usize, bias: Option<&[f32]>) -> Vec<f32> {
    let mut y = vec![0f32; t * dout];
    for r in 0..t {
        for o in 0..dout {
            let mut acc = if let Some(b) = bias { b[o] } else { 0.0 };
            let xr = &x[r * din..(r + 1) * din];
            let wr = &w[o * din..(o + 1) * din];
            for i in 0..din {
                acc += xr[i] * wr[i];
            }
            y[r * dout + o] = acc;
        }
    }
    y
}

fn rms_norm(x: &mut [f32], t: usize, d: usize, w: &[f32]) {
    for r in 0..t {
        let row = &mut x[r * d..(r + 1) * d];
        let ms: f32 = row.iter().map(|v| v * v).sum::<f32>() / d as f32;
        let inv = 1.0 / libm::sqrtf(ms + 1e-6);
        for i in 0..d {
            row[i] = row[i] * inv * w[i];
        }
    }
}

fn sigmoid(v: f32) -> f32 {
    1.0 / (1.0 + expf(-v))
}

/// Rope on one head row (head_dim 12, half 6): the ring position rotation.
fn rope(row: &mut [f32], pos: usize) {
    let half = HD / 2;
    for i in 0..half {
        let theta = 1.0 / libm::powf(10000.0, 2.0 * i as f32 / HD as f32);
        let a = pos as f32 * theta;
        let (c, s) = (libm::cosf(a), libm::sinf(a));
        let x1 = row[i];
        let x2 = row[half + i];
        row[i] = x1 * c - x2 * s;
        row[half + i] = x1 * s + x2 * c;
    }
}

/// AFWD/AREV: the parameterless ring shift on feature halves, θ = ±0.1.
fn ring_shift(x: &mut [f32], t: usize, forward: bool) {
    let half = D / 2;
    let theta: f32 = if forward { 0.1 } else { -0.1 };
    let (c, s) = (libm::cosf(theta), libm::sinf(theta));
    for r in 0..t {
        for i in 0..half {
            let a = x[r * D + i];
            let b = x[r * D + half + i];
            x[r * D + i] = a * c - b * s;
            x[r * D + half + i] = a * s + b * c;
        }
    }
}

// ── the 26-step protocol forward pass ─────────────────────────────────────

const D: usize = 144;
const HEADS: usize = 12;
const HD: usize = D / HEADS;

pub struct Vita {
    store: Store,
    voc: Vec<String>,
}

impl Vita {
    pub fn load() -> Option<Vita> {
        Some(Vita { store: Store::load()?, voc: vocab() })
    }

    /// Logits at the last position for the id sequence (batch 1).
    fn forward_last(&self, ids: &[u32]) -> Vec<f32> {
        let s = &self.store;
        let t = ids.len();
        let vocab_n = self.voc.len();
        // embed
        let (_, emb) = s.t("embed.weight");
        let mut x = vec![0f32; t * D];
        for (r, &id) in ids.iter().enumerate() {
            x[r * D..(r + 1) * D].copy_from_slice(&emb[id as usize * D..(id as usize + 1) * D]);
        }
        let mut stack: Vec<Vec<f32>> = Vec::new();

        for step in 0..26 {
            let pre = |n: &str| format!("step{step}.{n}");
            if s.has(&pre("down.weight")) {
                // IMSCRIB: S¹¹ core
                let (_, wd) = s.t(&pre("down.weight"));
                let (_, wu) = s.t(&pre("up.weight"));
                let mut z = linear(&x, t, D, wd, 12, None);
                for r in 0..t {
                    let row = &mut z[r * 12..(r + 1) * 12];
                    let n = libm::sqrtf(row.iter().map(|v| v * v).sum::<f32>()) + 1e-8;
                    for v in row.iter_mut() {
                        *v /= n;
                    }
                }
                let up = linear(&z, t, 12, wu, D, None);
                for i in 0..t * D {
                    x[i] += up[i];
                }
            } else if s.has(&pre("fork.weight")) {
                // CLINK: split-fuse attention with rope
                let (_, wn) = s.t(&pre("norm.weight"));
                let (_, wf) = s.t(&pre("fork.weight"));
                let (_, wu) = s.t(&pre("fuse.weight"));
                let mut xn = x.clone();
                rms_norm(&mut xn, t, D, wn);
                let qkv = linear(&xn, t, D, wf, 3 * D, None);
                let mut ctx = vec![0f32; t * D];
                for h in 0..HEADS {
                    // q,k,v for this head, rope-rotated
                    let mut q = vec![0f32; t * HD];
                    let mut k = vec![0f32; t * HD];
                    let mut v = vec![0f32; t * HD];
                    for r in 0..t {
                        for i in 0..HD {
                            q[r * HD + i] = qkv[r * 3 * D + h * HD + i];
                            k[r * HD + i] = qkv[r * 3 * D + D + h * HD + i];
                            v[r * HD + i] = qkv[r * 3 * D + 2 * D + h * HD + i];
                        }
                    }
                    for r in 0..t {
                        rope(&mut q[r * HD..(r + 1) * HD], r);
                        rope(&mut k[r * HD..(r + 1) * HD], r);
                    }
                    let scale = 1.0 / libm::sqrtf(HD as f32);
                    for r in 0..t {
                        // causal scores over 0..=r
                        let mut sc = vec![0f32; r + 1];
                        let mut mx = f32::NEG_INFINITY;
                        for c in 0..=r {
                            let mut a = 0f32;
                            for i in 0..HD {
                                a += q[r * HD + i] * k[c * HD + i];
                            }
                            a *= scale;
                            sc[c] = a;
                            if a > mx {
                                mx = a;
                            }
                        }
                        let mut sum = 0f32;
                        for c in 0..=r {
                            sc[c] = expf(sc[c] - mx);
                            sum += sc[c];
                        }
                        for c in 0..=r {
                            let w = sc[c] / sum;
                            for i in 0..HD {
                                ctx[r * D + h * HD + i] += w * v[c * HD + i];
                            }
                        }
                    }
                }
                let fused = linear(&ctx, t, D, wu, D, None);
                for i in 0..t * D {
                    x[i] += fused[i];
                }
            } else if s.has(&pre("floor.weight")) {
                let (_, wn) = s.t(&pre("floor.weight"));
                rms_norm(&mut x, t, D, wn);
            } else if s.has(&pre("delta.weight")) {
                stack.push(x.clone());
                let (_, w) = s.t(&pre("delta.weight"));
                let dx = linear(&x, t, D, w, D, None);
                for i in 0..t * D {
                    x[i] += dx[i];
                }
            } else if s.has(&pre("mu.weight")) {
                let right = stack.pop().unwrap_or_else(|| x.clone());
                let (_, w) = s.t(&pre("mu.weight"));
                let mut joined = vec![0f32; t * 2 * D];
                for r in 0..t {
                    joined[r * 2 * D..r * 2 * D + D].copy_from_slice(&x[r * D..(r + 1) * D]);
                    joined[r * 2 * D + D..(r + 1) * 2 * D].copy_from_slice(&right[r * D..(r + 1) * D]);
                }
                let mx = linear(&joined, t, 2 * D, w, D, None);
                for i in 0..t * D {
                    x[i] += mx[i];
                }
            } else if s.has(&pre("t_gate.weight")) {
                let (_, w) = s.t(&pre("t_gate.weight"));
                let (_, b) = s.t(&pre("t_gate.bias"));
                let g = linear(&x, t, D, w, D, Some(b));
                for i in 0..t * D {
                    x[i] *= sigmoid(g[i]);
                }
            } else if s.has(&pre("f_gate.weight")) {
                let (_, w) = s.t(&pre("f_gate.weight"));
                let (_, b) = s.t(&pre("f_gate.bias"));
                let g = linear(&x, t, D, w, D, Some(b));
                for i in 0..t * D {
                    x[i] *= 1.0 - sigmoid(g[i]);
                }
            } else if s.has(&pre("hold.weight")) {
                let arm = stack.pop().unwrap_or_else(|| x.clone());
                let (_, w) = s.t(&pre("hold.weight"));
                let mut held = vec![0f32; t * D];
                for i in 0..t * D {
                    let a = arm[i];
                    held[i] = x[i] * (a * sigmoid(a)); // silu
                }
                let hx = linear(&held, t, D, w, D, None);
                for i in 0..t * D {
                    x[i] += hx[i];
                }
            } else if s.has(&pre("record.weight")) {
                let (_, w) = s.t(&pre("record.weight"));
                let rx = linear(&x, t, D, w, D, None);
                for i in 0..t * D {
                    x[i] += rx[i];
                }
            } else {
                // VINIT (the embedding above) or AFWD/AREV: the parameterless
                // ring shift. Steps 6 and 14 are the two rotations.
                if step == 6 || step == 14 {
                    ring_shift(&mut x, t, step == 6);
                }
            }
        }
        let (_, wh) = s.t("head.weight");
        let last = &x[(t - 1) * D..t * D];
        let mut logits = vec![0f32; vocab_n];
        for o in 0..vocab_n {
            let mut a = 0f32;
            for i in 0..D {
                a += last[i] * wh[o * D + i];
            }
            logits[o] = a;
        }
        logits
    }

    /// Speak one certified turn and gate it with the on-board kernel.
    /// Returns the rendered report line.
    pub fn speak_turn(&self, seed: u64, temp: f32, word_cap: usize) -> String {
        let voc = &self.voc;
        let open_id = id_of(voc, "⊢");
        let seal_id = id_of(voc, "⊣");
        let split_id = id_of(voc, "◇");
        let fuse_id = id_of(voc, "●");
        let lang_id = id_of(voc, "⟨");
        let rang_id = id_of(voc, "⟩");
        let dot_id = id_of(voc, "·");
        let query_legal: Vec<u32> =
            "⊢⊣><=⊙◇●∈∋+×⊞~≁¬".chars().map(|c| id_of(voc, &c.to_string())).collect();
        let mut answer_legal: Vec<u32> = imasm_core::state_order()
            .iter()
            .map(|s| id_of(voc, &s.glyph().to_string()))
            .collect();
        answer_legal.extend([id_of(voc, "⊙"), id_of(voc, "×"), dot_id, rang_id]);
        let word_initial = [open_id, id_of(voc, "⊙")];

        let mut rng = seed | 1;
        let mut next_f32 = move || -> f32 {
            let mut z = rng;
            z ^= z >> 12;
            z ^= z << 25;
            z ^= z >> 27;
            rng = z;
            ((z.wrapping_mul(0x2545F4914F6CDD1D) >> 40) as f32) / ((1u64 << 24) as f32)
        };
        let mut sample = |logits: &[f32], mask: &dyn Fn(u32) -> bool| -> u32 {
            let mut mx = f32::NEG_INFINITY;
            for (i, &l) in logits.iter().enumerate() {
                if mask(i as u32) && l > mx {
                    mx = l;
                }
            }
            let mut exps = vec![0f32; logits.len()];
            let mut sum = 0f32;
            for (i, &l) in logits.iter().enumerate() {
                if mask(i as u32) {
                    exps[i] = expf((l - mx) / temp);
                    sum += exps[i];
                }
            }
            let mut r = next_f32() * sum;
            for (i, &e) in exps.iter().enumerate() {
                if e > 0.0 {
                    r -= e;
                    if r <= 0.0 {
                        return i as u32;
                    }
                }
            }
            logits.len() as u32 - 1
        };

        // ── δ arm ─────────────────────────────────────────────────────────
        let mut ids: Vec<u32> = vec![open_id, lang_id];
        let mut depth = 0usize;
        let mut sealed = false;
        loop {
            let logits = self.forward_last(&ids);
            let qlen = ids.len() - 2;
            let id = if sealed || qlen + depth + 2 >= word_cap {
                if depth > 0 {
                    fuse_id
                } else if !sealed {
                    seal_id
                } else {
                    rang_id
                }
            } else {
                let ql = &query_legal;
                sample(&logits, &|i: u32| {
                    if !ql.contains(&i) {
                        return false;
                    }
                    if i == open_id {
                        return qlen == 0 && word_initial.contains(&i);
                    }
                    if qlen == 0 {
                        return word_initial.contains(&i);
                    }
                    if i == fuse_id && depth == 0 {
                        return false;
                    }
                    if i == seal_id && depth > 0 {
                        return false;
                    }
                    true
                })
            };
            if id == split_id {
                depth += 1;
            }
            if id == fuse_id {
                depth = depth.saturating_sub(1);
            }
            if id == seal_id {
                sealed = true;
            }
            ids.push(id);
            if id == rang_id {
                break;
            }
        }
        let word = decode(voc, &ids[2..ids.len() - 1]);

        // ── skeleton + μ arm ──────────────────────────────────────────────
        for g in ["◇", "⊞", "⟨", "⟩", "●", "⟨"] {
            ids.push(id_of(voc, g));
        }
        let mut alen = 0usize;
        loop {
            let logits = self.forward_last(&ids);
            let al = &answer_legal;
            let id = if alen + 1 >= 16 {
                rang_id
            } else {
                sample(&logits, &|i: u32| al.contains(&i))
            };
            ids.push(id);
            alen += 1;
            if id == rang_id {
                break;
            }
        }
        ids.push(seal_id);
        let turn = decode(voc, &ids);

        // ── the gate, on-board ────────────────────────────────────────────
        let has_tri = word.chars().any(|c| matches!(c, '∈' | '∋' | '~' | '≁'));
        let verdict = if has_tri {
            let steps = imasm16_3::parse_glyph_word(&word);
            let (v, _) = imasm16_3::tri_ancestral_verdict(&steps);
            v
        } else {
            let toks: Vec<CTok> =
                word.chars().filter_map(|c| CTok::parse(&c.to_string())).collect();
            check::word_verdict(&toks).0
        };
        format!(
            "{turn}  gate {verdict}  → {}",
            if verdict == 'F' { "REFUSED" } else { "ADMITTED" }
        )
    }
}
