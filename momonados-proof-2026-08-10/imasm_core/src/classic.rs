//! The classic 12-opcode Token — moved here from ask_native/src/imasm.rs so
//! every consumer derives its faces from ONE definition.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Token {
    // Slot order is the catalog's column order, and the two are the same
    // ordering. ⊙ stands at slot nine, Criticality: IMSCRIB is ⊙ because
    // imscribing is inclosure, a boundary drawn around its own centre, and a
    // critical point is where a system turns on itself. One glyph, one meaning,
    // whether read as primitive, as type, or as token.
    Vinit,
    Tanch,
    Afwd,
    Arev,
    Clink,
    Evalt,
    Fsplit,
    Ffuse,
    Imscrib,
    Evalf,
    Engagr,
    Ifix,
    Fsplit3,
    Ffuse3,
    Evali,
    Rotat,
}

impl Token {
    pub fn name(self) -> &'static str {
        match self {
            Token::Vinit => "VINIT",
            Token::Tanch => "TANCH",
            Token::Afwd => "AFWD",
            Token::Arev => "AREV",
            Token::Clink => "CLINK",
            Token::Imscrib => "IMSCRIB",
            Token::Fsplit => "FSPLIT",
            Token::Ffuse => "FFUSE",
            Token::Evalt => "EVALT",
            Token::Evalf => "EVALF",
            Token::Engagr => "ENGAGR",
            Token::Ifix => "IFIX",
            Token::Fsplit3 => "FSPLIT3",
            Token::Ffuse3 => "FFUSE3",
            Token::Evali => "EVALI",
            Token::Rotat => "ROTAT",
        }
    }

    /// Single-glyph code — one symbol per opcode, so an opcode word reads as a
    /// sequence (a codon string) instead of a space-delimited token list. The alphabet
    /// is not invented. Every token is SYMBOLIC — no Latin initials, and nothing
    /// that already means something else in mathematics: VINIT ⊢ and TANCH ⊣ are
    /// the opening and closing boundary turnstiles, EVALT ⊤ and EVALF ⊥ the two
    /// poles they evaluate, CLINK ⋈ the join, IFIX ⊡ the box that closes, ENGAGR
    /// ⊞ the Belnap Both it holds, and the dyad is ∈ / ∋, membership in and out.
    /// The set is also the twelve-primitive alphabet: one glyph per axis, the same
    /// twelve, because the opcodes and the axes were always the same twelve.
    /// Nothing outside the twelve parses. The earlier spellings ◇ ● = ═ + × ¬ ~ ≁,
    /// the letter codes V/T/B, and ← for IMSCRIB are not tokens and are aliased to
    /// nothing: a word containing one reads that mark as empty. Full names and the
    /// short forms VI/TA/EG/IM still do parse — they spell a live opcode, not a dead one.
    /// IMSCRIB is ⊙ for a reason, not by availability: imscribing is the very act of
    /// INCLOSURE — the monadic operation itself — hence self-referential, and so referenced
    /// self-referentially. The glyph is a boundary drawn around its own centre, denoting the
    /// act of denoting. Its appearance as Criticality in the 12-primitive notation is not a
    /// collision: it is the same structure surfacing wherever inclosure closes on itself.
    pub fn code(self) -> &'static str {
        match self {
            Token::Vinit => "⊢",
            Token::Tanch => "⊣",
            Token::Afwd => ">",
            Token::Arev => "<",
            Token::Clink => "⋈",
            Token::Imscrib => "⊙",
            Token::Fsplit => "∈",
            Token::Ffuse => "∋",
            Token::Evalt => "⊤",
            Token::Evalf => "⊥",
            Token::Engagr => "⊞",
            Token::Ifix => "⊡",
            Token::Fsplit3 => "∈",
            Token::Ffuse3 => "∋",
            Token::Evali => "⊞",
            Token::Rotat => "↻",
        }
    }

    /// Does this opcode TRANSFORM the object (do work), as opposed to being an
    /// identity/structural node? IMSCRIB is the identity morphism, VINIT/TANCH are
    /// source/sink, FSPLIT/FFUSE are the split/fuse themselves — none transform.
    /// A μ∘δ closure whose arms carry only these is identity, not a type-check.
    pub fn transforms(self) -> bool {
        matches!(
            self,
            Token::Afwd | Token::Arev | Token::Clink | Token::Evalt | Token::Evalf
                | Token::Engagr | Token::Ifix | Token::Evali
                | Token::Rotat
        )
    }

    /// Is this the fork (δ)? Both spellings are the same operator.
    ///
    /// FSPLIT cuts the register along the truth axis, {T,t} | {F,f}. FSPLIT3 cuts
    /// it into {T} | {F} | {t,f}. Both are partitions of the four base values, so
    /// μ∘δ = id holds at either arity, and on the classical slice, where t and f
    /// are absent, the second is the first: the information arm carries nothing
    /// and {T} | {F} | {} is the truth cut. One operator on two carriers, the way
    /// FOUR is SIXTEEN_3's classical slice, so one ancestry rule and one close
    /// condition answer for both.
    pub fn is_brancher(self) -> bool {
        matches!(self, Token::Fsplit | Token::Fsplit3)
    }

    /// Is this the fuse (μ)? Both spellings join by union over however many arms
    /// arrive, which is why the fuse side never needed two operators at all.
    pub fn is_merger(self) -> bool {
        matches!(self, Token::Ffuse | Token::Ffuse3)
    }

    /// (arity_in, arity_out) — the max ports the opcode may carry.
    ///
    /// The dyad carries three, because there is one dyad and the carrier decides
    /// how many arms of it are visible: two on the classical slice, three when
    /// the information layer is in play. A fan of two is not a different opcode,
    /// it is this one with an empty arm, so both are inside the arity and only a
    /// fourth arm is an over-valence.
    pub fn arity(self) -> (usize, usize) {
        match self {
            Token::Vinit => (0, 1),
            Token::Fsplit | Token::Fsplit3 => (1, 3),
            Token::Ffuse | Token::Ffuse3 => (3, 1),
            _ => (1, 1),
        }
    }

    /// The full opcode table as JSON, for the export manifest: name, glyph,
    /// and arity, so the composer surface renders ports without re-deriving them.
    #[cfg(feature = "std")]
    pub fn parse_all_names() -> serde_json::Value {
        let all = [
            Token::Vinit, Token::Tanch, Token::Afwd, Token::Arev, Token::Clink,
            Token::Imscrib, Token::Fsplit, Token::Ffuse, Token::Evalt,
            Token::Evalf, Token::Engagr, Token::Ifix,
            Token::Fsplit3, Token::Ffuse3, Token::Evali,
            Token::Rotat,
        ];
        serde_json::Value::Array(
            all.iter()
                .map(|t| {
                    let (ain, aout) = t.arity();
                    serde_json::json!({
                        "name": t.name(), "code": t.code(),
                        "arity_in": ain, "arity_out": aout,
                    })
                })
                .collect(),
        )
    }

    /// Accepts full names (VINIT) and the IMSCRIBr short forms (VI, FS, FF, …),
    /// case-insensitively.
    pub fn parse(s: &str) -> Option<Token> {
        let u = s.trim().to_ascii_uppercase();
        Some(match u.as_str() {
            "VINIT" | "VI" | "⊢" => Token::Vinit,
            "TANCH" | "TA" | "⊣" => Token::Tanch,
            "AFWD" | "AF" | ">" => Token::Afwd,
            "AREV" | "AR" | "<" => Token::Arev,
            "CLINK" | "CL" | "⋈" => Token::Clink,
            "IMSCRIB" | "IMSCRIBE" | "IM" | "⊙" => Token::Imscrib,
            // ∈ and ∋ are the dyad. The Greek δ/μ name the fork and fuse; ∈ ∋ are
            // their glyphs. The old marks ◇ ● ☊ ☋ are NOT tokens and do not parse.
            "FSPLIT" | "FS" | "SPLIT" | "DELTA" | "∈" | "δ" => Token::Fsplit,
            "FFUSE" | "FF" | "FUSE" | "MU" | "∋" | "μ" => Token::Ffuse,
            "EVALT" | "ET" | "⊤" => Token::Evalt,
            "EVALF" | "EF" | "⊥" => Token::Evalf,
            "ENGAGR" | "EG" | "⊞" => Token::Engagr,
            "IFIX" | "IX" | "FIX" | "⊡" => Token::Ifix,
            "FSPLIT3" | "Fsplit3" | "F3" => Token::Fsplit3,
            "FFUSE3" | "Ffuse3" | "FF3" => Token::Ffuse3,
            "EVALI" => Token::Evali,
            "ROTAT" | "RT" | "↻" | "↺" => Token::Rotat,
            _ => return None,
        })
    }
}

/// The four families of the 12-opcode tower (mOMonadOS kernel classification).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Family {
    Logical,
    Frobenius,
    Dialetheia,
    Linear,
}

impl Token {
    pub fn family(self) -> Family {
        match self {
            Token::Vinit | Token::Tanch | Token::Afwd |
            Token::Arev | Token::Clink | Token::Imscrib | Token::Rotat => Family::Logical,
            Token::Fsplit | Token::Ffuse | Token::Fsplit3 | Token::Ffuse3 => Family::Frobenius,
            Token::Evalt | Token::Evalf | Token::Engagr | Token::Evali => Family::Dialetheia,
            Token::Ifix => Family::Linear,
        }
    }

    /// Input arity as the stack machine reads it (mOMonadOS kernel form).
    pub fn arity_in(self) -> u8 {
        match self {
            Token::Vinit => 0,
            Token::Ffuse => 2,
            Token::Ffuse3 => 3,
            _ => 1,
        }
    }

    /// Output arity as the stack machine reads it.
    pub fn arity_out(self) -> u8 {
        match self {
            Token::Tanch => 0,
            Token::Fsplit => 2,
            Token::Fsplit3 => 3,
            _ => 1,
        }
    }
}
