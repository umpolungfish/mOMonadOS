// ─── invariant.rs ─────────────────────────────────────────────
// Discover what survives transformations: ROTAT, IMSCRIB, FSPLIT/FFUSE, etc.
// Searches for quantities that remain unchanged across transformation families
#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum InvariantType {
    Topology,
    Tier,
    TruthState,
    DistanceClass,
    CycleLength,
    Entropy,
    PrimitiveCount,
    Algebraic,
    ProofStatus,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct InvariantResult {
    pub name: String,
    pub r#type: InvariantType,
    pub value: String,
    pub transformations_tested: Vec<String>,
    pub transformations_passed: Vec<String>,
    pub transformations_failed: Vec<String>,
    pub objects_tested: usize,
    pub counterexamples: Vec<String>,
}

pub struct InvariantEngine {
    transformations: Vec<String>,
}

impl InvariantEngine {
    pub fn new() -> Self {
        Self {
            transformations: vec![
                "ROTAT".to_string(),
                "IMSCRIB".to_string(),
                "FSPLIT".to_string(),
                "FFUSE".to_string(),
                "AFWD".to_string(),
                "AREV".to_string(),
                "CLINK".to_string(),
                "EVALT".to_string(),
                "EVALF".to_string(),
                "ENGAGR".to_string(),
                "IFIX".to_string(),
                "all".to_string(),
            ],
        }
    }

    pub fn test_invariant(&self, object: &str, transformation: &str) -> bool {
        // Test whether `object` is invariant under `transformation`. The test
        // is computed at the word level: ROTAT = cyclic shift, IMSCRIB = the
        // self-referential fixed point, AREV = reversal, AFWD = advance.
        // Opcodes with no word-level meaning at this layer fail closed.
        match transformation {
            "ROTAT" => self.test_rotat_invariant(object),
            "IMSCRIB" => self.test_imscrib_invariant(object),
            "AREV" => self.test_arev_invariant(object),
            "AFWD" => self.test_afwd_invariant(object),
            "all" => self.test_all_transformations(object),
            _ => false, // fail closed: unverifiable at this layer is not invariant
        }
    }

    fn test_rotat_invariant(&self, object: &str) -> bool {
        // ROTAT is the cyclic shift. A word is ROTAT-invariant iff its whole
        // rotation orbit is a single point, i.e. every glyph is identical.
        let chars: Vec<char> = object.chars().collect();
        if chars.len() <= 1 { return true; }
        chars.iter().all(|&c| c == chars[0])
    }

    fn test_imscrib_invariant(&self, object: &str) -> bool {
        // IMSCRIB (⊙) is self-reference. A word is a fixed point under
        // imscription iff it is exactly the self-referential atom, or empty.
        object.is_empty() || object == "⊙"
    }

    fn test_arev_invariant(&self, object: &str) -> bool {
        // AREV is the clearing reverse. Invariant iff palindromic.
        let chars: Vec<char> = object.chars().collect();
        chars.iter().eq(chars.iter().rev())
    }

    fn test_afwd_invariant(&self, object: &str) -> bool {
        // AFWD advances one step. Only the empty word is a fixed point of a
        // non-trivial advance at this layer.
        object.is_empty()
    }

    fn test_all_transformations(&self, object: &str) -> bool {
        // Test against all transformations
        for t in &self.transformations {
            if t != "all" && !self.test_invariant(object, t) {
                return false;
            }
        }
        true
    }

    pub fn search_catalog(&self, catalog: &str, transformation: &str) -> Vec<InvariantResult> {
        // Test the catalog word (the only object in scope) under the requested
        // transformation(s) and record the computed outcome. One result per
        // transformation actually tested.
        let transformations: Vec<String> = if transformation == "all" {
            self.transformations.iter().filter(|t| *t != "all").cloned().collect()
        } else {
            vec![transformation.to_string()]
        };

        let mut results = Vec::new();
        for t in &transformations {
            let passed = self.test_invariant(catalog, t);
            results.push(InvariantResult {
                name: t.clone(),
                r#type: InvariantType::Custom(t.clone()),
                value: if passed { "invariant".to_string() } else { "variant".to_string() },
                transformations_tested: vec![t.clone()],
                transformations_passed: if passed { vec![t.clone()] } else { vec![] },
                transformations_failed: if passed { vec![] } else { vec![t.clone()] },
                objects_tested: 1,
                counterexamples: if passed { vec![] } else { vec![catalog.to_string()] },
            });
        }
        results
    }

    pub fn census(&self, catalog: &str) -> String {
        let mut out = format!(
            "INVARIANT CENSUS\n================\n\
             Catalog: {}\n\
             Transformations: {}\n\n",
            catalog,
            self.transformations.iter().filter(|t| *t != "all").count()
        );
        for t in &self.transformations {
            if t == "all" { continue; }
            let verdict = if self.test_invariant(catalog, t) { "INVARIANT" } else { "variant" };
            out.push_str(&format!("  {}: {}\n", t, verdict));
        }
        out
    }
}

pub fn invariant_main(args: &[&str]) -> String {
    let engine = InvariantEngine::new();
    
    if args.is_empty() {
        return "USAGE:\n\
                 invariant <catalog> under <transformation>\n\
                 invariant <catalog> under all\n\
                 invariant census <catalog>\n\
                 \n\
                 Examples:\n\
                 invariant catalog under ROTAT\n\
                 invariant catalog under IMSCRIB\n\
                 invariant census catalog\n"
            .to_string();
    }

    match args[0] {
        "census" => {
            let catalog = args.get(1).unwrap_or(&"catalog");
            engine.census(catalog)
        }
        "under" => {
            let catalog = args.get(1).unwrap_or(&"catalog");
            let transformation = args.get(2).unwrap_or(&"all");
            let results = engine.search_catalog(catalog, transformation);
            
            format!(
                "INVARIANTS UNDER {}\n====================\n\
                 Catalog: {}\n\
                 Transformation: {}\n\
                 \n\
                 Discovered invariants: {}\n",
                transformation, catalog, transformation, results.len()
            )
        }
        _ => {
            // Assume first arg is catalog, look for "under" keyword
            let catalog = args[0];
            let transformation = args.iter()
                .skip(1)
                .find(|&&s| s == "ROTAT" || s == "IMSCRIB" || s == "all")
                .unwrap_or(&"all");
            
            engine.search_catalog(catalog, transformation)
                .iter()
                .map(|r| format!("- {}: {} (tested: {})", r.name, r.value, r.objects_tested))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}
