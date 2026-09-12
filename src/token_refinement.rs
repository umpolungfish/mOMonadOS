//! Lazy IMASM-on-IMASM refinement.
//!
//! A surface token is an interface to another complete IMASM word.  Refining
//! one occurrence replaces no semantic state on the parent level: the child
//! word remains behind that occurrence's interface and collapses to the same
//! parent token.  Since children are ordinary `Token`s, any child occurrence
//! can be refined again.  The representation is lazy, so depth is made by the
//! selected path instead of materialising the whole exponential tree.

use alloc::{format, string::{String, ToString}, vec::Vec};
use imasm_core::{check::{from_sequence, match_pairs}, classic::Token};

/// One closed word containing every member of the twelve-token alphabet.
/// It is the common internal vocabulary of every morphism, including itself.
pub const UNIVERSAL_REFINEMENT: &str = "⊢⊙∈≻⊤≺⊥⋈⊞∋⊡⊣";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Morphism {
    surface: Token,
    interior: Option<Vec<Morphism>>,
}

impl Morphism {
    pub fn atom(surface: Token) -> Self { Self { surface, interior: None } }
    pub fn surface(&self) -> Token { self.surface }
    pub fn is_refined(&self) -> bool { self.interior.is_some() }
    pub fn interior(&self) -> Option<&[Morphism]> { self.interior.as_deref() }

    /// Zoom into one occurrence. `path=[]` selects this occurrence;
    /// subsequent indices select tokens inside already-open interiors.
    pub fn zoom(&mut self, path: &[usize]) -> Result<(), String> {
        if let Some((&head, tail)) = path.split_first() {
            let children = self.interior.as_mut()
                .ok_or_else(|| "cannot descend through an unrefined token".to_string())?;
            let child = children.get_mut(head)
                .ok_or_else(|| format!("refinement child {head} is out of range"))?;
            return child.zoom(tail);
        }
        if self.interior.is_none() {
            self.interior = Some(refinement_tokens()?.into_iter().map(Morphism::atom).collect());
        }
        Ok(())
    }

    /// Forget the chosen finite view. The outer morphism is exactly preserved.
    pub fn collapse(&mut self) { self.interior = None; }

    /// The currently visible frontier, in execution order.
    pub fn leaves(&self, out: &mut Vec<Token>) {
        match &self.interior {
            Some(children) => for child in children { child.leaves(out); },
            None => out.push(self.surface),
        }
    }

    pub fn max_depth(&self) -> usize {
        self.interior.as_ref().map_or(0, |children|
            1 + children.iter().map(Morphism::max_depth).max().unwrap_or(0))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefinementWord { roots: Vec<Morphism> }

impl RefinementWord {
    pub fn parse(word: &str) -> Result<Self, String> {
        let roots: Vec<Morphism> = parse_tokens(word)?.into_iter().map(Morphism::atom).collect();
        if roots.is_empty() { return Err("an IMASM word cannot be empty".into()); }
        Ok(Self { roots })
    }
    pub fn roots(&self) -> &[Morphism] { &self.roots }
    pub fn zoom(&mut self, root: usize, path: &[usize]) -> Result<(), String> {
        self.roots.get_mut(root).ok_or_else(|| format!("root token {root} is out of range"))?.zoom(path)
    }
    pub fn collapse(&mut self, root: usize) -> Result<(), String> {
        let node = self.roots.get_mut(root).ok_or_else(|| format!("root token {root} is out of range"))?;
        node.collapse(); Ok(())
    }
    pub fn surface_word(&self) -> String { self.roots.iter().map(|n| n.surface.code()).collect() }
    pub fn visible_word(&self) -> String {
        let mut leaves=Vec::new();
        for root in &self.roots { root.leaves(&mut leaves); }
        leaves.iter().map(|t|t.code()).collect()
    }
    pub fn max_depth(&self) -> usize {
        self.roots.iter().map(Morphism::max_depth).max().unwrap_or(0)
    }
}

fn parse_tokens(word: &str) -> Result<Vec<Token>, String> {
    word.chars().filter(|c|!c.is_whitespace()).map(|c|
        Token::parse(&c.to_string()).ok_or_else(||format!("unknown IMASM mark {c}"))).collect()
}

fn refinement_tokens() -> Result<Vec<Token>, String> {
    let ops=parse_tokens(UNIVERSAL_REFINEMENT)?;
    let graph=from_sequence(&ops,&match_pairs(&ops));
    if !graph.validate().is_empty() || !graph.frobenius_closures().1 {
        return Err("universal refinement is not a closed IMASM composition".into());
    }
    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_token_opens_onto_the_whole_imasm_alphabet() {
        let alphabet="⊢⊣≻≺⋈⊙∈∋⊤⊥⊞⊡";
        for (i, expected) in alphabet.chars().enumerate() {
            let mut word=RefinementWord::parse(alphabet).unwrap();
            word.zoom(i,&[]).unwrap();
            let inside=word.roots()[i].interior().unwrap();
            assert!(inside.iter().any(|m|m.surface().code()==expected.to_string()));
            for required in alphabet.chars() {
                assert!(inside.iter().any(|m|m.surface().code()==required.to_string()));
            }
        }
    }

    #[test]
    fn zoom_makes_only_the_selected_time_and_can_continue_without_a_bound() {
        let mut word=RefinementWord::parse("⊤").unwrap();
        word.zoom(0,&[]).unwrap();
        word.zoom(0,&[4]).unwrap();       // ⊤ inside ⊤
        word.zoom(0,&[4,4]).unwrap();     // ⊤ inside ⊤ inside ⊤
        assert_eq!(word.max_depth(),3);
        assert_eq!(word.surface_word(),"⊤");
        assert_eq!(word.visible_word().chars().count(),34); // 1+3*(12-1)
    }

    #[test]
    fn collapse_recovers_the_exact_parent_morphism() {
        let original="⊢∈⊤≻⊥≺∋⊡⊣";
        let mut word=RefinementWord::parse(original).unwrap();
        word.zoom(3,&[]).unwrap();
        word.zoom(3,&[8]).unwrap();
        assert_ne!(word.visible_word(),original);
        word.collapse(3).unwrap();
        assert_eq!(word.visible_word(),original);
        assert_eq!(word.surface_word(),original);
    }
}
