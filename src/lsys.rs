/// Thanks to [mneumann](https://github.com/mneumann/lindenmayer-system) for the implementation of L-Systems.

use std::fmt::Debug;

/// Used to name symbols and variables.
pub trait Alphabet: Debug + PartialEq + Eq + Clone {}

impl Alphabet for &'static str {}
impl Alphabet for char {}
impl Alphabet for u8 {}
impl Alphabet for u16 {}
impl Alphabet for u32 {}
impl Alphabet for u64 {}
impl Alphabet for usize {}

/// An alphabet that distinguishes between terminal
/// and non-terminal symbols.
pub trait DualAlphabet: Alphabet {
    type Terminal;
    type NonTerminal: PartialOrd + Ord + Clone;

    /// If the character is a non-terminal, return Some(..). Otherwise return None.
    fn nonterminal(&self) -> Option<&Self::NonTerminal>;

    /// If the character is a terminal, return Some(..). Otherwise return None.
    fn terminal(&self) -> Option<&Self::Terminal>;
}
