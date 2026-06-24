// bifurcation_test.rs — Experimental determination of w_c
// This module varies substrate_weight and measures convergence behavior.
// Run with: cargo test --target x86_64-unknown-linux-gnu (host test)

#[cfg(test)]
mod bifurcation_tests {
    use crate::sequence;
    use crate::imas_ig::{IgTuple, IgPrim};

    // Helper: build a test tuple at O_∞ tier
    fn test_tuple_oinf() -> IgTuple {
        IgTuple {
            d: IgPrim::Odot,       // D=𐑦
            t: IgPrim::TOdot,      // T=𐑸
            r: IgPrim::LR,         // R=𐑾
            p: IgPrim::PmSym,      // P=𐑹
            f: IgPrim::Hbar,       // F=𐑐
            k: IgPrim::Slow,       // K=𐑧
            g: IgPrim::Aleph,      // G=𐑲
            c: IgPrim::Seq,        // C=𐑠
            phi: IgPrim::Monad,    // Phi=⊙
            h: IgPrim::Hinf,       // H=𐑫
            s: IgPrim::Up,         // S=𐑳
            omega: IgPrim::Ah,     // Omega=𐑭
        }
    }

    /// Measure how many distinct programs are generated before cycling.
    /// Returns the cycle length detected.
    fn measure_cycle(tuple: &IgTuple, weight: i32, max_iter: usize) -> usize {
        sequence::set_substrate_weight(weight);
        let mut seen: Vec<Vec<crate::tokens::Token>> = Vec::new();
        let mut prog = sequence::build_via_substrate(tuple, 12, false, 3);
        seen.push(prog.as_slice().to_vec());
        for i in 1..max_iter {
            let next = sequence::build_via_substrate(tuple, 12, false, 3);
            let tokens: Vec<crate::tokens::Token> = next.as_slice().to_vec();
            for (j, prev) in seen.iter().enumerate() {
                if prev == &tokens {
                    return i - j; // cycle length
                }
            }
            seen.push(tokens);
            prog = next;
        }
        max_iter // no cycle found
    }

    #[test]
    fn test_bifurcation_scan() {
        println!("Weight | Cycle Length");
        println!("-------|-------------");
        for w in 0..=10 {
            let prev = sequence::substrate_weight();
            let cycle = measure_cycle(&test_tuple_oinf(), w, 20);
            println!("{:>6} | {:>11}", w, cycle);
            // For w >= 3 we expect cycle >= 3 (O_∞), for w <= 1 we expect cycle <= 2
            if w >= 3 {
                assert!(cycle >= 3, "Weight {} expected O_∞ but got cycle {}", w, cycle);
            }
        }
    }

    #[test]
    fn test_w_eq_1_produces_o2() {
        sequence::set_substrate_weight(1);
        let tuple = test_tuple_oinf();
        let cycle = measure_cycle(&tuple, 1, 20);
        assert!(cycle <= 2, "w=1 expected O2 but got cycle {}", cycle);
    }

    #[test]
    fn test_w_eq_3_produces_oinf() {
        sequence::set_substrate_weight(3);
        let tuple = test_tuple_oinf();
        let cycle = measure_cycle(&tuple, 3, 20);
        assert!(cycle >= 3, "w=3 expected O_∞ but got cycle {}", cycle);
    }
}
