// Draw a braid word.
//
// `qc` and `jp` print braid words as runs of signed integers. That is the exact
// object and it is unreadable past a dozen crossings — a 385-generator word is
// a wall of digits in which nothing about the braid is visible. The word is a
// picture; this renders it as one, in two forms.
//
// The terminal form is a strand diagram, three rows per crossing, with the
// under-strand broken at the crossing so over and under are distinguishable
// rather than merely counted. The SVG form is the same diagram as a file: the
// permutation is tracked so each physical strand keeps its colour from top to
// bottom, which is what makes a long word legible at a glance.
//
// Both forms carry the crossing index in the gutter, because the reason to look
// at a 385-crossing braid is almost always to find one specific crossing in it.

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use libm::{sin, cos, ceil, fabs};
use core::fmt::Write as _;

/// Strands a word needs: sigma_k acts on k and k+1, so the largest generator
/// fixes the count. A word with no generators is still a braid — on the two
/// strands that would be its narrowest home.
pub fn strands_for(word: &[i32]) -> usize {
    let hi = word.iter().map(|g| g.unsigned_abs() as usize).max().unwrap_or(1);
    (hi + 1).max(2)
}

/// The permutation a braid word induces, as `perm[position] = original strand`.
/// Reading it top to bottom is reading which strand ends up where.
pub fn permutation(word: &[i32], strands: usize) -> Vec<usize> {
    let mut p: Vec<usize> = (0..strands).collect();
    for &g in word {
        let i = (g.unsigned_abs() as usize) - 1;
        if i + 1 < strands { p.swap(i, i + 1); }
    }
    p
}

/// A braid is pure when every strand returns to its own position.
pub fn is_pure(word: &[i32], strands: usize) -> bool {
    permutation(word, strands).iter().enumerate().all(|(i, &s)| i == s)
}

pub fn writhe(word: &[i32]) -> i32 { word.iter().map(|g| g.signum()).sum() }

/// Clamp a requested window to the word. Returns the half-open range actually
/// drawn, so the caller can say what it left out instead of quietly truncating.
pub fn window(len: usize, start: usize, count: usize) -> (usize, usize) {
    let s = start.min(len);
    let e = if count == 0 { len } else { (s + count).min(len) };
    (s, e)
}

// ── terminal form ────────────────────────────────────────────────

/// Strand diagram, three text rows per crossing.
///
/// Strand k sits in column 2k. A crossing between k and k+1 replaces those two
/// columns for its three rows: the strands part, cross, and rejoin. The
/// over-strand's diagonal runs unbroken through the middle row; the under one
/// is absent there. That break is the whole content of the sign — without it a
/// crossing and its inverse draw identically.
pub fn ascii(word: &[i32], strands: usize, start: usize, end: usize) -> String {
    let w = if strands == 0 { 1 } else { 2 * strands - 1 };
    let mut out = String::new();

    out.push_str("      ");
    for k in 0..strands {
        out.push_str(&format!("{}", (k + 1) % 10));
        if k + 1 < strands { out.push(' '); }
    }
    out.push('\n');

    for idx in start..end {
        let g = word[idx];
        let i = (g.unsigned_abs() as usize) - 1;
        let (a, m, b) = (2 * i, 2 * i + 1, 2 * i + 2);

        for row in 0..3usize {
            let mut cells: Vec<char> = (0..w)
                .map(|c| if c % 2 == 0 { '│' } else { ' ' })
                .collect();
            if b < w {
                match row {
                    0 => { cells[a] = '╲'; cells[b] = '╱'; }
                    1 => {
                        cells[a] = ' ';
                        cells[b] = ' ';
                        // The strand that stays whole through the middle is the
                        // one on top: '╲' runs left-over-right, '╱' right-over-left.
                        cells[m] = if g > 0 { '╲' } else { '╱' };
                    }
                    _ => { cells[a] = '╱'; cells[b] = '╲'; }
                }
            }
            let body: String = cells.into_iter().collect();
            if row == 1 {
                out.push_str(&format!("{:>5} {}  σ{}{}\n",
                    idx, body, i + 1, if g > 0 { "" } else { "⁻¹" }));
            } else {
                out.push_str(&format!("      {}\n", body));
            }
        }
    }
    out
}

// ── SVG form ─────────────────────────────────────────────────────

const PALETTE: [&str; 8] = [
    "#c0392b", "#2471a3", "#1e8449", "#b7950b",
    "#7d3c98", "#117a65", "#ba4a00", "#34495e",
];

/// The same eight, lifted for a dark background. A palette chosen to sit on
/// white goes muddy on black, and the closed form carries its own stylesheet, so
/// it can answer either theme with the colours meant for it.
const PALETTE_DARK: [&str; 8] = [
    "#ff7a6b", "#6aaef0", "#58c98a", "#e3c05a",
    "#c08fdb", "#4fc7ae", "#ff9d5c", "#a3b1c2",
];

const DX: i32 = 44;   // strand spacing
const DY: i32 = 30;   // one crossing
const MARGIN: i32 = 34;
const LW: i32 = 3;    // strand width
const HALO: i32 = 9;  // over-strand halo: what cuts the gap in the under strand

fn xof(col: i32, k: usize) -> i32 { col + DX * k as i32 }

/// Horizontal advance from one folded column to the next: the strands, the
/// crossing-index gutter, and air between.
fn col_advance(strands: usize) -> i32 {
    DX * (strands.saturating_sub(1)) as i32 + 74
}

/// The braid as a standalone SVG document.
///
/// Over/under is drawn the way it is drawn on paper: the under strand is laid
/// down whole, then the over strand is stroked twice — once wide in the
/// background colour, once at width in its own. The wide pass erases the under
/// strand exactly where it passes beneath, so the gap follows the curve without
/// anyone having to split it.
///
/// Colour follows the strand, not the position: the permutation is carried
/// along so the line leaving the top at position 3 is the same colour wherever
/// it arrives at the bottom.
///
/// `fold` wraps the braid into columns of that many crossings, read left to
/// right. A compiled circuit runs to hundreds of generators, and one column of
/// those is a page thirty thousand pixels tall — an image in the sense that a
/// filing cabinet is a book. Folded, the same word is a figure. The strand
/// entering a column at position k is the one that left the previous column
/// there, and the labels at each column's head and foot say which. `fold` of 0
/// draws the single column.
pub fn svg(word: &[i32], strands: usize, start: usize, end: usize, fold: usize) -> String {
    let rows = end.saturating_sub(start);
    let per = if fold == 0 { rows.max(1) } else { fold };
    let cols = if rows == 0 { 1 } else { (rows + per - 1) / per };
    let adv = col_advance(strands);
    let width = MARGIN * 2 + adv * cols as i32 - (adv - DX * (strands.saturating_sub(1)) as i32) + 46;
    let height = MARGIN * 2 + DY * (per.min(rows.max(1)) as i32 + 2);
    let bg = "#ffffff";

    // Which strand sits in each position at the top of the drawn window. The
    // window may begin mid-word, so replay the prefix rather than assume identity.
    let mut perm: Vec<usize> = permutation(&word[..start], strands);

    let mut s = String::new();
    s.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" \
viewBox=\"0 0 {} {}\">\n",
        width, height, width, height));
    s.push_str(&format!(
        "<title>braid word, {} generators on {} strands</title>\n",
        word.len(), strands));
    s.push_str(&format!(
        "<desc>writhe {}, {} braid, crossings {} to {}{}</desc>\n",
        writhe(word),
        if is_pure(word, strands) { "pure" } else { "non-pure" },
        start, end,
        if cols > 1 { format!(", folded into {} columns of {}", cols, per) }
        else { String::new() }));
    s.push_str(&format!("<rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n", bg));
    s.push_str("<g stroke-linecap=\"round\" fill=\"none\">\n");

    for c in 0..cols {
        let x0 = MARGIN + adv * c as i32;
        let lo = start + c * per;
        let hi = (lo + per).min(end);
        let mut y = MARGIN;

        // Lead-in, labelled with the strand each line arrives as.
        for k in 0..strands {
            s.push_str(&format!(
                "<path d=\"M {} {} L {} {}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
                xof(x0, k), y, xof(x0, k), y + DY, PALETTE[perm[k] % 8], LW));
            s.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"11\" \
text-anchor=\"middle\" fill=\"#555\" stroke=\"none\">{}</text>\n",
                xof(x0, k), y - 8, perm[k] + 1));
        }
        y += DY;

        for idx in lo..hi {
            let g = word[idx];
            let i = (g.unsigned_abs() as usize) - 1;
            let (xa, xb) = (xof(x0, i), xof(x0, i + 1));
            let (y0, y1) = (y, y + DY);

            for k in 0..strands {
                if k == i || k == i + 1 { continue; }
                s.push_str(&format!(
                    "<path d=\"M {} {} L {} {}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
                    xof(x0, k), y0, xof(x0, k), y1, PALETTE[perm[k] % 8], LW));
            }

            // The two legs of the crossing, as S-curves.
            let lr = format!("M {} {} C {} {} {} {} {} {}",
                             xa, y0, xa, y0 + DY / 2, xb, y1 - DY / 2, xb, y1);
            let rl = format!("M {} {} C {} {} {} {} {} {}",
                             xb, y0, xb, y0 + DY / 2, xa, y1 - DY / 2, xa, y1);
            let (under, over, cu, co) = if g > 0 {
                (rl, lr, PALETTE[perm[i + 1] % 8], PALETTE[perm[i] % 8])
            } else {
                (lr, rl, PALETTE[perm[i] % 8], PALETTE[perm[i + 1] % 8])
            };
            s.push_str(&format!("<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
                                under, cu, LW));
            // Butt caps on the halo. With the round caps the group sets, the halo
            // overhangs its own endpoints by half its width and erases the tip of
            // the segment feeding into the crossing — every strand arrived at
            // every crossing with a nick out of it.
            s.push_str(&format!(
                "<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"butt\"/>\n",
                over, bg, HALO));
            s.push_str(&format!("<path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
                                over, co, LW));

            s.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"9\" \
fill=\"#999\" stroke=\"none\">{}{}</text>\n",
                xof(x0, strands.saturating_sub(1)) + 12, y0 + DY / 2 + 3,
                idx, if g > 0 { "" } else { "\u{2212}" }));

            perm.swap(i, i + 1);
            y = y1;
        }

        for k in 0..strands {
            s.push_str(&format!(
                "<path d=\"M {} {} L {} {}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
                xof(x0, k), y, xof(x0, k), y + DY, PALETTE[perm[k] % 8], LW));
            s.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"monospace\" font-size=\"11\" \
text-anchor=\"middle\" fill=\"#555\" stroke=\"none\">{}</text>\n",
                xof(x0, k), y + DY + 14, perm[k] + 1));
        }
    }

    s.push_str("</g>\n</svg>\n");
    s
}

/// Word summary, printed above either form.
pub fn header(word: &[i32], strands: usize, start: usize, end: usize) -> String {
    let mut s = format!(
        "  {} generators on {} strands, writhe {}, {} braid\n",
        word.len(), strands, writhe(word),
        if is_pure(word, strands) { "pure" } else { "non-pure" });
    let p = permutation(word, strands);
    s.push_str("  permutation:");
    for &v in &p { s.push_str(&format!(" {}", v + 1)); }
    s.push('\n');
    if start > 0 || end < word.len() {
        s.push_str(&format!("  drawing crossings {}..{} of {}\n", start, end, word.len()));
    }
    s
}

// ── closed form ──────────────────────────────────────────────────

const TAU: f64 = 6.283185307179586;

/// A point on the ring, at angle `a` and radius `r`.
fn polar(cx: f64, cy: f64, a: f64, r: f64) -> (f64, f64) {
    (cx + r * cos(a), cy + r * sin(a))
}

/// Where a run starts, appended to the path being built.
fn move_to(out: &mut String, cx: f64, cy: f64, a: f64, r: f64) {
    let (x, y) = polar(cx, cy, a, r);
    let _ = write!(out, "M {:.2} {:.2}", x, y);
}

/// One run along the ring, from angle `a0` to `a1` and radius `ra` to `rb`, as
/// cubics fitted to the curve's own tangents. Appended without a move-to, so
/// consecutive runs concatenate into one path.
///
/// The radius follows a smoothstep in the angle. Its derivative vanishes at both
/// ends, so the run leaves its track along the track and rejoins the next one the
/// same way, and the seam between a crossing and the arcs either side of it is
/// not visible. `ra == rb` is the arc case and comes out as the circle it is.
///
/// One cubic tracks about a quarter turn before the error shows, so a wider span
/// is cut into that many pieces, each fitted from the exact position and
/// derivative at its own ends rather than from a guess at a control point.
///
/// It writes into the caller's buffer rather than returning a string of its own.
/// A ring of any size is tens of thousands of these, and on an arena that
/// reclaims in LIFO order a returned string that has to grow leaves every
/// intermediate copy behind it.
fn run_into(out: &mut String, cx: f64, cy: f64, a0: f64, a1: f64, ra: f64, rb: f64) {
    let da = a1 - a0;
    let segs = (ceil(fabs(da) / 0.45) as usize).max(1);
    let d_r = rb - ra;
    let at = |t: f64| a0 + da * t;
    let rt = |t: f64| ra + d_r * (3.0 * t * t - 2.0 * t * t * t);
    let rp = |t: f64| d_r * 6.0 * t * (1.0 - t);
    let pt = |t: f64| polar(cx, cy, at(t), rt(t));
    // d/dt of (r cos a, r sin a) with both r and a moving.
    let dp = |t: f64| {
        let (a, r, k) = (at(t), rt(t), rp(t));
        (k * cos(a) - r * da * sin(a), k * sin(a) + r * da * cos(a))
    };

    let h = 1.0 / segs as f64;
    for k in 0..segs {
        let (t0, t1) = (k as f64 * h, (k + 1) as f64 * h);
        let (px0, py0) = pt(t0);
        let (px1, py1) = pt(t1);
        let (dx0, dy0) = dp(t0);
        let (dx1, dy1) = dp(t1);
        let _ = write!(out, " C {:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
            px0 + dx0 * h / 3.0, py0 + dy0 * h / 3.0,
            px1 - dx1 * h / 3.0, py1 - dy1 * h / 3.0,
            px1, py1);
    }
}

/// One leg of a crossing: parallel into the slot, across it, parallel out.
fn leg_into(out: &mut String, cx: f64, cy: f64,
            a0: f64, b0: f64, b1: f64, a1: f64, ra: f64, rb: f64) {
    move_to(out, cx, cy, a0, ra);
    run_into(out, cx, cy, a0, b0, ra, ra);
    run_into(out, cx, cy, b0, b1, ra, rb);
    run_into(out, cx, cy, b1, a1, rb, rb);
}

/// The closed braid, drawn as the loop it is.
///
/// A braid word is strands running down a page, and closing it is bending the
/// page into a cylinder so that every strand's foot meets its own head. This
/// draws that cylinder end on. The strand positions are concentric tracks, one
/// crossing owns one angular slot, and going once around the ring is reading the
/// word once through. Nothing has to be added to close the picture, because a
/// circle is already closed: the strands of a permutation cycle join into a
/// single curve on their own, and each such curve is one component of the link.
/// The braid axis is the hole.
///
/// Colour follows the component rather than the strand, which is the one place
/// this parts company with the flat form. A component that changed colour where
/// two of its strands meet would be one closed curve drawn as several.
///
/// Everything is arcs and cubics fitted to their own tangents, so a strand runs
/// through a crossing without a corner at either end of it. Radial spacing is
/// fixed and the radius follows the crossing count, so a longer word makes a
/// wider ring rather than a denser one, and the crossings stay the same size to
/// read.
pub fn svg_loop(word: &[i32], strands: usize) -> String {
    let strands = strands.max(2);
    let m_all = word.len();

    // The cycles of the permutation are the closed curves, so they are what
    // carries colour: every strand a component passes through gets its colour.
    let p = permutation(word, strands);
    let mut comp = vec![0usize; strands];
    let mut seen = vec![false; strands];
    let mut ncomp = 0usize;
    for s0 in 0..strands {
        if seen[s0] { continue; }
        let mut c = s0;
        while !seen[c] { seen[c] = true; comp[c] = ncomp; c = p[c]; }
        ncomp += 1;
    }

    // A ring costs its crossings, and the arena has to hold the whole document
    // before a byte of it reaches the serial line. Draw what fits and say so,
    // rather than emitting most of a picture and dying in the middle of it.
    let per_crossing = 600 + 180 * strands.saturating_sub(2);
    let (used, total) = crate::heap_used();
    let budget = (total.saturating_sub(used) / 3) / per_crossing.max(1);
    let m = m_all.min(budget.max(16));

    // Geometry. Crossings keep their size and the ring grows to fit them, down
    // to a floor where a short word would otherwise draw a ring smaller than its
    // own strand band.
    let dr = 26.0f64;
    let band = dr * (strands - 1) as f64;
    let step = if m <= 24 { 44.0 } else if m >= 400 { 17.0 }
               else { 44.0 - (m - 24) as f64 * 27.0 / 376.0 };
    let r_mid = ((m.max(1) as f64) * step / TAU).max(band * 1.35 + 78.0);
    let r_out = r_mid + band / 2.0;
    let r_in = r_mid - band / 2.0;
    let margin = 52.0;
    let size = 2.0 * (r_out + margin);
    let (cx, cy) = (size / 2.0, size / 2.0);
    let rk = |k: usize| r_out - dr * k as f64;
    let da = TAU / (m.max(1) as f64);
    // How much of a slot the crossing itself takes; the rest is the parallel run
    // into and out of it. A slot a third of the ring wide reads better with more
    // of it given to the run, or the crossing swings so far that the ring looks
    // like two circles overlapping rather than a braid closed on itself.
    let pad_frac = 0.21 + 0.13 * (((12.0 - m as f64) / 9.0).clamp(0.0, 1.0));
    let pad = da * pad_frac;
    // Angles run clockwise from twelve o'clock, which is where the word starts.
    let a_of = |j: usize| -TAU / 4.0 + da * (j as f64);
    let lw = 3.2f64;
    let halo = 10.0f64;
    let colour = |strand: usize| comp[strand] % 8;

    // Index every crossing while they are far enough apart to read, then thin
    // out rather than overprint.
    let label_every = if m <= 64 { 1 } else if m <= 260 { 5 }
                      else if m <= 1200 { 25 } else { 0 };
    // Indices and the start mark scale with the figure, or a wide ring carries
    // annotation sized for a small one and reads as unlabelled. The ceiling is
    // the gap between two labelled slots, so they cannot grow into each other.
    let idx_gap = step * (label_every.max(1) as f64) * 0.9;
    let idx_fs = (size / 78.0).max(9.0).min(idx_gap.max(9.0));

    // One allocation of the size the document will be, written into in place.
    // Formatting each element into a string of its own and pushing that costs
    // an allocation per element and leaves its growth behind on an arena that
    // reclaims in LIFO order, which is how a drawing well inside its budget ran
    // the kernel out of heap.
    let mut s = String::with_capacity(per_crossing * m + 4096);
    let _ = writeln!(s,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.0}\" height=\"{:.0}\" \
viewBox=\"0 0 {:.0} {:.0}\">", size, size, size, size);
    let _ = writeln!(s,
        "<title>closed braid, {} generators on {} strands</title>", m_all, strands);
    let _ = write!(s, "<desc>writhe {}, {} component{}, {} braid",
        writhe(word), ncomp, if ncomp == 1 { "" } else { "s" },
        if is_pure(word, strands) { "pure" } else { "non-pure" });
    if m < m_all {
        let _ = write!(s, ", closure of crossings 0 to {} of {}", m, m_all);
    }
    s.push_str("</desc>\n");

    // The gap in an under strand is cut by overstroking it in the background
    // colour, so the background has to be a colour and not the absence of one.
    // A viewer in a dark theme would otherwise see the gaps filled with white.
    // Naming both in a stylesheet lets the one file answer either theme, and
    // costs less per path than spelling the colour out on every halo.
    s.push_str("<style>\n.bg{fill:#ffffff}.halo{stroke:#ffffff}\
.cap{fill:#5f6368}.idx{fill:#9aa0a6}");
    for (n, c) in PALETTE.iter().enumerate() {
        let _ = write!(s, ".c{}{{stroke:{}}}", n, c);
    }
    s.push_str("\n@media(prefers-color-scheme:dark){.bg{fill:#101215}\
.halo{stroke:#101215}.cap{fill:#9aa0a6}.idx{fill:#6b7075}");
    for (n, c) in PALETTE_DARK.iter().enumerate() {
        let _ = write!(s, ".c{}{{stroke:{}}}", n, c);
    }
    s.push_str("}\n</style>\n");
    s.push_str("<rect class=\"bg\" width=\"100%\" height=\"100%\"/>\n");
    let _ = writeln!(s,
        "<g fill=\"none\" stroke-linecap=\"round\" stroke-linejoin=\"round\" \
stroke-width=\"{}\">", lw);

    // The empty word closes to as many unknots as it has strands, and that is
    // the picture: bare tracks, no crossings.
    if m == 0 {
        for k in 0..strands {
            let _ = writeln!(s,
                "<circle class=\"c{}\" cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\"/>",
                colour(k), cx, cy, rk(k));
        }
    }

    // Three buffers reused for every crossing. Cleared rather than dropped, so
    // after the first slot they never allocate again.
    let mut d_under = String::new();
    let mut d_over = String::new();
    let mut d_halo = String::new();

    let mut occ: Vec<usize> = (0..strands).collect();   // occ[position] = strand
    for j in 0..m {
        let (a0, a1) = (a_of(j), a_of(j + 1));
        let g = word[j];
        let i = (g.unsigned_abs() as usize) - 1;

        // A generator wider than the strand count acts on nothing here. Run
        // every track straight through the slot rather than drop the slot.
        let crossing = i + 1 < strands;

        for k in 0..strands {
            if crossing && (k == i || k == i + 1) { continue; }
            let _ = write!(s, "<path class=\"c{}\" d=\"", colour(occ[k]));
            move_to(&mut s, cx, cy, a0, rk(k));
            run_into(&mut s, cx, cy, a0, a1, rk(k), rk(k));
            s.push_str("\"/>\n");
        }
        if !crossing { continue; }

        // The two legs run parallel into the slot, cross in the middle of it,
        // and leave parallel. Swinging across the whole slot instead draws a
        // ring of waves, in which a crossing is not a crossing to look at.
        let (b0, b1) = (a0 + pad, a1 - pad);
        let (ro, ri) = (rk(i), rk(i + 1));   // position i is the outer track
        // Sign says which way the outer track's leg passes. The halo belongs to
        // whichever leg goes over, and covers a little of the run either side of
        // the crossing, so the gap it cuts is centred on the crossing rather
        // than ending in it.
        let (r_under, r_over) = if g > 0 { ((ri, ro), (ro, ri)) } else { ((ro, ri), (ri, ro)) };
        let (cu, co) = if g > 0 {
            (colour(occ[i + 1]), colour(occ[i]))
        } else {
            (colour(occ[i]), colour(occ[i + 1]))
        };

        d_under.clear();
        leg_into(&mut d_under, cx, cy, a0, b0, b1, a1, r_under.0, r_under.1);
        d_over.clear();
        leg_into(&mut d_over, cx, cy, a0, b0, b1, a1, r_over.0, r_over.1);
        d_halo.clear();
        let (h0, h1) = (b0 - pad * 0.5, b1 + pad * 0.5);
        leg_into(&mut d_halo, cx, cy, h0, b0, b1, h1, r_over.0, r_over.1);

        let _ = writeln!(s, "<path class=\"c{}\" d=\"{}\"/>", cu, d_under);
        // Butt caps on the halo, or it overhangs its own ends by half its width
        // and takes a nick out of the run feeding into the slot.
        let _ = writeln!(s,
            "<path class=\"halo\" d=\"{}\" stroke-width=\"{}\" stroke-linecap=\"butt\"/>",
            d_halo, halo);
        let _ = writeln!(s, "<path class=\"c{}\" d=\"{}\"/>", co, d_over);

        if label_every > 0 && j % label_every == 0 {
            let (lx, ly) = polar(cx, cy, (a0 + a1) / 2.0, r_out + idx_fs * 1.1);
            let _ = writeln!(s,
                "<text class=\"idx\" x=\"{:.2}\" y=\"{:.2}\" font-family=\"monospace\" \
font-size=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
stroke=\"none\">{}{}</text>",
                lx, ly, idx_fs, j, if g > 0 { "" } else { "\u{2212}" });
        }

        occ.swap(i, i + 1);
    }

    // Where the reading starts and which way it goes, as a mark outside the ring
    // at twelve o'clock rather than a caption pointing at one.
    let (mx, my) = polar(cx, cy, -TAU / 4.0, r_out + idx_fs * 2.6);
    let mk = idx_fs * 0.62;
    let _ = writeln!(s,
        "<path class=\"idx\" d=\"M {:.2} {:.2} l {:.2} {:.2} l 0 {:.2} z\" \
stroke=\"none\"/>", mx + mk, my, -mk, -mk * 0.85, mk * 1.7);

    // The hole is the one place a caption can go without crossing anything.
    if r_in >= 66.0 {
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("{} generators", m_all));
        lines.push(format!("{} strands, {} component{}",
                           strands, ncomp, if ncomp == 1 { "" } else { "s" }));
        lines.push(format!("writhe {}", writhe(word)));
        if m < m_all { lines.push(format!("drawn: crossings 0 to {}", m)); }
        let widest = lines.iter().map(|l| l.len()).max().unwrap_or(1) as f64;
        // The caption scales with the hole it sits in, so a wide ring is not
        // labelled in the same nine points as a small one.
        // Sized so the caption spans about a third of the figure whatever the
        // figure is, and never more than the hole can hold.
        let fs = ((0.30 * size) / (widest * 0.62)).min(r_in / 4.6).max(9.0);
        let y0 = cy - (lines.len() as f64 - 1.0) * fs * 0.72;
        for (n, l) in lines.iter().enumerate() {
            let _ = writeln!(s,
                "<text class=\"cap\" x=\"{:.2}\" y=\"{:.2}\" font-family=\"monospace\" \
font-size=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
stroke=\"none\">{}</text>",
                cx, y0 + n as f64 * fs * 1.45, fs, l);
        }
    }

    s.push_str("</g>\n</svg>\n");
    s
}
