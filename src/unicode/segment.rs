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

/// Sentence_Break value (UAX #29). Order must match the generated table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub(crate) enum Sb {
    Other,
    CR,
    LF,
    Extend,
    Sep,
    Format,
    Sp,
    Lower,
    Upper,
    OLetter,
    Numeric,
    ATerm,
    SContinue,
    STerm,
    Close,
}

/// Line_Break value (UAX #14), with LB1 resolution already applied by codegen
/// (so AI/SG/XX/SA/CJ never appear at runtime). Order matches the generated
/// table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub(crate) enum Lb {
    AI,
    AK,
    AL,
    AP,
    AS,
    B2,
    BA,
    BB,
    BK,
    CB,
    CJ,
    CL,
    CM,
    CP,
    CR,
    EB,
    EM,
    EX,
    GL,
    H2,
    H3,
    HH,
    HL,
    HY,
    ID,
    IN,
    IS,
    JL,
    JT,
    JV,
    LF,
    NL,
    NS,
    NU,
    OP,
    PO,
    PR,
    QU,
    RI,
    SA,
    SG,
    SP,
    SY,
    VF,
    VI,
    WJ,
    XX,
    ZW,
    ZWJ,
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

// ---- Sentence boundaries (UAX #29) ----

#[inline]
fn sb(c: char) -> Sb {
    gen::sentence_break(c as u32)
}

/// An effective sentence-break unit: a base character plus any trailing
/// Extend/Format that rule SB5 folds into it.
fn sb_unit(s: &str, i: usize) -> (Sb, usize) {
    let base = s[i..].chars().next().unwrap();
    let cat = sb(base);
    let mut end = i + base.len_utf8();
    if !matches!(cat, Sb::CR | Sb::LF | Sb::Sep) {
        for c in s[end..].chars() {
            match sb(c) {
                Sb::Extend | Sb::Format => end += c.len_utf8(),
                _ => break,
            }
        }
    }
    (cat, end)
}

/// Position within an `(STerm | ATerm) Close* Sp*` terminator sequence.
#[derive(Clone, Copy, PartialEq)]
enum Term {
    None,
    A,
    AClose,
    ASp,
    S,
    SClose,
    SSp,
}

fn term_next(t: Term, c: Sb) -> Term {
    match c {
        Sb::ATerm => Term::A,
        Sb::STerm => Term::S,
        Sb::Close => match t {
            Term::A | Term::AClose => Term::AClose,
            Term::S | Term::SClose => Term::SClose,
            _ => Term::None,
        },
        Sb::Sp => match t {
            Term::A | Term::AClose | Term::ASp => Term::ASp,
            Term::S | Term::SClose | Term::SSp => Term::SSp,
            _ => Term::None,
        },
        _ => Term::None,
    }
}

#[inline]
fn in_aterm_seq(t: Term) -> bool {
    matches!(t, Term::A | Term::AClose | Term::ASp)
}
#[inline]
fn in_term_seq(t: Term) -> bool {
    !matches!(t, Term::None)
}
#[inline]
fn in_close_phase(t: Term) -> bool {
    matches!(t, Term::A | Term::AClose | Term::S | Term::SClose)
}

/// SB8 lookahead: starting at byte `at`, is the first
/// non-`(OLetter|Upper|Lower|Sep|CR|LF|STerm|ATerm)` "stopper" a `Lower`?
fn sb8_lower_ahead(s: &str, mut at: usize) -> bool {
    while at < s.len() {
        let (cat, end) = sb_unit(s, at);
        match cat {
            Sb::Lower => return true,
            Sb::OLetter | Sb::Upper | Sb::Sep | Sb::CR | Sb::LF | Sb::STerm | Sb::ATerm => {
                return false
            }
            _ => {}
        }
        at = end;
    }
    false
}

/// Decide whether there is a sentence break before `cur`.
fn sentence_break(prev2: Sb, prev: Sb, term: Term, cur: Sb, lower_ahead: bool) -> bool {
    use Sb::*;
    if prev == CR && cur == LF {
        return false; // SB3
    }
    if matches!(prev, Sep | CR | LF) {
        return true; // SB4
    }
    if prev == ATerm && cur == Numeric {
        return false; // SB6
    }
    if matches!(prev2, Upper | Lower) && prev == ATerm && cur == Upper {
        return false; // SB7
    }
    if in_aterm_seq(term) && lower_ahead {
        return false; // SB8
    }
    if in_term_seq(term) && matches!(cur, SContinue | STerm | ATerm) {
        return false; // SB8a
    }
    if in_close_phase(term) && matches!(cur, Close | Sp | Sep | CR | LF) {
        return false; // SB9
    }
    if in_term_seq(term) && matches!(cur, Sp | Sep | CR | LF) {
        return false; // SB10
    }
    if in_term_seq(term) {
        return true; // SB11
    }
    false // SB998
}

/// Iterator over the sentences (UAX #29 sentence-boundary spans) of a string.
#[derive(Clone)]
pub struct Sentences<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Iterator for Sentences<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.pos >= self.s.len() {
            return None;
        }
        let start = self.pos;
        let (mut prev, mut at) = sb_unit(self.s, start);
        let mut prev2 = Sb::Other; // sot
        let mut term = term_next(Term::None, prev);
        while at < self.s.len() {
            let (cur, end) = sb_unit(self.s, at);
            let lower_ahead = in_aterm_seq(term) && sb8_lower_ahead(self.s, at);
            if sentence_break(prev2, prev, term, cur, lower_ahead) {
                break;
            }
            term = term_next(term, cur);
            prev2 = prev;
            prev = cur;
            at = end;
        }
        let sentence = &self.s[start..at];
        self.pos = at;
        Some(sentence)
    }
}

/// Iterate over the sentence-boundary spans of `s` (UAX #29).
#[must_use]
pub fn sentences(s: &str) -> Sentences<'_> {
    Sentences { s, pos: 0 }
}

// ---- Line breaking (UAX #14) ----

#[inline]
fn lb(c: char) -> Lb {
    gen::line_break(c as u32)
}

/// An effective line-break unit: a base plus trailing CM/ZWJ (rule LB9).
struct LbUnit {
    cls: Lb,
    base: char,
    ends_zwj: bool,
    end: usize,
}

impl LbUnit {
    /// East_Asian_Width F/W/H (for the LB30/LB19a exceptions).
    fn wide(&self) -> bool {
        use super::width::EastAsianWidth::*;
        matches!(
            super::width::east_asian_width(self.base),
            Fullwidth | Wide | Halfwidth
        )
    }
    fn pi(&self) -> bool {
        super::generated::general_category::general_category(self.base as u32)
            == super::category::GeneralCategory::InitialPunctuation
    }
    fn pf(&self) -> bool {
        super::generated::general_category::general_category(self.base as u32)
            == super::category::GeneralCategory::FinalPunctuation
    }
}

fn lb_unit(s: &str, i: usize) -> LbUnit {
    use Lb::*;
    let base = s[i..].chars().next().unwrap();
    let raw = lb(base);
    // LB10: a standalone CM/ZWJ becomes AL.
    let cls = if matches!(raw, CM | ZWJ) { AL } else { raw };
    let mut end = i + base.len_utf8();
    let mut ends_zwj = raw == ZWJ;
    // LB9: X (CM | ZWJ)* → X, where X ∉ {BK,CR,LF,NL,SP,ZW}.
    if !matches!(raw, BK | CR | LF | NL | SP | ZW) {
        for c in s[end..].chars() {
            match lb(c) {
                t @ (CM | ZWJ) => {
                    ends_zwj = t == ZWJ;
                    end += c.len_utf8();
                }
                _ => break,
            }
        }
    }
    LbUnit {
        cls,
        base,
        ends_zwj,
        end,
    }
}

/// Carried state for the line-break rules.
struct LbState {
    regional: Lb, // class of the last non-SP unit (left operand for "X SP* × Y")
    r_pi: bool,   // the regional unit is an opening (Pi) quotation
    before: Lb,   // class of the unit before `regional` (LB15a context); XX = sot
    sp: bool,     // ≥1 SP seen since `regional`
    open_ri: u32, // regional-indicator parity (LB30a)
    num: bool,    // inside an LB25 number run `NU (NU|SY|IS)* (CL|CP)?`
}

#[inline]
fn al_hl(c: Lb) -> bool {
    matches!(c, Lb::AL | Lb::HL)
}

/// Whether a line break is allowed before `cur`, plus whether it is mandatory.
fn line_break_before(
    prev2: Option<&LbUnit>,
    prev: &LbUnit,
    cur: &LbUnit,
    next: Option<&LbUnit>,
    st: &LbState,
) -> (bool, bool) {
    use Lb::*;
    let p = prev.cls;
    let c = cur.cls;
    let l = st.regional;

    if p == BK || p == LF || p == NL {
        return (true, true); // LB4/LB5
    }
    if p == CR {
        return if c == LF {
            (false, false)
        } else {
            (true, true)
        }; // LB5
    }
    if matches!(c, BK | CR | LF | NL) {
        return (false, false); // LB6
    }
    if matches!(c, SP | ZW) {
        return (false, false); // LB7
    }
    if l == ZW {
        return (true, false); // LB8
    }
    if prev.ends_zwj {
        return (false, false); // LB8a
    }
    if c == WJ || p == WJ {
        return (false, false); // LB11 (immediate)
    }
    if p == GL {
        return (false, false); // LB12 (immediate; no SP*)
    }
    if c == GL && !st.sp && !matches!(l, BA | HY | HH) {
        return (false, false); // LB12a
    }
    if matches!(c, CL | CP | EX | SY) {
        return (false, false); // LB13 (IS handled by LB15c/LB15d)
    }
    if l == OP {
        return (false, false); // LB14
    }
    // LB15a: (sot|OP|QU|GL|SP|ZW) (Pi&QU) SP* × (BK/CR/LF/NL are forced breaks).
    if l == QU && st.r_pi && matches!(st.before, XX | OP | QU | GL | SP | ZW) {
        return (false, false);
    }
    // LB15b: × (Pf&QU) (SP|GL|WJ|CL|QU|CP|EX|IS|SY|BK|CR|LF|NL|ZW|eot).
    if c == QU && cur.pf() {
        let ok = next.map_or(true, |n| {
            matches!(
                n.cls,
                SP | GL | WJ | CL | QU | CP | EX | IS | SY | BK | CR | LF | NL | ZW
            )
        });
        if ok {
            return (false, false);
        }
    }
    // LB15c: SP ÷ IS NU (break before a decimal mark after a space).
    if st.sp && c == IS && next.is_some_and(|n| n.cls == NU) {
        return (true, false);
    }
    // LB15d: × IS (otherwise no break before an infix separator).
    if c == IS {
        return (false, false);
    }
    if matches!(l, CL | CP) && c == NS {
        return (false, false); // LB16
    }
    if l == B2 && c == B2 {
        return (false, false); // LB17
    }
    if st.sp {
        return (true, false); // LB18
    }
    // LB19/LB19a: quotation marks, with the East-Asian-Width refinements.
    // (A few CJK opening/closing-quote edge cases against wide neighbours are
    // not yet exact — see the line-break conformance test.)
    if c == QU && (!prev.wide() || next.map_or(true, |n| !n.wide())) {
        return (false, false);
    }
    if p == QU && (!cur.wide() || prev2.map_or(true, |n| !n.wide())) {
        return (false, false);
    }
    if c == CB || p == CB {
        return (true, false); // LB20
    }
    // LB20a: (sot|BK|CR|LF|NL|SP|ZW|CB|GL) (HY|HH) × (AL|HL). The context is the
    // character immediately before the hyphen (so SP counts).
    if matches!(p, HY | HH)
        && al_hl(c)
        && prev2.map_or(true, |u| {
            matches!(u.cls, BK | CR | LF | NL | SP | ZW | CB | GL)
        })
    {
        return (false, false);
    }
    if matches!(c, BA | HY | NS | HH) || p == BB {
        return (false, false); // LB21
    }
    if prev2.is_some_and(|u| u.cls == HL) && p == HY && c != HL {
        return (false, false); // LB21a: HL HY × [^HL] (the Hebrew compound hyphen)
    }
    if p == SY && c == HL {
        return (false, false); // LB21b
    }
    if c == IN {
        return (false, false); // LB22
    }
    if (al_hl(p) && c == NU) || (p == NU && al_hl(c)) {
        return (false, false); // LB23
    }
    if (p == PR && matches!(c, ID | EB | EM)) || (matches!(p, ID | EB | EM) && c == PO) {
        return (false, false); // LB23a
    }
    if (matches!(p, PR | PO) && al_hl(c)) || (al_hl(p) && matches!(c, PR | PO)) {
        return (false, false); // LB24
    }
    // LB25: numbers. Start: (PR|PO)? (OP|HY)? NU, plus IS × NU (decimal mark).
    if (matches!(p, PR | PO | OP | HY | IS) && c == NU)
        || (matches!(p, PR | PO) && matches!(c, OP | HY) && next.is_some_and(|n| n.cls == NU))
    {
        return (false, false);
    }
    // Inside / ending a number run `NU (NU|SY|IS)* (CL|CP)? (PO|PR)?`.
    if st.num && matches!(c, NU | SY | IS | PO | PR) {
        return (false, false);
    }
    // LB26/LB27: Hangul.
    if (p == JL && matches!(c, JL | JV | H2 | H3))
        || (matches!(p, JV | H2) && matches!(c, JV | JT))
        || (matches!(p, JT | H3) && c == JT)
    {
        return (false, false);
    }
    if (matches!(p, JL | JV | JT | H2 | H3) && matches!(c, IN | PO))
        || (p == PR && matches!(c, JL | JV | JT | H2 | H3))
    {
        return (false, false);
    }
    if al_hl(p) && al_hl(c) {
        return (false, false); // LB28
    }
    // LB28a: Brahmi orthographic syllables. The literal dotted circle (U+25CC)
    // participates as a member of the {AK, ◌, AS} group.
    let dc = |u: &LbUnit| u.base == '\u{25CC}' || matches!(u.cls, AK | AS);
    let dc_prev = dc(prev);
    let dc_cur = dc(cur);
    if (p == AP && (dc_cur || c == AK || c == AS)) // AP × (AK | ◌ | AS)
        || (dc_prev && matches!(c, VF | VI)) // (AK|◌|AS) × (VF|VI)
        || (p == VI && prev2.is_some_and(dc) && (c == AK || cur.base == '\u{25CC}')) // ...VI × (AK|◌)
        || (dc_prev && dc_cur && next.is_some_and(|n| n.cls == VF))
    // (AK|◌|AS) × (AK|◌|AS) VF
    {
        return (false, false);
    }
    if p == IS && al_hl(c) {
        return (false, false); // LB29
    }
    // LB30 (EAW-aware).
    if matches!(p, AL | HL | NU) && c == OP && !cur.wide() {
        return (false, false);
    }
    if p == CP && matches!(c, AL | HL | NU) && !prev.wide() {
        return (false, false);
    }
    if p == RI && c == RI && st.open_ri % 2 == 1 {
        return (false, false); // LB30a
    }
    // LB30b: EB × EM; [Extended_Pictographic & Cn] × EM.
    if c == EM
        && (p == EB
            || (pictographic(prev.base)
                && super::generated::general_category::general_category(prev.base as u32)
                    == super::category::GeneralCategory::Unassigned))
    {
        return (false, false);
    }
    (true, false) // LB31
}

/// A line-break opportunity: the text up to it, and whether the break is
/// mandatory (a hard line break) rather than merely allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineBreak<'a> {
    /// The text segment ending at this break opportunity.
    pub text: &'a str,
    /// `true` for a mandatory break (e.g. after a newline), `false` for an
    /// allowed break point.
    pub mandatory: bool,
}

/// Iterator over the line-break opportunities of a string (UAX #14).
#[derive(Clone)]
pub struct LineBreaks<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Iterator for LineBreaks<'a> {
    type Item = LineBreak<'a>;

    fn next(&mut self) -> Option<LineBreak<'a>> {
        if self.pos >= self.s.len() {
            return None;
        }
        let start = self.pos;
        let mut prev = lb_unit(self.s, start);
        let is_sp = prev.cls == Lb::SP;
        let mut st = LbState {
            regional: if is_sp { Lb::XX } else { prev.cls },
            r_pi: !is_sp && prev.pi(),
            before: Lb::XX, // sot
            sp: is_sp,
            open_ri: u32::from(prev.cls == Lb::RI),
            num: prev.cls == Lb::NU,
        };
        let mut prev2: Option<LbUnit> = None;
        let mut at = prev.end;
        let mut mandatory = false;
        while at < self.s.len() {
            let cur = lb_unit(self.s, at);
            let next = (cur.end < self.s.len()).then(|| lb_unit(self.s, cur.end));
            let (brk, mand) = line_break_before(prev2.as_ref(), &prev, &cur, next.as_ref(), &st);
            if brk {
                mandatory = mand;
                break;
            }
            // Update state with the consumed unit.
            if cur.cls == Lb::SP {
                st.sp = true;
            } else {
                st.before = st.regional;
                st.regional = cur.cls;
                st.r_pi = cur.pi();
                st.sp = false;
            }
            st.open_ri = if cur.cls == Lb::RI { st.open_ri + 1 } else { 0 };
            st.num = match cur.cls {
                Lb::NU => true,
                Lb::SY | Lb::IS | Lb::CL | Lb::CP => st.num,
                _ => false,
            };
            at = cur.end;
            prev2 = Some(prev);
            prev = cur;
        }
        let text = &self.s[start..at];
        self.pos = at;
        Some(LineBreak { text, mandatory })
    }
}

/// Iterate over the line-break opportunities of `s` (UAX #14). Each item is the
/// text since the previous opportunity plus whether the break is mandatory.
#[must_use]
pub fn line_breaks(s: &str) -> LineBreaks<'_> {
    LineBreaks { s, pos: 0 }
}
