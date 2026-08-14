// ─── dialetheic_compiler.rs ────────────────────────────────────────────
// Boolean → Belnap compiler (spec: dialetheic-compiler).
//
// Take a classical 2-input gate, run its truth table, and lift each row into
// Belnap FOUR. The lift feeds the two classical bits as T/F into the kernel's
// own knowledge-consensus operators (band = join, T⊗F = B) and shows exactly
// where a classical row secretly rests on a paradox (B) or a gap (N).
#![allow(dead_code)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use crate::belnap::B4;

fn classical(gate: &str, a: bool, b: bool) -> Option<bool> {
    Some(match gate {
        "and" => a && b,
        "or" => a || b,
        "xor" => a ^ b,
        "nand" => !(a && b),
        "nor" => !(a || b),
        "xnor" => !(a ^ b),
        "imp" => !a || b,
        _ => return None,
    })
}

fn bit(x: bool) -> B4 { if x { B4::T } else { B4::F } }

pub fn dialetheic_compiler_main(args: &[&str]) -> String {
    let flat: alloc::vec::Vec<&str> = args.iter().flat_map(|s| s.split_whitespace()).collect();
    let gate = flat.first().copied().unwrap_or("");
    if gate.is_empty() || gate == "help" {
        return "dialetheic-compiler <gate>\n\n\
                gates: and or xor nand nor xnor imp\n\n\
                Runs the classical truth table and lifts each row into Belnap\n\
                FOUR. band is the knowledge join (T⊗F=B, a manufactured\n\
                paradox); bor is the meet (T⊕F=N, no shared ground). Where the\n\
                Belnap column reads B the classical row rested on a dialetheia.\n\n\
                Try:  dialetheic-compiler xor\n".to_string();
    }
    if classical(gate, false, false).is_none() {
        return format!("no gate '{}'. gates: and or xor nand nor xnor imp\n", gate);
    }
    let mut out = String::from("DIALETHEIC-COMPILER\n===================\n\n");
    out.push_str(&format!("gate: {}\n\n", gate));
    out.push_str("  a b | classical | band(a,b) bor(a,b)\n");
    out.push_str("  ----+-----------+-------------------\n");
    let mut classic_row = String::new();
    let mut belnap_row = String::new();
    for &(a, b) in &[(false, false), (false, true), (true, false), (true, true)] {
        let c = classical(gate, a, b).unwrap();
        let band = bit(a).band(bit(b));
        let bor = bit(a).bor(bit(b));
        out.push_str(&format!(
            "  {} {} |     {}     |    {}        {}\n",
            bit(a).name(), bit(b).name(), bit(c).name(), band.name(), bor.name()
        ));
        classic_row.push_str(bit(c).name());
        belnap_row.push_str(band.name());
    }
    out.push_str(&format!("\nclassical: {}\n", classic_row));
    out.push_str(&format!("belnap:    {}\n", belnap_row));
    let paradoxes = belnap_row.matches('B').count();
    let gaps = belnap_row.matches('N').count();
    out.push_str(&format!(
        "\n{} of 4 rows lift to a paradox (B), {} to a gap (N) under the\n\
         knowledge join. Those are the rows classical logic collapses.\n",
        paradoxes, gaps
    ));
    out
}
