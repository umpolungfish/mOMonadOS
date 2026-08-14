// bifurcation_test.rs — what the substrate weight does to a composed word.
//
// This module varies `sequence::SUBSTRATE_WEIGHT` and measures what changes.
//
// The findings here were re-derived after 73537e4 ("imasm write composed every
// tuple to the same word") replaced the builder these tests were first written
// against. That commit removed `build_program_from_scores`, which recomputed a
// single argmax at all twelve positions, in favour of `slot_votes` +
// `build_program_from_slots`, which ranks each slot by its own axis's row. The
// earlier findings — that the zero-weight program is pure self-imscription,
// that cycle length is flat at 1 across the sweep, and that a w_c exists only
// where the family matrix leads with IMSCRIB — were all phenomenology OF the
// single-argmax builder, and none of them survives it. They are not restated
// as history in the assertions; what is asserted below is measured against the
// builder that exists.

#[cfg(test)]
mod bifurcation_tests {
    use crate::sequence;
    use crate::imas_ig::{IgTuple, IgPrim};
    use crate::tokens::Token;
    use alloc::vec::Vec;

    /// `sequence::SUBSTRATE_WEIGHT` is a process-wide `static mut`, so these
    /// tests cannot run concurrently — each would be reading a weight another
    /// had just overwritten. The lock serializes them.
    static WEIGHT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// A tuple at O_∞ tier: self-referential topology, universal range.
    fn test_tuple_oinf() -> IgTuple {
        IgTuple {
            d: IgPrim::if_,     t: IgPrim::are,   r: IgPrim::ian,   p: IgPrim::or_,
            f: IgPrim::peep,    k: IgPrim::egg,   g: IgPrim::ice,   c: IgPrim::measure,
            phi: IgPrim::monad, h: IgPrim::wool,  s: IgPrim::up,    omega: IgPrim::ah,
        }
    }

    /// `self_ref` is passed explicitly rather than derived from the tuple:
    /// the tests that vary T need the self-reference flag HELD while the
    /// topology value changes, or they would be measuring the flag and the
    /// axis at once and could not tell which moved the word.
    fn program_at(tuple: &IgTuple, weight: i32, self_ref: bool) -> Vec<Token> {
        sequence::set_substrate_weight(weight);
        sequence::build_via_substrate(tuple, 12, self_ref, 3).as_slice().to_vec()
    }

    /// How many distinct programs the self-imscription loop passes through
    /// before repeating one.
    fn measure_cycle(tuple: &IgTuple, weight: i32, max_iter: usize) -> usize {
        sequence::set_substrate_weight(weight);
        let mut seen: Vec<Vec<Token>> = Vec::new();
        let mut current = *tuple;
        let mut prog = sequence::build_via_substrate(&current, 12, current.t == IgPrim::are, 3);
        seen.push(prog.as_slice().to_vec());
        for i in 1..max_iter {
            let snap = crate::kernel::self_imscribe(&prog);
            current = IgTuple::from_snapshot(&snap);
            let next = sequence::build_via_substrate(&current, 12, current.t == IgPrim::are, 3);
            let tokens: Vec<Token> = next.as_slice().to_vec();
            if let Some(j) = seen.iter().position(|prev| prev == &tokens) {
                return i - j;
            }
            seen.push(tokens);
            prog = next;
        }
        max_iter
    }

    /// The loop closes at every weight, in one step or two — never longer.
    ///
    /// Cycle length is no longer flat: it is 2 at weights 1, 3, 9 and 10, and 1
    /// elsewhere. So the observable does move, but not monotonically and not
    /// with a threshold; a period-2 orbit and a fixed point sit interleaved
    /// along the sweep. What the sweep does establish is a ceiling — twenty
    /// iterations never find an orbit longer than two at any weight from 0 to
    /// 10.
    #[test]
    fn cycle_length_is_one_or_two_at_every_weight() {
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let measured: Vec<usize> =
            (0..=10).map(|w| measure_cycle(&test_tuple_oinf(), w, 20)).collect();
        assert_eq!(measured, alloc::vec![1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 2]);
        assert!(measured.iter().all(|&c| c == 1 || c == 2));
    }

    /// The zero-weight word is a twelve-mark word, not one mark twelve times.
    ///
    /// This is the property 73537e4 was for. With the substrate vote
    /// annihilated the ranking is the family matrix alone, and because each
    /// slot is now ranked by its own axis's row rather than by one global
    /// argmax, the twelve axes stay distinguishable: six distinct marks appear
    /// with no substrate contribution at all.
    #[test]
    fn zero_weight_word_is_not_uniform() {
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let at_0 = program_at(&test_tuple_oinf(), 0, true);
        assert_eq!(at_0.len(), 12);
        let mut distinct: Vec<Token> = Vec::new();
        for t in &at_0 {
            if !distinct.contains(t) { distinct.push(*t); }
        }
        assert_eq!(distinct.len(), 6, "w=0 word: {:?}", at_0);
        assert_eq!(at_0[0], Token::Imscrib);
        assert_eq!(*at_0.last().unwrap(), Token::Imscrib);
    }

    /// The weight reorders the word repeatedly, then saturates.
    ///
    /// There is no single w_c. Weights 0, 1, 2 and 3 each give a different
    /// word, and from 7 up the word stops changing — the substrate vote has
    /// taken every ranking it can take, and further weight cannot overturn
    /// anything that is left.
    #[test]
    fn weight_moves_the_word_then_saturates() {
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tuple = test_tuple_oinf();
        let progs: Vec<Vec<Token>> = (0..=10).map(|w| program_at(&tuple, w, true)).collect();

        for a in 0..4 {
            for b in (a + 1)..4 {
                assert_ne!(progs[a], progs[b], "w={} and w={} gave the same word", a, b);
            }
        }
        for w in 8..=10 {
            assert_eq!(progs[w], progs[7], "w={} left the saturated word", w);
        }
        assert_ne!(progs[7], progs[6], "saturation should begin at 7, not earlier");
    }

    /// The substrate vote moves every tuple tested, and the topology axis alone
    /// selects five different second marks.
    ///
    /// Under the single-argmax builder a w_c existed only where the family
    /// matrix led with IMSCRIB, which was a narrow place. Under per-slot
    /// ranking the vote reaches every tuple here: w_c = 1 for every T and every
    /// G value tried. The leader — the mark the family matrix alone puts after
    /// the opening IMSCRIB — now depends on the topology value, which is the
    /// axes being distinguishable rather than the substrate shouting louder.
    #[test]
    fn substrate_vote_reaches_every_tuple() {
        let _guard = WEIGHT_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        let leader = |t: &IgTuple| -> Token { program_at(t, 0, true)[1] };
        let wc = |t: &IgTuple| -> Option<i32> {
            let base = program_at(t, 0, true);
            (1..=64).find(|&w| program_at(t, w, true) != base)
        };

        let seed = test_tuple_oinf();
        assert_eq!(wc(&seed), Some(1));

        // Five topology values, five different second marks.
        let by_topology = [
            (IgPrim::are,   Token::Clink),
            (IgPrim::oil,   Token::Ifix),
            (IgPrim::judge, Token::Afwd),
            (IgPrim::mime,  Token::Engagr),
            (IgPrim::eat,   Token::Arev),
        ];
        let mut seen: Vec<Token> = Vec::new();
        for (tv, expected) in by_topology {
            let mut t = seed; t.t = tv;
            assert_eq!(leader(&t), expected, "T={:?}", tv);
            assert_eq!(wc(&t), Some(1), "T={:?} did not move under any weight", tv);
            assert!(!seen.contains(&expected), "T={:?} repeated a leader", tv);
            seen.push(expected);
        }
        assert_eq!(seen.len(), 5);

        // Granularity leaves the leader alone but the word still moves at w=1.
        for gv in [IgPrim::ice, IgPrim::bib, IgPrim::thigh] {
            let mut t = seed; t.g = gv;
            assert_eq!(leader(&t), Token::Clink, "G={:?}", gv);
            assert_eq!(wc(&t), Some(1), "G={:?} did not move under any weight", gv);
        }
    }
}
