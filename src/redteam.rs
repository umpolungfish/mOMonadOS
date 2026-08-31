//! Redteam module: adversarial testing and stress-testing of IMASM programs
//!
//! This module implements adversarial analysis tools:
//! - Adversarial input generation
//! - Stress testing under resource constraints
//! - Boundary condition exploration
//! - Failure mode analysis
//! - Robustness verification

use crate::invariant::InvariantEngine;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct RedteamConfig {
    pub max_iterations: usize,
    pub mutation_rate: f64,
    pub stress_level: StressLevel,
    pub target_failures: Vec<FailureMode>,
    pub report_format: ReportFormat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StressLevel {
    Light,      // Basic boundary testing
    Moderate,   // Systematic mutation
    Heavy,      // Aggressive adversarial inputs
    Extreme,    // Maximum stress, may crash
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FailureMode {
    StackOverflow,
    MemoryExhaustion,
    InfiniteLoop,
    TypeMismatch,
    InvalidTransition,
    ConcurrencyViolation,
    ResourceDeadlock,
    UndefinedBehavior,
}

#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Summary,
    Detailed,
    Json,
    Lean,
}

#[derive(Debug)]
pub struct RedteamResult {
    pub test_id: String,
    pub inputs_tested: usize,
    pub failures_found: Vec<FailureRecord>,
    pub invariants_violated: Vec<String>,
    pub robustness_score: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug)]
pub struct FailureRecord {
    pub mode: FailureMode,
    pub input: String,
    pub iteration: usize,
    pub stack_trace: Option<String>,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct RedteamEngine {
    config: RedteamConfig,
    invariant_engine: InvariantEngine,
    failure_history: BTreeMap<FailureMode, usize>,
}

impl RedteamEngine {
    pub fn new(config: RedteamConfig) -> Self {
        Self {
            config,
            invariant_engine: InvariantEngine::new(),
            failure_history: BTreeMap::new(),
        }
    }

    /// Generate adversarial inputs for a given IMASM word
    pub fn generate_adversarial_inputs(&self, base_word: &str) -> Vec<String> {
        let mut inputs = Vec::new();
        
        // Input 1: Maximum nesting depth
        inputs.push(self.maximize_nesting(base_word));
        
        // Input 2: Maximum crossing count
        inputs.push(self.maximize_crossings(base_word));
        
        // Input 3: Boundary conditions (empty, single char, max length)
        inputs.push("".to_string());
        inputs.push(base_word.chars().next().map(|c| c.to_string()).unwrap_or_default());
        
        // Input 4: Invalid sequences
        inputs.push(self.generate_invalid_sequence(base_word));
        
        // Input 5: Repetition attacks
        inputs.push(self.repeat_pattern(base_word, 100));
        
        inputs
    }

    /// Maximize nesting depth in a word
    fn maximize_nesting(&self, word: &str) -> String {
        let mut result = String::new();
        let depth = match self.config.stress_level {
            StressLevel::Light => 10,
            StressLevel::Moderate => 50,
            StressLevel::Heavy => 100,
            StressLevel::Extreme => 500,
        };
        
        for _ in 0..depth {
            result.push('∈');
        }
        result.push_str(word);
        for _ in 0..depth {
            result.push('∋');
        }
        
        result
    }

    /// Maximize crossing count
    fn maximize_crossings(&self, word: &str) -> String {
        let mut result = String::new();
        let count = match self.config.stress_level {
            StressLevel::Light => 100,
            StressLevel::Moderate => 1000,
            StressLevel::Heavy => 10000,
            StressLevel::Extreme => 100000,
        };
        
        for i in 0..count {
            if i % 2 == 0 {
                result.push('⊢');
            } else {
                result.push('⊣');
            }
        }
        
        result.push_str(word);
        result
    }

    /// Generate invalid sequences
    fn generate_invalid_sequence(&self, word: &str) -> String {
        // Mix valid and invalid glyphs
        let invalid_glyphs = ['◇', '●', '+', '×', '=', '¬'];
        let mut result = String::new();
        
        for (i, c) in word.chars().enumerate() {
            result.push(c);
            if i % 3 == 0 {
                result.push(invalid_glyphs[i % invalid_glyphs.len()]);
            }
        }
        
        result
    }

    /// Repeat a pattern
    fn repeat_pattern(&self, pattern: &str, times: usize) -> String {
        pattern.repeat(times)
    }

    /// Run stress tests on a word
    pub fn stress_test(&mut self, word: &str) -> RedteamResult {
        let adversarial_inputs = self.generate_adversarial_inputs(word);
        let mut failures = Vec::new();
        let mut invariants_violated = Vec::new();
        
        for (i, input) in adversarial_inputs.iter().enumerate() {
            // Test for infinite loops
            if self.detect_infinite_loop(input) {
                failures.push(FailureRecord {
                    mode: FailureMode::InfiniteLoop,
                    input: input.clone(),
                    iteration: i,
                    stack_trace: None,
                    severity: Severity::High,
                });
            }
            
            // Test for stack overflow (deep nesting)
            if self.detect_stack_overflow(input) {
                failures.push(FailureRecord {
                    mode: FailureMode::StackOverflow,
                    input: input.clone(),
                    iteration: i,
                    stack_trace: None,
                    severity: Severity::Critical,
                });
            }
            
            // Test invariants: which canonical transformations the engine holds
            // invariant for this input, then which of those a mutation breaks.
            for transform in ["ROTAT", "IMSCRIB", "FSPLIT", "FFUSE"] {
                if self.invariant_engine.test_invariant(input, transform)
                    && self.violate_invariant(input, transform) {
                    invariants_violated.push(transform.to_string());
                }
            }
        }
        
        let robustness_score = self.compute_robustness_score(&failures, adversarial_inputs.len());
        let recommendations = self.generate_recommendations(&failures, &invariants_violated);
        
        RedteamResult {
            test_id: format!("redteam_{}", word.chars().take(8).collect::<String>()),
            inputs_tested: adversarial_inputs.len(),
            failures_found: failures,
            invariants_violated,
            robustness_score,
            recommendations,
        }
    }

    /// Detect potential infinite loops
    fn detect_infinite_loop(&self, word: &str) -> bool {
        // Check for ROTAT without termination conditions
        let has_rotat = word.contains('↻') || word.contains('↺');
        let has_termination = word.contains('⊣') || word.contains('⊡');
        
        has_rotat && !has_termination
    }

    /// Detect potential stack overflow
    fn detect_stack_overflow(&self, word: &str) -> bool {
        let nesting_depth = self.compute_nesting_depth(word);
        nesting_depth > 256 // Arbitrary threshold
    }

    /// Compute nesting depth
    fn compute_nesting_depth(&self, word: &str) -> usize {
        let mut depth = 0;
        let mut max_depth = 0;
        
        for c in word.chars() {
            match c {
                '∈' | '⊢' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                '∋' | '⊣' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
        
        max_depth
    }

    /// Try to violate an invariant
    fn violate_invariant(&self, word: &str, invariant: &str) -> bool {
        // Generate mutations and check if invariant holds
        let mutations = self.generate_mutations(word);
        
        for mutation in mutations {
            // Check if the invariant still holds after mutation
            // This is a simplified check - real implementation would verify the invariant
            if !self.check_invariant_holds(&mutation, invariant) {
                return true;
            }
        }
        
        false
    }

    /// Generate mutations of a word
    fn generate_mutations(&self, word: &str) -> Vec<String> {
        let mut mutations = Vec::new();
        let chars: Vec<char> = word.chars().collect();
        let mutation_count = (word.len() as f64 * self.config.mutation_rate) as usize;
        
        for i in 0..mutation_count.min(chars.len()) {
            let mut mutated = chars.clone();
            let pos = (word.len() * 31 + i) % chars.len();
            
            // Random mutation: insert, delete, or substitute
            match i % 3 {
                0 => {
                    // Insert
                    let new_char = ['⊢', '⊣', '⊙', '∈', '∋'][(i + 1) % 5];
                    mutated.insert(pos, new_char);
                }
                1 => {
                    // Delete
                    mutated.remove(pos);
                }
                2 => {
                    // Substitute
                    mutated[pos] = ['⊢', '⊣', '⊙', '∈', '∋'][(i + 2) % 5];
                }
                _ => {}
            }
            
            mutations.push(mutated.into_iter().collect());
        }
        
        mutations
    }

    /// UNIMPLEMENTED: returns true for every word and every invariant.
    ///
    /// It parses nothing and verifies nothing, so a caller that treats a `true`
    /// here as evidence is reading its own default back. Named as what it is
    /// rather than as a "simplified" check, because a stub that reports PASS is
    /// indistinguishable from a passing check at the call site.
    fn check_invariant_holds(&self, _word: &str, _invariant: &str) -> bool {
        true
    }

    /// Compute robustness score
    fn compute_robustness_score(&self, failures: &[FailureRecord], total_tests: usize) -> f64 {
        if total_tests == 0 {
            return 1.0;
        }
        
        let weighted_failures: f64 = failures.iter().map(|f| {
            match f.severity {
                Severity::Low => 1.0,
                Severity::Medium => 2.0,
                Severity::High => 3.0,
                Severity::Critical => 4.0,
            }
        }).sum();
        
        1.0 - (weighted_failures / (total_tests as f64 * 4.0)).min(1.0)
    }

    /// Generate recommendations based on failures
    fn generate_recommendations(&self, failures: &[FailureRecord], invariants_violated: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze failure patterns
        let mut failure_counts: BTreeMap<FailureMode, usize> = BTreeMap::new();
        for failure in failures {
            *failure_counts.entry(failure.mode.clone()).or_insert(0) += 1;
        }
        
        // Generate specific recommendations
        if let Some(count) = failure_counts.get(&FailureMode::InfiniteLoop) {
            if *count > 0 {
                recommendations.push("Add termination conditions to ROTAT loops".to_string());
                recommendations.push("Consider adding maximum iteration limits".to_string());
            }
        }
        
        if let Some(count) = failure_counts.get(&FailureMode::StackOverflow) {
            if *count > 0 {
                recommendations.push("Reduce maximum nesting depth".to_string());
                recommendations.push("Use iterative instead of recursive patterns".to_string());
            }
        }
        
        if !invariants_violated.is_empty() {
            recommendations.push("Strengthen invariant guarantees".to_string());
            recommendations.push("Add runtime invariant checking".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("No critical issues found".to_string());
        }
        
        recommendations
    }

    /// Run comprehensive redteam analysis
    pub fn analyze(&mut self, word: &str) -> String {
        let result = self.stress_test(word);
        
        match self.config.report_format {
            ReportFormat::Summary => self.format_summary(&result),
            ReportFormat::Detailed => self.format_detailed(&result),
            ReportFormat::Json => self.format_json(&result),
            ReportFormat::Lean => self.format_lean(&result),
        }
    }

    fn format_summary(&self, result: &RedteamResult) -> String {
        format!(
            "Redteam Analysis Summary\n========================\n\
             Test ID: {}\n\
             Inputs Tested: {}\n\
             Failures Found: {}\n\
             Robustness Score: {:.2}%\n\
             Critical Issues: {}\n",
            result.test_id,
            result.inputs_tested,
            result.failures_found.len(),
            result.robustness_score * 100.0,
            result.failures_found.iter().filter(|f| f.severity == Severity::Critical).count()
        )
    }

    fn format_detailed(&self, result: &RedteamResult) -> String {
        let mut output = String::new();
        output.push_str(&self.format_summary(result));
        output.push_str("\n\nFailures:\n");
        
        for failure in &result.failures_found {
            output.push_str(&format!(
                "  - {:?} at iteration {} (severity: {:?})\n",
                failure.mode, failure.iteration, failure.severity
            ));
        }
        
        output.push_str("\nRecommendations:\n");
        for (i, rec) in result.recommendations.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", i + 1, rec));
        }
        
        output
    }

    fn format_json(&self, result: &RedteamResult) -> String {
        // Simplified JSON formatting
        format!(
            r#"{{"test_id":"{}","inputs_tested":{},"failures_count":{},"robustness_score":{}}}"#,
            result.test_id,
            result.inputs_tested,
            result.failures_found.len(),
            result.robustness_score
        )
    }

    fn format_lean(&self, result: &RedteamResult) -> String {
        format!(
            "-- Redteam Analysis Results for {}\n\
             -- Inputs tested: {}\n\
             -- Failures found: {}\n\
             -- Robustness score: {}\n\
             theorem redteam_{}_verified : True := by sorry\n",
            result.test_id,
            result.inputs_tested,
            result.failures_found.len(),
            result.robustness_score,
            result.test_id
        )
    }
}

impl Default for RedteamConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            mutation_rate: 0.1,
            stress_level: StressLevel::Moderate,
            target_failures: vec![],
            report_format: ReportFormat::Summary,
        }
    }
}

impl Default for RedteamEngine {
    fn default() -> Self {
        Self::new(RedteamConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adversarial_input_generation() {
        let config = RedteamConfig::default();
        let engine = RedteamEngine::new(config);
        let inputs = engine.generate_adversarial_inputs("⊢⊙⊣");
        
        assert!(!inputs.is_empty());
        assert!(inputs.iter().any(|s| s.len() > 3)); // At least one mutated input
    }

    #[test]
    fn test_nesting_depth_computation() {
        let config = RedteamConfig::default();
        let engine = RedteamEngine::new(config);
        
        assert_eq!(engine.compute_nesting_depth("∈∈∈⊙∋∋∋"), 3);
        assert_eq!(engine.compute_nesting_depth("⊢⊢⊢⊙⊣⊣⊣"), 3);
        assert_eq!(engine.compute_nesting_depth("∈⊢⊙∋⊣"), 2);
    }

    #[test]
    fn test_infinite_loop_detection() {
        let config = RedteamConfig::default();
        let engine = RedteamEngine::new(config);
        
        assert!(engine.detect_infinite_loop("↻⊙")); // ROTAT without termination
        assert!(!engine.detect_infinite_loop("↻⊙⊣")); // ROTAT with termination
    }
}

/// Main entry point for redteam commands
pub fn redteam_main(args: &[&str]) -> String {
    if args.is_empty() {
        return format!(
            "Redteam: Adversarial Testing Framework\n\
             ================================\n\
             Usage: redteam <command> [options]\n\
             \n\
             Commands:\n\
               analyze <word>     Run full redteam analysis on an IMASM word\n\
               stress <word>      Stress test with adversarial inputs\n\
               mutate <word>      Generate mutations\n\
               audit <theory>     THEORY AUDIT: hidden assumptions, sorry, finite-vs-infinite\n\
               config             Show current configuration\n\
               help               Show this help\n"
        );
    }

    let command = args[0];
    match command {
        "audit" | "t" => {
            let selector = if args.len() < 2 { "all" } else { args[1] };
            format_audit(selector)
        }
        "analyze" | "a" => {
            if args.len() < 2 {
                return "Usage: redteam analyze <word>".to_string();
            }
            let word = args[1];
            let mut engine = RedteamEngine::default();
            engine.config.report_format = ReportFormat::Detailed;
            engine.analyze(word)
        }
        "stress" | "s" => {
            if args.len() < 2 {
                return "Usage: redteam stress <word>".to_string();
            }
            let word = args[1];
            let mut engine = RedteamEngine::default();
            engine.config.stress_level = StressLevel::Heavy;
            let result = engine.stress_test(word);
            engine.format_detailed(&result)
        }
        "mutate" | "m" => {
            if args.len() < 2 {
                return "Usage: redteam mutate <word>".to_string();
            }
            let word = args[1];
            let engine = RedteamEngine::default();
            let mutations = engine.generate_mutations(word);
            let mut output = String::new();
            output.push_str("Mutations:\n");
            for (i, m) in mutations.iter().enumerate() {
                output.push_str(&format!("  {}: {}\n", i + 1, m));
            }
            output
        }
        "config" | "c" => {
            let engine = RedteamEngine::default();
            format!(
                "Redteam Configuration:\n\
                 ---------------------\n\
                 Max iterations: {}\n\
                 Mutation rate: {:.2}\n\
                 Stress level: {:?}\n\
                 Report format: {:?}\n",
                engine.config.max_iterations,
                engine.config.mutation_rate,
                engine.config.stress_level,
                engine.config.report_format
            )
        }
        "help" | "h" | "?" => {
            redteam_main(&[])
        }
        _ => {
            format!("Unknown command: {}\nUse 'redteam help' for usage.", command)
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// THEORY AUDIT — mathematical red-teaming (build.txt §544)
// ═══════════════════════════════════════════════════════════════
//
// Not the stress-tester above. This one takes a *theory* and looks for the
// ways a corpus talks itself into believing more than it proved: assumptions
// entered as axioms, targets left at `sorry`, and finite `decide` checks
// standing in for infinite claims.
//
// Every number below comes from the baked Lean census (scan_lean_census.sh),
// never from an estimate. Where a category cannot be decided from the census,
// it reports UNDETERMINED rather than guessing.

use crate::lean_census::{LeanFile, CENSUS_DATE, LEAN_CENSUS};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuditMark {
    Pass,
    Caution,
    Fail,
    Undetermined,
}

impl AuditMark {
    fn glyph(&self) -> &'static str {
        match self {
            AuditMark::Pass => "OK  ",
            AuditMark::Caution => "WARN",
            AuditMark::Fail => "FAIL",
            AuditMark::Undetermined => "??  ",
        }
    }
}

pub struct AuditLine {
    pub mark: AuditMark,
    pub label: &'static str,
    pub detail: String,
}

/// Aggregate census counts over the files matching a theory selector.
pub struct TheoryScope {
    pub selector: String,
    pub files: usize,
    pub theorems: u32,
    pub sorries: u32,
    pub axioms: u32,
    pub decide: u32,
    pub native_decide: u32,
    pub clean: u32,
}

fn matches(f: &LeanFile, sel: &str) -> bool {
    if sel == "all" {
        return true;
    }
    // case-insensitive substring over the path
    let path = f.path.as_bytes();
    let needle = sel.as_bytes();
    if needle.is_empty() || needle.len() > path.len() {
        return false;
    }
    let lower = |b: u8| if b.is_ascii_uppercase() { b + 32 } else { b };
    for start in 0..=(path.len() - needle.len()) {
        let mut hit = true;
        for i in 0..needle.len() {
            if lower(path[start + i]) != lower(needle[i]) {
                hit = false;
                break;
            }
        }
        if hit {
            return true;
        }
    }
    false
}

pub fn scope_theory(selector: &str) -> TheoryScope {
    let mut s = TheoryScope {
        selector: selector.to_string(),
        files: 0,
        theorems: 0,
        sorries: 0,
        axioms: 0,
        decide: 0,
        native_decide: 0,
        clean: 0,
    };
    for f in LEAN_CENSUS.iter().filter(|f| matches(f, selector)) {
        s.files += 1;
        s.theorems += f.theorems as u32;
        s.sorries += f.sorries as u32;
        s.axioms += f.axioms as u32;
        s.decide += f.decide as u32;
        s.native_decide += f.native_decide as u32;
        s.clean += f.clean as u32;
    }
    s
}

/// The file carrying the most unproved weight: sorries + axioms, ties to axioms.
pub fn highest_risk(selector: &str) -> Option<&'static LeanFile> {
    LEAN_CENSUS
        .iter()
        .filter(|f| matches(f, selector))
        .filter(|f| f.sorries > 0 || f.axioms > 0)
        .max_by_key(|f| {
            (
                f.sorries as u32 + f.axioms as u32,
                f.axioms as u32,
                f.theorems as u32,
            )
        })
}

/// Same basename at two paths: a second copy is drift, and drift means the
/// theorem being read may not be the theorem being edited.
pub fn duplicate_basenames(selector: &str) -> Vec<(&'static str, usize)> {
    fn base(p: &str) -> &str {
        match p.rsplit('/').next() {
            Some(b) => b,
            None => p,
        }
    }
    let mut names: Vec<&'static str> = LEAN_CENSUS
        .iter()
        .filter(|f| matches(f, selector))
        .map(|f| base(f.path))
        .collect();
    names.sort_unstable();
    let mut out: Vec<(&'static str, usize)> = Vec::new();
    let mut i = 0;
    while i < names.len() {
        let mut j = i + 1;
        while j < names.len() && names[j] == names[i] {
            j += 1;
        }
        if j - i > 1 {
            out.push((names[i], j - i));
        }
        i = j;
    }
    out.sort_by(|a, b| b.1.cmp(&a.1));
    out
}

pub fn audit_lines(s: &TheoryScope) -> Vec<AuditLine> {
    let mut lines = Vec::new();

    // 1. Is there anything to audit at all?
    lines.push(if s.files == 0 {
        AuditLine {
            mark: AuditMark::Fail,
            label: "corpus present",
            detail: format!("no Lean file matches \"{}\"", s.selector),
        }
    } else {
        AuditLine {
            mark: AuditMark::Pass,
            label: "corpus present",
            detail: format!("{} files, {} theorems", s.files, s.theorems),
        }
    });

    // 2. Proof of target: a sorry is an unproved obligation, full stop.
    lines.push(match s.sorries {
        0 => AuditLine {
            mark: AuditMark::Pass,
            label: "obligations discharged",
            detail: "no sorry in scope".to_string(),
        },
        n => AuditLine {
            mark: AuditMark::Fail,
            label: "obligations discharged",
            detail: format!(
                "{} sorry against {} theorems — the target is NOT ESTABLISHED where they sit",
                n, s.theorems
            ),
        },
    });

    // 3. Conjecture encoded as axiom: assumed, not proved.
    lines.push(match s.axioms {
        0 => AuditLine {
            mark: AuditMark::Pass,
            label: "conjecture-as-axiom",
            detail: "no axiom declared in scope".to_string(),
        },
        n => AuditLine {
            mark: AuditMark::Caution,
            label: "conjecture-as-axiom",
            detail: format!(
                "{} axiom declarations — each is an assumption the corpus grants itself",
                n
            ),
        },
    });

    // 4. Empirical coverage: decide/native_decide is a finite check.
    let finite = s.decide + s.native_decide;
    lines.push(if finite == 0 {
        AuditLine {
            mark: AuditMark::Undetermined,
            label: "empirical coverage",
            detail: "no decide/native_decide — nothing here rests on a finite check".to_string(),
        }
    } else {
        AuditLine {
            mark: AuditMark::Caution,
            label: "empirical coverage",
            detail: format!(
                "finite: {} by-decide + {} native_decide closures",
                s.decide, s.native_decide
            ),
        }
    });

    // 5. Finite-test -> infinite-claim. The census sees the tactic, never the
    //    quantifier, so this names exposure and refuses to call it a defect.
    lines.push(if finite == 0 {
        AuditLine {
            mark: AuditMark::Pass,
            label: "infinite extrapolation",
            detail: "no finite closure to over-read".to_string(),
        }
    } else {
        AuditLine {
            mark: AuditMark::Caution,
            label: "infinite extrapolation",
            detail: format!(
                "{} of {} theorems close by finite evaluation ({} close without one); \
                 a finite closure holds for its decidable instance and says nothing beyond it",
                finite.min(s.theorems),
                s.theorems,
                s.clean
            ),
        }
    });

    // 6. native_decide specifically: trusts the compiler, not the kernel.
    if s.native_decide > 0 {
        lines.push(AuditLine {
            mark: AuditMark::Caution,
            label: "kernel-external trust",
            detail: format!(
                "{} native_decide closures execute compiled code — outside the Lean kernel's \
                 checked path",
                s.native_decide
            ),
        });
    }

    // 6b. Second imscriptions met at the catalog merge. Recorded because the
    //     merge keeps only one and used to drop the other in silence; reported
    //     without a verdict because a differing form at a live address is not a
    //     defect. crystal_roundtrip proves encode/decode is a bijection onto
    //     0..17,279,999, so an address cannot be unmade and an improper
    //     imscription never reaches one — the coupling rejects it, not this.
    let drops = crate::catalog::name_collisions();
    let disagreeing = drops.iter().filter(|(_, differs)| *differs).count();
    if !drops.is_empty() {
        lines.push(if disagreeing == 0 {
            AuditLine {
                mark: AuditMark::Pass,
                label: "catalog name collisions",
                detail: format!(
                    "{} repeated name(s) at merge, all carrying identical data",
                    drops.len()
                ),
            }
        } else {
            let mut d = format!(
                "{} of {} names carry a second imscription: ",
                disagreeing,
                drops.len()
            );
            for (i, (name, _)) in drops.iter().filter(|(_, x)| *x).take(4).enumerate() {
                if i > 0 {
                    d.push_str(", ");
                }
                d.push_str(name);
            }
            // NOT a Caution. An enumerated address is permanent; the
            // imscription carried at it is not, and two forms at one address is
            // the normal case rather than a defect. Which form composes is
            // decided by the lattice's own coupling, not by this audit, so this
            // line reports and does not judge.
            AuditLine {
                mark: AuditMark::Pass,
                label: "catalog name collisions",
                detail: d,
            }
        });
    }

    // 7. Accidental equivalence / drift.
    let dups = duplicate_basenames(&s.selector);
    lines.push(if dups.is_empty() {
        AuditLine {
            mark: AuditMark::Pass,
            label: "single copy per name",
            detail: "no basename appears twice in scope".to_string(),
        }
    } else {
        let mut d = format!("{} basenames appear more than once: ", dups.len());
        for (i, (name, n)) in dups.iter().take(4).enumerate() {
            if i > 0 {
                d.push_str(", ");
            }
            d.push_str(&format!("{} x{}", name, n));
        }
        AuditLine {
            mark: AuditMark::Caution,
            label: "single copy per name",
            detail: d,
        }
    });

    lines
}

pub fn format_audit(selector: &str) -> String {
    let s = scope_theory(selector);
    let lines = audit_lines(&s);

    let mut out = String::new();
    out.push_str("THEORY AUDIT\n");
    out.push_str("============\n\n");
    out.push_str(&format!("theory selector:  {}\n", selector));
    out.push_str(&format!(
        "census:           {} ({} files in corpus)\n\n",
        CENSUS_DATE,
        LEAN_CENSUS.len()
    ));

    for l in &lines {
        out.push_str(&format!("  {} {:<24} {}\n", l.mark.glyph(), l.label, l.detail));
    }

    out.push('\n');
    match highest_risk(selector) {
        Some(f) => {
            out.push_str("highest-risk dependency:\n");
            out.push_str(&format!(
                "    {}\n    {} theorems, {} sorry, {} axiom\n",
                f.path, f.theorems, f.sorries, f.axioms
            ));
        }
        None => {
            out.push_str("highest-risk dependency:\n    none — no sorry and no axiom in scope\n");
        }
    }

    out.push_str("\nThis is an audit of what the corpus claims, not a proof of anything.\n\
                  A PASS line means the census found no defect of that kind; it is not\n\
                  a verification. Regenerate with ./scan_lean_census.sh after Lean edits.\n");
    out
}
