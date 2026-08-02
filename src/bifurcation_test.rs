// bifurcation_test.rs — Experimental determination of w_c
// This module varies substrate_weight and measures convergence behavior.
// Run with: cargo test --target x86_64-unknown-linux-gnu (host test)
//
// Result as it stands: cycle length is the wrong observable. It sits at 1 for
// every weight from 0 to 10 — each weight self-closes in a single step — so a
// scan of cycle lengths locates no w_c and would report the substrate as
// inert.
//
// The program's content does bifurcate, and sharply, at w_c = 1. At w=0 the
// substrate vote is annihilated and the ranking is the family matrix alone,
// which puts IMSCRIB one point above AFWD; the program is then pure
// self-imscription carrying no advance. From w=1 up the substrate vote is
// present, AFWD leads it, and the program advances. The margin being a single
// point is why the transition is a step and not a slope.

#[cfg(test)]
mod bifurcation_tests {
    use crate::sequence;
    use crate::imas_ig::{IgTuple, IgPrim};
    use alloc::vec::Vec;

    /// `sequence::SUBSTRATE_WEIGHT` is a process-wide `static mut`, so these
    /// tests cannot run concurrently — each would be reading a weight another
    /// had just overwritten. The lock serializes them.
    static WEIGHT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    // Helper: build a test tuple at O_∞ tier
    fn test_tuple_oinf() -> IgTuple {
        IgTuple {
            d: IgPrim::D_odot,       // D=𐑦
            t: IgPrim::T_odot,      // T=𐑸
            r: IgPrim::R_lr,         // R=𐑾
            p: IgPrim::P_pmsym,      // P=𐑹
            f: IgPrim::F_hbar,       // F=𐑐
            k: IgPrim::K_slow,       // K=𐑧
            g: IgPrim::G_aleph,      // G=𐑲
            c: IgPrim::C_seq,        // C=𐑠
            phi: IgPrim::Phi_crit,    // Phi=⊙
            h: IgPrim::H_inf,       // H=𐑫
            s: IgPrim::S_nm,         // S=𐑳
            omega: IgPrim::Omega_z,     // Omega=𐑭
        }
    }

    /// Measure how many distinct programs are generated before cycling.
    /// Returns the cycle length detected.
    fn measure_cycle(tuple: &IgTuple, weight: i32, max_iter: usize) -> usize {
        sequence::set_substrate_weight(weight);
        let mut seen: Vec<Vec<crate::tokens::Token>> = Vec::new();
        let mut current_tuple = *tuple;
        let mut prog = sequence::build_via_substrate(&current_tuple, 12, current_tuple.t == IgPrim::T_odot, 3);
        seen.push(prog.as_slice().to_vec());
        for i in 1..max_iter {
            let snap = crate::kernel::self_imscribe(&prog);
            current_tuple = crate::imas_ig::IgTuple::from_snapshot(&snap);
            let next = sequence::build_via_substrate(&current_tuple, 12, current_tuple.t == IgPrim::T_odot, 3);
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
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut table = [0usize; 11];
        for w in 0..=10usize {
            table[w] = measure_cycle(&test_tuple_oinf(), w as i32, 20);
        }
        std::println!("Weight | Cycle Length");
        std::println!("-------|-------------");
        for (w, &c) in table.iter().enumerate() {
            std::println!("{:>6} | {:>11}", w, c);
        }
        // Measured, not hypothesised: every weight self-closes in one step, so
        // cycle length is flat across the sweep and cannot locate w_c. The
        // transition it misses is in the program's content — see
        // test_program_bifurcates_at_w1.
        for (w, &c) in table.iter().enumerate() {
            assert_eq!(c, 1, "weight {} left the fixed point (cycle {})", w, c);
        }
    }

    /// The transition the cycle-length scan cannot see.
    ///
    /// `build_program_from_scores` reads the score vector only as a ranking:
    /// at each position it takes the first admissible token in preference
    /// order. So what the substrate weight can do is reorder that ranking, and
    /// here it reorders exactly once. With the substrate vote annihilated the
    /// family matrix alone leads with IMSCRIB, one point clear of AFWD, and
    /// the program is pure self-imscription. One unit of substrate vote is
    /// enough to overturn a one-point margin, so from w=1 up AFWD leads and
    /// the program advances, and it does not change again through w=10.
    #[test]
    fn test_program_bifurcates_at_w1() {
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tuple = test_tuple_oinf();
        let prog_at = |w: i32| -> Vec<crate::tokens::Token> {
            sequence::set_substrate_weight(w);
            sequence::build_via_substrate(&tuple, 12, true, 3).as_slice().to_vec()
        };

        let at_0 = prog_at(0);
        assert!(at_0.iter().all(|&t| t == crate::tokens::Token::Imscrib),
            "w=0 should carry no advance, got {:?}", at_0);

        let at_1 = prog_at(1);
        assert_ne!(at_0, at_1, "w_c is not 1 — the ranking did not flip at w=1");
        assert!(at_1.contains(&crate::tokens::Token::Afwd),
            "the advancing regime should carry AFWD, got {:?}", at_1);

        for w in 2..=10 {
            assert_eq!(prog_at(w), at_1, "w={} left the advancing regime", w);
        }
    }

    /// Where a w_c exists at all, and why it exists so rarely.
    ///
    /// The substrate weight can only move the program by overturning whichever
    /// token the family matrix leads with, so a w_c exists exactly where that
    /// leader is not already AFWD. Sweeping T and G shows that is a narrow
    /// place. Only the self-referential topology and the universal range leave
    /// IMSCRIB in front, and those are precisely the tuples the family matrix
    /// alone would leave spinning on themselves carrying no advance. Every
    /// other T or G already advances with the substrate vote annihilated, and
    /// no weight up to 64 changes anything about them. So what the substrate
    /// vote does is carry a self-referential tuple out of pure
    /// self-imscription, and on this evidence it does nothing else.
    #[test]
    fn test_wc_exists_only_where_family_leads_with_imscrib() {
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // The token the family matrix alone puts after the opening IMSCRIB.
        let leader = |t: &IgTuple| -> crate::tokens::Token {
            sequence::set_substrate_weight(0);
            sequence::build_via_substrate(t, 12, true, 3).as_slice()[1]
        };
        // Lowest weight whose program differs from the zero-weight program,
        // or None if no weight up to 64 does.
        let wc = |t: &IgTuple| -> Option<i32> {
            sequence::set_substrate_weight(0);
            let base: Vec<crate::tokens::Token> =
                sequence::build_via_substrate(t, 12, true, 3).as_slice().to_vec();
            (1..=64).find(|&w| {
                sequence::set_substrate_weight(w);
                sequence::build_via_substrate(t, 12, true, 3).as_slice().to_vec() != base
            })
        };

        let seed = test_tuple_oinf();
        assert_eq!(leader(&seed), crate::tokens::Token::Imscrib);
        assert_eq!(wc(&seed), Some(1));

        // F never reaches the contested top of the ranking, so it moves nothing.
        for f in [IgPrim::F_ell, IgPrim::F_eth, IgPrim::F_hbar] {
            let mut t = seed; t.f = f;
            assert_eq!(leader(&t), crate::tokens::Token::Imscrib, "F={:?}", f);
            assert_eq!(wc(&t), Some(1), "F={:?}", f);
        }

        // Step off the self-referential topology and the family matrix already
        // leads with the advance, leaving the substrate nothing to overturn.
        for tv in [IgPrim::T_boxtimes, IgPrim::T_net, IgPrim::T_bowtie, IgPrim::T_in] {
            let mut t = seed; t.t = tv;
            assert_eq!(leader(&t), crate::tokens::Token::Afwd, "T={:?}", tv);
            assert_eq!(wc(&t), None, "T={:?} moved under some weight", tv);
        }

        // Narrowing the interaction range does the same.
        for gv in [IgPrim::G_beth, IgPrim::G_gimel] {
            let mut t = seed; t.g = gv;
            assert_eq!(leader(&t), crate::tokens::Token::Afwd, "G={:?}", gv);
            assert_eq!(wc(&t), None, "G={:?} moved under some weight", gv);
        }
    }
}
