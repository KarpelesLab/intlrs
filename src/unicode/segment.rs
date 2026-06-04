//! Text segmentation (UAX #29). Currently: extended grapheme clusters.
//!
//! ```
//! use intl::unicode::graphemes;
//!
//! // A base letter + combining mark is a single grapheme cluster.
//! let g: Vec<&str> = graphemes("a\u{0301}b!").collect();
//! assert_eq!(g, ["a\u{0301}", "b", "!"]);
//! ```
//!
//! (With the `full` tier, emoji ZWJ sequences and flag pairs are also single
//! clusters.)

use super::generated::segmentation as gen;

/// Grapheme_Cluster_Break value. (Variants are tier-conditionally constructed
/// in the generated table, so some are unused under a narrow range tier.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub(crate) enum Gcb {
    Other,
    CR,
    LF,
    Control,
    Extend,
    ZWJ,
    RegionalIndicator,
    Prepend,
    SpacingMark,
    L,
    V,
    T,
    LV,
    LVT,
}

/// Indic_Conjunct_Break value (for rule GB9c).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Incb {
    None,
    Consonant,
    Linker,
    Extend,
}

#[inline]
fn gcb(c: char) -> Gcb {
    gen::grapheme_break(c as u32)
}
#[inline]
fn pictographic(c: char) -> bool {
    gen::extended_pictographic(c as u32)
}
#[inline]
fn incb(c: char) -> Incb {
    gen::indic_conjunct_break(c as u32)
}

// State for the multi-character grapheme rules, tracking the run ending at the
// previous character.

/// GB11: `Extended_Pictographic Extend* ZWJ × Extended_Pictographic`.
#[derive(Clone, Copy, PartialEq)]
enum Emoji {
    None,
    Pictographic, // saw Extended_Pictographic, possibly followed by Extend*
    Zwj,          // ...followed by ZWJ
}

/// GB9c: `Consonant [Extend Linker]* Linker [Extend Linker]* × Consonant`.
#[derive(Clone, Copy, PartialEq)]
enum Conjunct {
    None,
    Consonant,  // saw a Consonant, no Linker yet
    LinkerSeen, // ...with at least one Linker since
}

#[derive(Clone, Copy)]
struct State {
    ri: u32,
    emoji: Emoji,
    conjunct: Conjunct,
}

impl State {
    fn start(c: char) -> Self {
        State {
            ri: u32::from(gcb(c) == Gcb::RegionalIndicator),
            emoji: if pictographic(c) {
                Emoji::Pictographic
            } else {
                Emoji::None
            },
            conjunct: if incb(c) == Incb::Consonant {
                Conjunct::Consonant
            } else {
                Conjunct::None
            },
        }
    }

    /// Fold the next consumed character into the state.
    fn advance(&mut self, c: char) {
        self.ri = if gcb(c) == Gcb::RegionalIndicator {
            self.ri + 1
        } else {
            0
        };

        self.emoji =
            if pictographic(c) || (gcb(c) == Gcb::Extend && self.emoji == Emoji::Pictographic) {
                Emoji::Pictographic
            } else if gcb(c) == Gcb::ZWJ && self.emoji == Emoji::Pictographic {
                Emoji::Zwj
            } else {
                Emoji::None
            };

        self.conjunct = match incb(c) {
            Incb::Consonant => Conjunct::Consonant,
            Incb::Linker if self.conjunct != Conjunct::None => Conjunct::LinkerSeen,
            Incb::Extend if self.conjunct != Conjunct::None => self.conjunct,
            _ => Conjunct::None,
        };
    }
}

/// Decide whether there is a grapheme break between `prev` and `cur`, given the
/// state of the run ending at `prev`.
fn is_break(prev: char, cur: char, st: &State) -> bool {
    let (l, r) = (gcb(prev), gcb(cur));
    use Gcb::*;
    // GB3 / GB4 / GB5: CR-LF and Control.
    if l == CR && r == LF {
        return false;
    }
    if matches!(l, Control | CR | LF) || matches!(r, Control | CR | LF) {
        return true;
    }
    // GB6 / GB7 / GB8: Hangul syllables.
    if l == L && matches!(r, L | V | LV | LVT) {
        return false;
    }
    if matches!(l, LV | V) && matches!(r, V | T) {
        return false;
    }
    if matches!(l, LVT | T) && r == T {
        return false;
    }
    // GB9 / GB9a / GB9b.
    if matches!(r, Extend | ZWJ) || r == SpacingMark || l == Prepend {
        return false;
    }
    // GB9c: Indic conjunct.
    if st.conjunct == Conjunct::LinkerSeen && incb(cur) == Incb::Consonant {
        return false;
    }
    // GB11: emoji ZWJ sequence.
    if st.emoji == Emoji::Zwj && pictographic(cur) {
        return false;
    }
    // GB12 / GB13: regional indicator pairs.
    if l == RegionalIndicator && r == RegionalIndicator && st.ri % 2 == 1 {
        return false;
    }
    // GB999.
    true
}

/// Iterator over the extended grapheme clusters of a string (UAX #29).
#[derive(Clone)]
pub struct Graphemes<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let rest = &self.s[self.pos..];
        let mut iter = rest.char_indices();
        let (_, first) = iter.next()?;
        let mut state = State::start(first);
        let mut prev = first;
        let mut len = rest.len();
        for (i, c) in iter {
            if is_break(prev, c, &state) {
                len = i;
                break;
            }
            state.advance(c);
            prev = c;
        }
        let cluster = &rest[..len];
        self.pos += len;
        Some(cluster)
    }
}

/// Iterate over the extended grapheme clusters of `s` (UAX #29).
#[must_use]
pub fn graphemes(s: &str) -> Graphemes<'_> {
    Graphemes { s, pos: 0 }
}
