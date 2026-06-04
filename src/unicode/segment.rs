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

/// Word_Break value (UAX #29). Order must match the generated table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub(crate) enum Wb {
    Other,
    CR,
    LF,
    Newline,
    Extend,
    ZWJ,
    RegionalIndicator,
    Format,
    Katakana,
    HebrewLetter,
    ALetter,
    SingleQuote,
    DoubleQuote,
    MidNumLet,
    MidLetter,
    MidNum,
    Numeric,
    ExtendNumLet,
    WSegSpace,
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

// ---- Word boundaries (UAX #29) ----

#[inline]
fn wb(c: char) -> Wb {
    gen::word_break(c as u32)
}

/// An "effective" word-break unit: a base character plus any trailing
/// Extend/Format/ZWJ that rule WB4 folds into it.
#[derive(Clone, Copy)]
struct WbUnit {
    cat: Wb,
    pictographic: bool, // base is Extended_Pictographic (for WB3c)
    ends_zwj: bool,     // last character of the unit is ZWJ (for WB3c)
    bare: bool,         // the unit absorbed no Extend/Format/ZWJ (for WB3d)
    end: usize,         // byte index just past the unit
}

/// Read the effective word-break unit starting at byte index `i`.
fn wb_unit(s: &str, i: usize) -> WbUnit {
    let base = s[i..].chars().next().unwrap();
    let cat = wb(base);
    let base_end = i + base.len_utf8();
    let mut end = base_end;
    let mut ends_zwj = cat == Wb::ZWJ;
    // WB4: fold trailing Extend/Format/ZWJ into the unit — but not after a
    // mandatory break (CR/LF/Newline absorb nothing).
    if !matches!(cat, Wb::CR | Wb::LF | Wb::Newline) {
        for c in s[end..].chars() {
            match wb(c) {
                t @ (Wb::Extend | Wb::Format | Wb::ZWJ) => {
                    ends_zwj = t == Wb::ZWJ;
                    end += c.len_utf8();
                }
                _ => break,
            }
        }
    }
    WbUnit {
        cat,
        pictographic: pictographic(base),
        ends_zwj,
        bare: end == base_end,
        end,
    }
}

#[inline]
fn ah(w: Wb) -> bool {
    matches!(w, Wb::ALetter | Wb::HebrewLetter)
}

/// Decide whether there is a word break before `cur`, given the two preceding
/// effective categories, the unit after `cur` (`next`), and RI parity.
#[allow(clippy::too_many_arguments)]
fn word_break(prev2: Wb, prev: &WbUnit, cur: &WbUnit, next: Wb, ri: u32) -> bool {
    use Wb::*;
    let (p, c) = (prev.cat, cur.cat);
    if p == CR && c == LF {
        return false; // WB3
    }
    if matches!(p, Newline | CR | LF) || matches!(c, Newline | CR | LF) {
        return true; // WB3a / WB3b
    }
    if prev.ends_zwj && cur.pictographic {
        return false; // WB3c
    }
    if p == WSegSpace && c == WSegSpace && prev.bare {
        return false; // WB3d (literal adjacency; pre-WB4)
    }
    if matches!(c, Extend | Format | ZWJ) {
        return false; // WB4: Any × (Format | Extend | ZWJ)
    }
    if ah(p) && ah(c) {
        return false; // WB5
    }
    if ah(p) && matches!(c, MidLetter | MidNumLet | SingleQuote) && ah(next) {
        return false; // WB6
    }
    if ah(prev2) && matches!(p, MidLetter | MidNumLet | SingleQuote) && ah(c) {
        return false; // WB7
    }
    if p == HebrewLetter && c == SingleQuote {
        return false; // WB7a
    }
    if p == HebrewLetter && c == DoubleQuote && next == HebrewLetter {
        return false; // WB7b
    }
    if prev2 == HebrewLetter && p == DoubleQuote && c == HebrewLetter {
        return false; // WB7c
    }
    if p == Numeric && c == Numeric {
        return false; // WB8
    }
    if ah(p) && c == Numeric {
        return false; // WB9
    }
    if p == Numeric && ah(c) {
        return false; // WB10
    }
    if prev2 == Numeric && matches!(p, MidNum | MidNumLet | SingleQuote) && c == Numeric {
        return false; // WB11
    }
    if p == Numeric && matches!(c, MidNum | MidNumLet | SingleQuote) && next == Numeric {
        return false; // WB12
    }
    if p == Katakana && c == Katakana {
        return false; // WB13
    }
    if matches!(
        p,
        ALetter | HebrewLetter | Numeric | Katakana | ExtendNumLet
    ) && c == ExtendNumLet
    {
        return false; // WB13a
    }
    if p == ExtendNumLet && matches!(c, ALetter | HebrewLetter | Numeric | Katakana) {
        return false; // WB13b
    }
    if p == RegionalIndicator && c == RegionalIndicator && ri % 2 == 1 {
        return false; // WB15 / WB16
    }
    true // WB999
}

/// Iterator over the words (UAX #29 word-boundary spans) of a string.
#[derive(Clone)]
pub struct Words<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.pos >= self.s.len() {
            return None;
        }
        let start = self.pos;
        let mut prev2 = Wb::Other; // sot
        let mut prev = wb_unit(self.s, start);
        let mut ri = u32::from(prev.cat == Wb::RegionalIndicator);
        let mut at = prev.end;
        while at < self.s.len() {
            let cur = wb_unit(self.s, at);
            let next = if cur.end < self.s.len() {
                wb_unit(self.s, cur.end).cat
            } else {
                Wb::Other
            };
            if word_break(prev2, &prev, &cur, next, ri) {
                break;
            }
            ri = if cur.cat == Wb::RegionalIndicator {
                ri + 1
            } else {
                0
            };
            prev2 = prev.cat;
            prev = cur;
            at = cur.end;
        }
        let word = &self.s[start..at];
        self.pos = at;
        Some(word)
    }
}

/// Iterate over the word-boundary spans of `s` (UAX #29). Spans include
/// whitespace and punctuation runs, not just "letters"; filter with e.g.
/// [`char::is_alphanumeric`] for word-like tokens.
#[must_use]
pub fn words(s: &str) -> Words<'_> {
    Words { s, pos: 0 }
}
