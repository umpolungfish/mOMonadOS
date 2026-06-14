#![allow(dead_code)]
// aleph.rs — Hebrew Letter → IG Primitive Mapping
//
// Bridges ALEPH_OS aleph encoding into the mOMonadOS kernel.
// 22 Hebrew letters map to structural positions in the IG primitive lattice.
// The mapping follows the Sefer Yetzirah threefold division:
//   3 Mothers (Aleph, Mem, Shin)  → Dimensionality ⊗ Criticality
//   7 Doubles (Bet, Gimel, Dalet, Kaf, Pe, Resh, Tav) → Topology ⊗ Winding
//   12 Simples (He, Vav, Zayin, Chet, Tet, Yod, Lamed, Nun, Samekh, Ayin, Tzadi, Qof) → Coupling ⊗ Chirality

use crate::imas_ig::IgPrim;

/// Hebrew letter with its name and IG primitive mapping.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AlephLetter {
    pub glyph: &'static str,  // Hebrew glyph as UTF-8
    pub name:  &'static str,  // Letter name
    pub prim:  IgPrim,        // Primary IG primitive
    pub family: AlephFamily,  // Sefer Yetzirah family
    pub value: u16,            // Gematria value
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AlephFamily { Mother, Double, Simple }

/// All 22 Hebrew letters with IG primitive correspondences.
pub const ALEPH_LETTERS: [AlephLetter; 22] = [
    // 3 Mothers
    AlephLetter { glyph: "א", name: "Aleph",  prim: IgPrim::D_odot,   family: AlephFamily::Mother, value: 1 },
    AlephLetter { glyph: "מ", name: "Mem",    prim: IgPrim::Phi_c,    family: AlephFamily::Mother, value: 40 },
    AlephLetter { glyph: "ש", name: "Shin",   prim: IgPrim::Phi_ep,   family: AlephFamily::Mother, value: 300 },
    // 7 Doubles
    AlephLetter { glyph: "ב", name: "Bet",    prim: IgPrim::T_in,     family: AlephFamily::Double, value: 2 },
    AlephLetter { glyph: "ג", name: "Gimel",  prim: IgPrim::T_net,    family: AlephFamily::Double, value: 3 },
    AlephLetter { glyph: "ד", name: "Dalet",  prim: IgPrim::T_bowtie, family: AlephFamily::Double, value: 4 },
    AlephLetter { glyph: "כ", name: "Kaf",    prim: IgPrim::Omega_z,  family: AlephFamily::Double, value: 20 },
    AlephLetter { glyph: "פ", name: "Pe",     prim: IgPrim::Omega_z2, family: AlephFamily::Double, value: 80 },
    AlephLetter { glyph: "ר", name: "Resh",   prim: IgPrim::T_boxtimes, family: AlephFamily::Double, value: 200 },
    AlephLetter { glyph: "ת", name: "Tav",    prim: IgPrim::Omega_0,  family: AlephFamily::Double, value: 400 },
    // 12 Simples
    AlephLetter { glyph: "ה", name: "He",     prim: IgPrim::R_lr,     family: AlephFamily::Simple, value: 5 },
    AlephLetter { glyph: "ו", name: "Vav",    prim: IgPrim::C_seq,    family: AlephFamily::Simple, value: 6 },
    AlephLetter { glyph: "ז", name: "Zayin",  prim: IgPrim::H1,       family: AlephFamily::Simple, value: 7 },
    AlephLetter { glyph: "ח", name: "Chet",   prim: IgPrim::H2,       family: AlephFamily::Simple, value: 8 },
    AlephLetter { glyph: "ט", name: "Tet",    prim: IgPrim::R_dagger, family: AlephFamily::Simple, value: 9 },
    AlephLetter { glyph: "י", name: "Yod",    prim: IgPrim::C_and,    family: AlephFamily::Simple, value: 10 },
    AlephLetter { glyph: "ל", name: "Lamed",  prim: IgPrim::R_super,  family: AlephFamily::Simple, value: 30 },
    AlephLetter { glyph: "נ", name: "Nun",    prim: IgPrim::H_inf,    family: AlephFamily::Simple, value: 50 },
    AlephLetter { glyph: "ס", name: "Samekh", prim: IgPrim::C_or,     family: AlephFamily::Simple, value: 60 },
    AlephLetter { glyph: "ע", name: "Ayin",   prim: IgPrim::C_broad,  family: AlephFamily::Simple, value: 70 },
    AlephLetter { glyph: "צ", name: "Tzadi",  prim: IgPrim::R_cat,    family: AlephFamily::Simple, value: 90 },
    AlephLetter { glyph: "ק", name: "Qof",    prim: IgPrim::H0,       family: AlephFamily::Simple, value: 100 },
];

impl AlephLetter {
    /// Look up a letter by its Hebrew glyph (UTF-8).
    pub fn from_glyph(g: &str) -> Option<AlephLetter> {
        ALEPH_LETTERS.iter().find(|l| l.glyph == g).copied()
    }

    /// Look up a letter by its name (case-insensitive prefix match).
    pub fn from_name(name: &str) -> Option<AlephLetter> {
        let n = name.to_lowercase();
        ALEPH_LETTERS.iter().find(|l| l.name.to_lowercase().starts_with(&n)).copied()
    }

    /// Gematria sum of letters in a word. Returns 0 for unrecognized glyphs.
    pub fn gematria(word: &str) -> u32 {
        word.chars().filter_map(|c| {
            let mut buf = [0u8; 4];
            let s = c.encode_utf8(&mut buf);
            ALEPH_LETTERS.iter().find(|l| l.glyph == s).map(|l| l.value as u32)
        }).sum()
    }
}

/// Aleph-encoded word: a sequence of IG primitives derived from Hebrew letters.
pub struct AlephWord {
    pub text:    &'static str,
    pub letters: [Option<AlephLetter>; 12], // max 12 letters
    pub count:   usize,
}

impl AlephWord {
    /// Encode a word into its aleph structure.
    /// Unknown glyphs are skipped. Max 12 letters stored.
    pub fn encode(text: &str) -> Self {
        let mut letters = [None; 12];
        let mut count = 0;
        for c in text.chars() {
            let mut buf = [0u8; 4];
            let s = c.encode_utf8(&mut buf);
            if let Some(l) = AlephLetter::from_glyph(&s) {
                if count < 12 {
                    letters[count] = Some(l);
                    count += 1;
                }
            }
        }
        AlephWord { text: "", letters, count }
    }

    /// Distance between two aleph words: count of differing letter→primitive mappings.
    pub fn distance(&self, other: &AlephWord) -> usize {
        let n = self.count.min(other.count);
        if n == 0 { return 12; }
        let mut d = (self.count as isize - other.count as isize).unsigned_abs();
        for i in 0..n {
            let a = self.letters[i].map(|l| l.prim as u8);
            let b = other.letters[i].map(|l| l.prim as u8);
            if a != b { d += 1; }
        }
        d
    }

    /// Get the IG primitives for this word as a compact array.
    pub fn primitives(&self) -> [Option<IgPrim>; 12] {
        let mut prims = [None; 12];
        for i in 0..self.count {
            prims[i] = self.letters[i].map(|l| l.prim);
        }
        prims
    }
}
