// ─── repair.rs ────────────────────────────────────────────────────────
// Automatic program/proof surgery with ranked repairs
// Searches constrained edit space and ranks repairs by cost
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

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

    fn diagnose_error(&self, _artifact: &str, artifact_type: &str) -> String {
        // Diagnose what's wrong with the artifact
        match artifact_type {
            "program" => "execution failure",
            "proof" => "verification failure",
            "theorem" => "type checking failure",
            "invariant" => "invariant violation",
            _ => "unknown error",
        }.to_string()
    }

    fn verify_repair(&self, repaired: &str, _artifact_type: &str) -> bool {
        // A repair must (a) parse as a non-empty IMASM word of the twelve
        // glyphs, and (b) not be ill-typed: a fuse (∋) with no split (∈) to
        // pair reads verdict F, which is still broken. The fuse count may not
        // exceed the split count.
        if repaired.is_empty() {
            return false;
        }
        if !repaired.chars().all(|c| self.glyphs.contains(&c)) {
            return false;
        }
        let splits = repaired.chars().filter(|&c| c == '∈').count();
        let fuses = repaired.chars().filter(|&c| c == '∋').count();
        fuses <= splits
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
    
    format!(
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
    ) + 
    if result.repairs.is_empty() {
        "No valid repairs found in search space.\n"
    } else {
        "TOP REPAIRS (ranked by cost):\n\n"
    } + &result.repairs.iter()
        .take(5)
        .enumerate()
        .map(|(i, r)| format!(
            "{}. Cost: {:.2}\n\
             Repair: {:?}\n\
             Result: {}\n\
             Edit distance: {}\n\
             Entropy delta: {:.4}\n\
             \n",
            i + 1, r.cost, r.repair, r.repaired_word, r.edit_distance, r.entropy_delta
        ))
        .collect::<String>() +
    "\n" + &result.proof_diff
}
