//! Composition at a protocol's split/fuse boundary.
use alloc::{format, string::{String, ToString}, vec::Vec};
use imasm_core::{check::{from_sequence, match_pairs}, classic::Token};

fn parse(word: &str) -> Result<Vec<Token>, String> {
    word.chars().filter(|c| !c.is_whitespace()).map(|c| {
        Token::parse(&c.to_string()).ok_or_else(|| format!("unknown mark {c}"))
    }).collect()
}

/// Insert a bounded process on the working arm immediately before the enclosing
/// fuse. Consume its source and terminal interfaces; retain its interior dyads.
/// A terminal payload fixation is fulfilled by the operator's post-fuse IFIX.
/// Executing it inside the arm would freeze the enclosing fuse itself.
/// The enclosing process must identify one unambiguous split/fuse pair.
pub fn enclose(operator: &str, payload: &str) -> Result<String, String> {
    let mut outer = parse(operator)?;
    let inner = parse(payload)?;
    for (label, ops) in [("operator", &outer), ("payload", &inner)] {
        if ops.first() != Some(&Token::Vinit) || ops.last() != Some(&Token::Tanch) {
            return Err(format!("{label} must have source and terminal interfaces"));
        }
        let g = from_sequence(ops, &match_pairs(ops));
        if !g.validate().is_empty() || !g.frobenius_closures().1 {
            return Err(format!("{label} is not a well-typed, fully paired protocol"));
        }
    }
    let graph = from_sequence(&outer, &match_pairs(&outer));
    let (pairs, _) = graph.frobenius_closures();
    if pairs.len() != 1 { return Err("operator needs one explicit enclosing dyad".into()); }
    let (_, fuse) = pairs[0];
    if outer[..fuse].contains(&Token::Ifix) {
        return Err("operator fixes before its enclosing fuse can execute".into());
    }
    let mut end = inner.len()-1;
    if inner[end-1] == Token::Ifix {
        if !outer[fuse+1..].contains(&Token::Ifix) {
            return Err("payload's terminal fixation needs an operator IFIX after the fuse".into());
        }
        end -= 1;
    }
    if inner[1..end].contains(&Token::Ifix) {
        return Err("payload fixes before its terminal interface; the enclosing fuse would be inert".into());
    }
    outer.splice(fuse..fuse, inner[1..end].iter().copied());
    let composed = from_sequence(&outer, &match_pairs(&outer));
    if !composed.validate().is_empty() || !composed.frobenius_closures().1 {
        return Err("composed protocol failed grammar or ancestry pairing".into());
    }
    Ok(outer.iter().map(|t| t.code()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn containment_preserves_inner_work_and_pairs_outer_seal() {
        let word = enclose("⊢∈⊙∋⊣", "⊢∈≻∋⊣").unwrap();
        assert_eq!(word, "⊢∈⊙∈≻∋∋⊣");
        let ops = parse(&word).unwrap();
        let g = from_sequence(&ops, &match_pairs(&ops));
        let (pairs, full) = g.frobenius_closures();
        assert!(full);
        assert_eq!(pairs.len(), 2);
        assert!(pairs.iter().all(|&(f,j)| g.transforms_between(f,j)));
    }
    #[test]
    fn identity_payload_stays_identity() {
        let word = enclose("⊢∈⊙∋⊣", "⊢∈⊙∋⊣").unwrap();
        let ops = parse(&word).unwrap();
        let g = from_sequence(&ops, &match_pairs(&ops));
        assert!(matches!(g.closure_state(), imasm_core::check::ClosureState::Identity));
    }
    #[test]
    fn refuses_ambiguous_or_open_interfaces() {
        assert!(enclose("⊢∈∈≻∋∋⊣", "⊢≻⊣").is_err());
        assert!(enclose("⊢∈⊙∋⊣", "⊢∈≻⊣").is_err());
        assert!(enclose("⊢∈⊙∋⊣", "invalid").is_err());
    }

    #[test]
    fn terminal_fixation_waits_for_the_enclosing_fuse() {
        let operator = "⊢⊙∈≻⊤≺⊥⋈⊞∋⊡⊣";
        let composed = enclose(operator, "⊢∈⊤∋⊡⊣").unwrap();
        let banked = imasm_core::lattice_flow::banked_walk(&composed).unwrap();
        assert!(banked.holds());
        assert_eq!(banked.inert, 1); // Only the final TANCH is after fixation.
        assert_eq!(composed.chars().filter(|&c| c == '⊡').count(), 1);
        assert_eq!(banked.reg, [1, 1, 1, 1]); // Fusing takes MAX, not a sum.
    }

    #[test]
    fn fixed_payload_cannot_freeze_an_uncommitted_parent() {
        assert!(enclose("⊢∈⊙∋⊣", "⊢∈⊤∋⊡⊣").is_err());
        assert!(enclose("⊢∈⊙∋⊡⊣", "⊢⊡∈⊤∋⊣").is_err());
        assert!(enclose("⊢∈⊡∋⊣", "⊢∈⊤∋⊣").is_err());
    }
}
