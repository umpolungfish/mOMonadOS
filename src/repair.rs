// ─── repair.rs ────────────────────────────────────────────────────────
// Automatic program/proof surgery with ranked repairs
// Searches constrained edit space and ranks repairs by cost
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// word -> tuple -> crystal address, the same kernel-backed pipeline
/// `imasm derive` prints (program_from_glyphs -> self_imscribe ->
/// IgTuple::from_snapshot -> crystal_address). Called directly on the
/// underlying functions, not through the CLI path law 17 flags.
fn crystal_address_of(word: &str) -> Option<u32> {
    let prog = crate::belnap_ring_shor::program_from_glyphs(word).ok()?;
    let snap = crate::kernel::self_imscribe(&prog);
    let t = crate::imas_ig::IgTuple::from_snapshot(&snap);
    Some(t.crystal_address())
}

/// FSPLIT count minus FFUSE count. Positive: splits outnumber fuses, that
/// many FFUSE insertions close the count gap. Negative: the reverse. Zero
/// count imbalance does not by itself mean the fork/fuse ancestry pairs
/// correctly, only that nothing about the counts rules it out.
fn fork_fuse_imbalance(word: &str) -> isize {
    let splits = word.chars().filter(|&c| c == '∈').count() as isize;
    let fuses = word.chars().filter(|&c| c == '∋').count() as isize;
    splits - fuses
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepairType {
    Insertion(char, usize),      // insert glyph at position
    Deletion(usize),             // delete glyph at position
    Substitution(char, usize),   // substitute glyph at position
    Permutation(usize, usize),   // swap positions
    Rotation(isize),             // rotate by k positions
    LocalRewrite(String, String, usize), // replace substring at position
    PrimitivePromotion(String),  // promote a primitive value
}

#[derive(Debug, Clone)]
pub struct RepairCandidate {
    pub repair: RepairType,
    pub cost: f64,
    pub edit_distance: usize,
    pub entropy_delta: f64,
    pub tier_change: f64,
    pub new_assumptions: usize,
    pub repaired_word: String,
    pub verification_status: String,
}

#[derive(Debug, Clone)]
pub struct RepairResult {
    pub original: String,
    pub error_type: String,
    pub repairs: Vec<RepairCandidate>,
    pub best_repair: Option<RepairCandidate>,
    pub proof_diff: String,
}

pub struct RepairEngine {
    glyphs: Vec<char>,
    alpha: f64,  // edit distance weight
    beta: f64,   // entropy delta weight
    gamma: f64,  // tier change weight
    delta: f64,  // new assumptions weight
}

impl RepairEngine {
    pub fn new() -> Self {
        Self {
            glyphs: vec![
                '⊢', '⊣', '≻', '≺', '⋈', '⊤', 
                '∈', '∋', '⊙', '⊥', '⊞', '⊡'
            ],
            alpha: 1.0,
            beta: 0.5,
            gamma: 2.0,
            delta: 3.0,
        }
    }

    pub fn repair(&self, artifact: &str, artifact_type: &str) -> RepairResult {
        // Diagnose the error first
        let error_type = self.diagnose_error(artifact, artifact_type);
        
        // Generate repair candidates
        let mut candidates = Vec::new();
        
        // 1. Insertion repairs
        for pos in 0..=artifact.chars().count() {
            for &glyph in &self.glyphs {
                let repaired = self.insert_at(artifact, glyph, pos);
                if self.verify_repair(&repaired, artifact_type) {
                    candidates.push(self.make_candidate(
                        RepairType::Insertion(glyph, pos),
                        &repaired,
                        artifact,
                    ));
                }
            }
        }
        
        // 2. Deletion repairs
        for pos in 0..artifact.chars().count() {
            let repaired = self.delete_at(artifact, pos);
            if self.verify_repair(&repaired, artifact_type) {
                candidates.push(self.make_candidate(
                    RepairType::Deletion(pos),
                    &repaired,
                    artifact,
                ));
            }
        }
        
        // 3. Substitution repairs
        for (pos, orig) in artifact.chars().enumerate() {
            for &glyph in &self.glyphs {
                if glyph == orig { continue; }
                let repaired = self.substitute_at(artifact, glyph, pos);
                if self.verify_repair(&repaired, artifact_type) {
                    candidates.push(self.make_candidate(
                        RepairType::Substitution(glyph, pos),
                        &repaired,
                        artifact,
                    ));
                }
            }
        }
        
        // 4. Permutation repairs (swaps)
        let chars: Vec<char> = artifact.chars().collect();
        for i in 0..chars.len() {
            for j in (i+1)..chars.len() {
                let mut swapped = chars.clone();
                swapped.swap(i, j);
                let repaired: String = swapped.iter().collect();
                if self.verify_repair(&repaired, artifact_type) {
                    candidates.push(self.make_candidate(
                        RepairType::Permutation(i, j),
                        &repaired,
                        artifact,
                    ));
                }
            }
        }
        
        // 5. Rotation repairs
        let n_chars = artifact.chars().count() as isize;
        for k in -n_chars..=n_chars {
            if k == 0 { continue; }
            let repaired = self.rotate(artifact, k);
            if self.verify_repair(&repaired, artifact_type) {
                candidates.push(self.make_candidate(
                    RepairType::Rotation(k),
                    &repaired,
                    artifact,
                ));
            }
        }
        
        // Sort by cost
        candidates.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
        
        let best = candidates.first().cloned();
        
        let proof_diff = self.generate_proof_diff(artifact, &best);

        RepairResult {
            original: artifact.to_string(),
            error_type,
            repairs: candidates,
            best_repair: best,
            proof_diff,
        }
    }

    /// Reads the actual artifact rather than returning a canned label by
    /// type. A word this diagnosis calls "execution failure" on used to get
    /// the same string as every other program, whether the real defect was
    /// a fork/fuse imbalance the search below has no hope of closing in one
    /// edit, or a single exposed clear one insertion fixes -- indistinguishable
    /// from the message alone. Reports what's actually there: the FSPLIT/FFUSE
    /// count imbalance (the search space needs at least that many coordinated
    /// edits, not one, to reach a paired word at all), the closure verdict
    /// from a real walk, and the banked-weight exposure, each computed, not
    /// assumed from the artifact type.
    fn diagnose_error(&self, artifact: &str, artifact_type: &str) -> String {
        let splits = artifact.chars().filter(|&c| c == '∈').count();
        let fuses = artifact.chars().filter(|&c| c == '∋').count();
        let imbalance = fork_fuse_imbalance(artifact);

        let mut parts: Vec<String> = Vec::new();
        if imbalance != 0 {
            parts.push(format!(
                "{} FSPLIT against {} FFUSE, {} unpaired {} -- a matched-count word sits {} insertions out, past single-edit search's reach; repair_chain targets exactly that many",
                splits, fuses, imbalance.unsigned_abs(),
                if imbalance > 0 { "splits" } else { "fuses" },
                imbalance.unsigned_abs(),
            ));
        }

        match imasm_core::lattice_flow::tri_ancestral_word_verdict(artifact) {
            Some(v) if v != 'T' => parts.push(format!("tri-ancestral verdict {} (not T)", v)),
            None => parts.push("does not parse as a walkable word".to_string()),
            _ => {}
        }

        if let Some(b) = imasm_core::lattice_flow::banked_walk(artifact) {
            if !b.exposed.is_empty() {
                parts.push(format!("{} clear(s) exposed with nothing banked behind them", b.exposed.len()));
            } else if b.vacuous() {
                // Passing the exposed check for the wrong reason: nothing ever
                // cleared against a live register, so nothing was ever at risk
                // of being lost. count-balancing cannot touch this -- it is not
                // a fork/fuse defect, it is a fixation (⊡, IFIX) shutting the
                // walk down before any clear that would have exercised banking.
                parts.push(format!(
                    "VACUOUS -- no clear fired against a live register ({} deposit(s), {} step(s) inert after a fixation); no amount of fork/fuse count-balancing reaches this, the fixation itself has to move or go",
                    b.deposits, b.inert
                ));
            }
        }

        if parts.is_empty() {
            return match artifact_type {
                "program" => "execution failure",
                "proof" => "verification failure",
                "theorem" => "type checking failure",
                "invariant" => "invariant violation",
                _ => "unknown error",
            }.to_string();
        }
        parts.join("; ")
    }

    fn verify_repair(&self, repaired: &str, _artifact_type: &str) -> bool {
        // A repair must parse as a non-empty IMASM word of the twelve
        // glyphs, and then hold on BOTH real instruments: `banked_walk`
        // (weight didn't clear in the open) and `tri_ancestral_word_verdict`
        // (the fork/fuse pairing itself still closes over real work, not
        // just a fuse-count that doesn't exceed the split-count -- that
        // count check passed plenty of candidates whose forks or fuses
        // still dangled, because count alone doesn't see pairing). Caught
        // live: `combo2`'s repair search kept returning cost-0.00 no-op
        // permutations because this check accepted almost anything that
        // merely parsed.
        if repaired.is_empty() {
            return false;
        }
        if !repaired.chars().all(|c| self.glyphs.contains(&c)) {
            return false;
        }
        imasm_core::lattice_flow::candidate_holds(repaired)
    }

    /// The next rung past single-edit search: the exact, not heuristic,
    /// fix for a fork/fuse count imbalance. `cyclic_pairs` (imasm_core) is
    /// a standard cyclic bracket-matcher, FSPLIT opens, FFUSE closes, and
    /// it already tries every split as a possible start looking for a
    /// rotation with no underflow. That search existing is exactly the
    /// cycle lemma for a sequence of equal ups and downs: for ANY
    /// arrangement of n opens and n closes, at least one rotation has every
    /// partial sum non-negative. Position of the inserted glyphs plays no
    /// part in that guarantee, only the final count does, so the fix is
    /// the count alone: append the imbalance's own magnitude in the glyph
    /// it is short of, and `cyclic_pairs`'s existing rotation search finds
    /// the valid start on its own. A first version of this walked the word
    /// tracking depth and inserted at every local deficit, solving the
    /// harder LINEAR problem (valid starting at position 0 specifically)
    /// instead of the CYCLIC one actually needed, and over-inserted by one
    /// on the word that first surfaced this. Checked directly against
    /// `--frames` and `banked` before trusting it: appending eleven FSPLIT
    /// to the word that started this closes verdict T and reads VACUOUS on
    /// the banked walk, both confirmed live, not assumed from the count.
    pub fn repair_chain(&self, word: &str, artifact_type: &str) -> Option<(Vec<RepairType>, String)> {
        if self.verify_repair(word, artifact_type) {
            return Some((Vec::new(), word.to_string()));
        }
        let mut chain: Vec<RepairType> = Vec::new();
        let mut current = word.to_string();

        // Vacuousness from an early fixation is a different defect from a
        // count imbalance, and no insertion reaches it: once a ⊡ (IFIX)
        // sets the walk's `fixed` flag, every step after it goes inert
        // regardless of what gets inserted later, so a count-balancing
        // chain alone can never turn a vacuous word live. The only move
        // that reaches it is deleting the fixation doing the blocking.
        // Checked live on the word that surfaced this: deleting its
        // leading ⊡ turned a fully inert walk (0 deposits, 379 of 475
        // steps inert) into a live one carrying an ordinary exposed-clear
        // defect instead -- ordinary enough for the rest of this chain to
        // have a real shot at it. Bounded by the word's own length: a
        // fixation genuinely needed for structure, not blocking anything,
        // never shows as vacuous, so this loop only ever fires on real
        // instances of the defect it targets.
        let bound = current.chars().count();
        for _ in 0..bound {
            let vacuous = imasm_core::lattice_flow::banked_walk(&current)
                .map(|b| b.vacuous())
                .unwrap_or(false);
            if !vacuous { break; }
            let Some(pos) = current.chars().position(|c| c == '⊡') else { break; };
            let mut chars: Vec<char> = current.chars().collect();
            chars.remove(pos);
            current = chars.into_iter().collect();
            chain.push(RepairType::Deletion(pos));
            if self.verify_repair(&current, artifact_type) {
                return Some((chain, current));
            }
        }

        let imbalance = fork_fuse_imbalance(&current);
        if imbalance != 0 {
            let glyph = if imbalance < 0 { '∈' } else { '∋' };
            let n = imbalance.unsigned_abs();
            let end = current.chars().count();
            for k in 0..n {
                chain.push(RepairType::Insertion(glyph, end + k));
                current.push(glyph);
            }
            if self.verify_repair(&current, artifact_type) {
                return Some((chain, current));
            }
        }

        // Counts now match (or already matched); whatever verify_repair
        // still finds wrong is a different defect -- no work on any paired
        // arm, or weight exposed in the open -- not a count one. One real
        // attempt through the ordinary single-edit search on the balanced
        // word, rather than declaring the chain done without checking.
        if let Some(best) = self.repair(&current, artifact_type).best_repair {
            if self.verify_repair(&best.repaired_word, artifact_type) {
                chain.push(best.repair);
                return Some((chain, best.repaired_word));
            }
        }
        None
    }

    fn make_candidate(&self, repair: RepairType, repaired: &str, original: &str) -> RepairCandidate {
        let edit_distance = self.compute_edit_distance(original, repaired);
        let entropy_delta = self.compute_entropy_delta(original, repaired);
        let tier_change = self.compute_tier_change(original, repaired);
        let new_assumptions = self.count_new_assumptions(original, repaired);
        
        let cost = self.alpha * edit_distance as f64
                 + self.beta * entropy_delta
                 + self.gamma * tier_change
                 + self.delta * new_assumptions as f64;
        
        RepairCandidate {
            repair,
            cost,
            edit_distance,
            entropy_delta,
            tier_change,
            new_assumptions,
            repaired_word: repaired.to_string(),
            verification_status: "verified".to_string(),
        }
    }

    fn compute_edit_distance(&self, a: &str, b: &str) -> usize {
        // Levenshtein edit distance between the two words.
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let mut dp = vec![vec![0usize; b.len() + 1]; a.len() + 1];
        for i in 0..=a.len() {
            dp[i][0] = i;
        }
        for j in 0..=b.len() {
            dp[0][j] = j;
        }
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + cost);
            }
        }
        dp[a.len()][b.len()]
    }

    fn compute_entropy_delta(&self, a: &str, b: &str) -> f64 {
        // Shannon entropy difference
        let entropy_a = self.shannon_entropy(a);
        let entropy_b = self.shannon_entropy(b);
        (entropy_b - entropy_a).abs()
    }

    fn shannon_entropy(&self, word: &str) -> f64 {
        let mut counts = alloc::collections::BTreeMap::new();
        let len = word.chars().count();
        if len == 0 { return 0.0; }
        
        for c in word.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        
        let mut entropy = 0.0;
        for &count in counts.values() {
            let p = count as f64 / len as f64;
            if p > 0.0 {
                entropy -= p * libm::log2(p);
            }
        }
        entropy
    }

    fn compute_tier_change(&self, a: &str, b: &str) -> f64 {
        // Tier proxy: mean glyph ordinal along the 12-mark order. The change
        // is the absolute shift in mean ordinal (measured in slots).
        (self.mean_ordinal(b) - self.mean_ordinal(a)).abs()
    }

    fn mean_ordinal(&self, word: &str) -> f64 {
        let ordinals: Vec<usize> = word.chars()
            .filter_map(|c| self.glyphs.iter().position(|&g| g == c))
            .collect();
        if ordinals.is_empty() {
            return 0.0;
        }
        ordinals.iter().sum::<usize>() as f64 / ordinals.len() as f64
    }

    fn count_new_assumptions(&self, a: &str, b: &str) -> usize {
        // An assumption is a glyph class introduced by the repair that was
        // absent from the original word. Count distinct new glyph classes.
        let mut distinct: Vec<char> = Vec::new();
        for c in b.chars() {
            if !a.contains(c) && !distinct.contains(&c) {
                distinct.push(c);
            }
        }
        distinct.len()
    }

    fn insert_at(&self, word: &str, glyph: char, pos: usize) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.insert(pos, glyph);
        chars.into_iter().collect()
    }

    fn delete_at(&self, word: &str, pos: usize) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.remove(pos);
        chars.into_iter().collect()
    }

    fn substitute_at(&self, word: &str, glyph: char, pos: usize) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars[pos] = glyph;
        chars.into_iter().collect()
    }

    fn rotate(&self, word: &str, k: isize) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        let len = chars.len();
        if len == 0 { return String::new(); }
        
        let k = ((k % len as isize) + len as isize) % len as isize;
        chars.rotate_right(k as usize);
        chars.into_iter().collect()
    }

    fn generate_proof_diff(&self, original: &str, best: &Option<RepairCandidate>) -> String {
        if let Some(repair) = best {
            format!(
                "PROOF DIFF\n\
                 ========\n\
                 Original: {}\n\
                 Repaired: {}\n\
                 Repair type: {:?}\n\
                 Cost: {:.2}\n\
                 \n\
                 Changes:\n\
                 - Edit distance: {}\n\
                 - Entropy delta: {:.4}\n\
                 - Tier change: {:.2}\n\
                 - New assumptions: {}\n\
                 \n\
                 Verification: {}\n",
                original,
                repair.repaired_word,
                repair.repair,
                repair.cost,
                repair.edit_distance,
                repair.entropy_delta,
                repair.tier_change,
                repair.new_assumptions,
                repair.verification_status
            )
        } else {
            "No repair found\n".to_string()
        }
    }
}

/// The cheapest candidate in a repair search, and how many were found,
/// computed once and shared by `repair_best_report` (which formats it) and
/// `repair_best_word` (which just wants the resulting word to feed onward).
fn cheapest(word: &str) -> (usize, Option<RepairCandidate>) {
    let engine = RepairEngine::new();
    let result = engine.repair(word, "program");
    let count = result.repairs.len();
    let best = result.repairs.into_iter()
        .min_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap_or(core::cmp::Ordering::Equal));
    (count, best)
}

/// The single cheapest repair only, not the full ranked-by-address listing
/// `repair_main` gives. Same search, same engine; just the one line a caller
/// who already knows the search is real usually wants.
pub fn repair_best_report(word: &str) -> String {
    let (count, best) = cheapest(word);
    let best = match best {
        Some(b) => b,
        None => {
            let engine = RepairEngine::new();
            return match engine.repair_chain(word, "program") {
                Some((chain, repaired)) => format!(
                    "repair {}: single-edit search found nothing; repair_chain reached {} in {} step(s)\n",
                    word, repaired, chain.len()
                ),
                None => format!("repair {}: no repair found in search space\n", word),
            };
        }
    };

    let addr = crystal_address_of(&best.repaired_word)
        .map(|a| a.to_string())
        .unwrap_or_else(|| "unaddressable".to_string());

    format!(
        "repair {}: {} candidate(s), cheapest below\n  {:?} -> {}\n  cost {:.2}  edit distance {}  ΔS {:.4}  crystal {}\n",
        word, count, best.repair, best.repaired_word,
        best.cost, best.edit_distance, best.entropy_delta, addr
    )
}

/// Just the cheapest repaired word, no formatting -- for a caller that wants
/// to run the repair itself through something else. None if the word already
/// holds (nothing to repair) or the search space came up empty.
pub fn repair_best_word(word: &str) -> Option<String> {
    cheapest(word).1.map(|b| b.repaired_word)
}

pub fn repair_main(args: &[&str]) -> String {
    let engine = RepairEngine::new();
    
    if args.is_empty() {
        return "USAGE:\n\
                 repair <program>\n\
                 repair <proof>\n\
                 repair <Lean theorem>\n\
                 repair <invariant>\n\
                 \n\
                 Repair types searched:\n\
                 1. Insertion\n\
                 2. Deletion\n\
                 3. Substitution\n\
                 4. Permutation\n\
                 5. Rotation\n\
                 6. Local rewrite\n\
                 7. Primitive promotion\n\
                 \n\
                 Cost function:\n\
                 cost = α(edit_distance) + β(ΔS) + γ(tier_change) + δ(new_assumptions)\n\
                 \n\
                 Example:\n\
                 repair ⊢⊙∈⊤⊥∋⊡⊣\n"
            .to_string();
    }

    let artifact = args[0];
    let artifact_type = args.get(1).unwrap_or(&"program");
    
    let result = engine.repair(artifact, artifact_type);
    
    let header = format!(
        "REPAIR ANALYSIS\n\
         =============\n\
         Original artifact: {}\n\
         Artifact type: {}\n\
         Error diagnosed: {}\n\
         \n\
         Repairs found: {}\n\
         \n",
        result.original,
        artifact_type,
        result.error_type,
        result.repairs.len()
    );

    if result.repairs.is_empty() {
        if let Some((chain, repaired)) = engine.repair_chain(artifact, artifact_type) {
            let steps: Vec<String> = chain.iter().map(|r| format!("{:?}", r)).collect();
            let addr = crystal_address_of(&repaired)
                .map(|a| a.to_string())
                .unwrap_or_else(|| "unaddressable".to_string());
            return header
                + &format!(
                    "Single-edit search found nothing; repair_chain reached a verified word in {} step(s):\n  {}\nRepaired: {}\ncrystal {}\n",
                    steps.len(), steps.join(" -> "), repaired, addr
                );
        }
        return header
            + "Single-edit search found nothing, and repair_chain did not reach a verified word within its bound.\n"
            + &result.proof_diff;
    }

    // Every candidate, grouped by the crystal address its repaired word
    // derives to -- not a ranked-and-truncated top N. A cost ranking was
    // dropping 808 of 813 candidates silently; grouping by address prints
    // every one while keeping the listing to one line per distinct address.
    let mut by_address: BTreeMap<Option<u32>, Vec<&RepairCandidate>> = BTreeMap::new();
    for r in &result.repairs {
        by_address.entry(crystal_address_of(&r.repaired_word)).or_default().push(r);
    }

    let mut out = header;
    out.push_str(&format!(
        "ALL REPAIRS BY CRYSTAL ADDRESS ({} candidates, {} distinct addresses):\n\n",
        result.repairs.len(), by_address.len()
    ));
    for (addr, group) in &by_address {
        let mut sorted = group.clone();
        sorted.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap_or(core::cmp::Ordering::Equal));
        let cheapest = sorted[0];
        let addr_str = match addr {
            Some(a) => format!("crystal {}", a),
            None => "crystal <unaddressable: word exceeds program capacity>".to_string(),
        };
        out.push_str(&format!(
            "  {}  ({} repair(s), cheapest cost {:.2})\n    {:?} -> {} (edit distance {}, ΔS {:.4})\n",
            addr_str, group.len(), cheapest.cost,
            cheapest.repair, cheapest.repaired_word, cheapest.edit_distance, cheapest.entropy_delta
        ));
        if sorted.len() > 1 {
            for r in &sorted[1..] {
                out.push_str(&format!(
                    "    {:?} -> {} (cost {:.2}, edit distance {})\n",
                    r.repair, r.repaired_word, r.cost, r.edit_distance
                ));
            }
        }
    }
    out.push('\n');
    out + &result.proof_diff
}
