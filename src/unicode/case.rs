//! Full, unconditional case mapping and case folding.
//!
//! `to_uppercase`, `to_lowercase`, `to_titlecase`, and `case_fold` return an
//! iterator over the mapped characters, because a single character can map to
//! several (e.g. `ß` uppercases to `SS`). Only the *unconditional* Unicode
//! mappings are applied — language- and context-sensitive special cases
//! (Turkish dotless i, Greek final sigma, …) are intentionally not handled here:
//! they need surrounding characters, so they live in the whole-string
//! [`lowercase_str`], [`lowercase_str_lang`], [`uppercase_str_lang`] and
//! [`titlecase`], which implement the conditional section of `SpecialCasing.txt`.
//!
//! Case folding ([`case_fold`]) yields the *full* fold (UCD statuses C + F), the
//! basis for caseless string comparison.

use super::generated::case as tables;

/// Internal per-codepoint mapping value. `Same` means "maps to the input
/// character itself"; the wrapper substitutes the original `char`.
///
/// `One`/`Two`/`Three` are constructed only in the generated tables, so under a
/// narrow range tier (or none) some are unused — hence `allow(dead_code)`.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum CaseMap {
    Same,
    One(char),
    Two(char, char),
    Three(char, char, char),
}

/// Iterator over the characters a case mapping produces (1–3 chars). Allocates
/// nothing.
#[derive(Debug, Clone)]
pub struct CaseMapIter {
    buf: [char; 3],
    len: u8,
    pos: u8,
}

impl CaseMapIter {
    #[inline]
    fn new(c: char, m: CaseMap) -> Self {
        let (buf, len) = match m {
            CaseMap::Same => ([c, '\0', '\0'], 1),
            CaseMap::One(a) => ([a, '\0', '\0'], 1),
            CaseMap::Two(a, b) => ([a, b, '\0'], 2),
            CaseMap::Three(a, b, c) => ([a, b, c], 3),
        };
        CaseMapIter { buf, len, pos: 0 }
    }
}

impl Iterator for CaseMapIter {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if self.pos < self.len {
            let c = self.buf[self.pos as usize];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = (self.len - self.pos) as usize;
        (rem, Some(rem))
    }
}

impl ExactSizeIterator for CaseMapIter {}

/// The full uppercase mapping of `c`.
#[inline]
#[must_use]
pub fn to_uppercase(c: char) -> CaseMapIter {
    CaseMapIter::new(c, tables::to_upper(c as u32))
}

/// The full lowercase mapping of `c`.
#[inline]
#[must_use]
pub fn to_lowercase(c: char) -> CaseMapIter {
    CaseMapIter::new(c, tables::to_lower(c as u32))
}

/// The full titlecase mapping of `c`.
#[inline]
#[must_use]
pub fn to_titlecase(c: char) -> CaseMapIter {
    CaseMapIter::new(c, tables::to_title(c as u32))
}

/// The full case folding of `c` (UCD statuses C + F), for caseless matching.
#[inline]
#[must_use]
pub fn case_fold(c: char) -> CaseMapIter {
    CaseMapIter::new(c, tables::fold(c as u32))
}

/// Iterator adaptor applying a per-character case mapping across a whole `char`
/// stream, flattening multi-character mappings. Allocates nothing.
///
/// Constructed via [`uppercase`], [`lowercase`], or [`fold`]. With `std`/`alloc`
/// you can `.collect::<String>()`:
///
/// ```
/// use intl::unicode::uppercase;
/// assert_eq!(uppercase("Weiß".chars()).collect::<String>(), "WEISS");
/// ```
#[derive(Clone)]
pub struct CaseMapping<I> {
    iter: I,
    map: fn(char) -> CaseMapIter,
    cur: Option<CaseMapIter>,
}

impl<I: Iterator<Item = char>> Iterator for CaseMapping<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        loop {
            if let Some(m) = self.cur.as_mut()
                && let Some(c) = m.next()
            {
                return Some(c);
            }
            let ch = self.iter.next()?;
            self.cur = Some((self.map)(ch));
        }
    }
}

/// Map a `char` stream to its full uppercase form (e.g. `"Weiß"` → `"WEISS"`).
#[inline]
pub fn uppercase<I: Iterator<Item = char>>(iter: I) -> CaseMapping<I> {
    CaseMapping {
        iter,
        map: to_uppercase,
        cur: None,
    }
}

/// Map a `char` stream to its full lowercase form.
#[inline]
pub fn lowercase<I: Iterator<Item = char>>(iter: I) -> CaseMapping<I> {
    CaseMapping {
        iter,
        map: to_lowercase,
        cur: None,
    }
}

/// Map a `char` stream to its full case-folded form, for caseless comparison:
/// `fold(a).eq(fold(b))` is `true` when `a` and `b` differ only by case.
#[inline]
pub fn fold<I: Iterator<Item = char>>(iter: I) -> CaseMapping<I> {
    CaseMapping {
        iter,
        map: case_fold,
        cur: None,
    }
}

// ---------------------------------------------------------------------------
// SpecialCasing.txt conditional mappings
//
// The entries in the "Conditional Mappings" section of SpecialCasing.txt fire
// only when a context condition holds over the *original* string (never the
// partially built result). Each condition below is implemented as a predicate
// over the neighbouring characters, per the definitions in the "Default Case
// Algorithms" section of the core specification (UAX #21).
// ---------------------------------------------------------------------------

/// The nearest neighbour, in iteration order, whose canonical combining class is
/// 0 (a starter) or 230 (Above).
///
/// Every dot-above condition is phrased as "… with no intervening character of
/// combining class 0 or 230", so each reduces to a single test on this
/// character: an Above mark or a new starter takes over the position above the
/// letter and thereby breaks the context.
#[cfg(feature = "alloc")]
fn ccc_boundary(mut it: impl Iterator<Item = char>) -> Option<char> {
    use super::normalize::canonical_combining_class;
    it.find(|&c| matches!(canonical_combining_class(c), 0 | 230))
}

/// `After_Soft_Dotted`: some `Soft_Dotted` character precedes, with no
/// intervening character of combining class 0 or 230 (Above).
#[cfg(feature = "alloc")]
fn after_soft_dotted(before: &[char]) -> bool {
    use super::generated::binary_props::soft_dotted;
    ccc_boundary(before.iter().rev().copied()).is_some_and(|c| soft_dotted(c as u32))
}

/// `After_I`: an uppercase `I` precedes, with no intervening character of
/// combining class 0 or 230 (Above). Only U+0049 itself counts — `Į` and `İ`
/// do not.
#[cfg(feature = "alloc")]
fn after_i(before: &[char]) -> bool {
    ccc_boundary(before.iter().rev().copied()) == Some('I')
}

/// `More_Above`: a character of combining class 230 (Above) follows, with no
/// intervening character of combining class 0 or 230.
#[cfg(feature = "alloc")]
fn more_above(after: &[char]) -> bool {
    use super::normalize::canonical_combining_class;
    ccc_boundary(after.iter().copied()).is_some_and(|c| canonical_combining_class(c) == 230)
}

/// `Before_Dot`: U+0307 COMBINING DOT ABOVE follows, with no intervening
/// character of combining class 0 or 230.
#[cfg(feature = "alloc")]
fn before_dot(after: &[char]) -> bool {
    ccc_boundary(after.iter().copied()) == Some('\u{0307}')
}

/// `Final_Sigma`: a cased letter precedes and none follows, skipping
/// case-ignorable characters on both sides — i.e. `Σ` sits at the end of a word.
#[cfg(feature = "alloc")]
fn final_sigma(before: &[char], after: &[char]) -> bool {
    use super::generated::binary_props::{case_ignorable, cased};
    let preceded = before
        .iter()
        .rev()
        .find(|&&p| !case_ignorable(p as u32))
        .is_some_and(|&p| cased(p as u32));
    let followed = after
        .iter()
        .find(|&&n| !case_ignorable(n as u32))
        .is_some_and(|&n| cased(n as u32));
    preceded && !followed
}

/// Which language-sensitive block of SpecialCasing.txt a whole-string mapping
/// applies on top of the language-independent conditions.
#[cfg(feature = "alloc")]
#[derive(Clone, Copy)]
enum Tailoring {
    /// No language tailoring; only the language-insensitive conditions apply.
    None,
    /// `tr`/`az`: dotted/dotless i, `After_I` and `Not_Before_Dot`.
    Turkic,
    /// `lt`: retained dot above, `More_Above` and `After_Soft_Dotted`.
    Lithuanian,
}

/// Shared lowercasing pass. `tailoring` selects the language-sensitive entries;
/// the language-insensitive `Final_Sigma` applies in every locale (verified
/// against ICU: `'ΑΣ'.toLocaleLowerCase('tr')` is `"ας"` with final sigma).
#[cfg(feature = "alloc")]
fn lowercase_tailored(s: &str, tailoring: Tailoring) -> alloc::string::String {
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    let mut out = alloc::string::String::new();
    for (i, &c) in chars.iter().enumerate() {
        let (before, after) = (&chars[..i], &chars[i + 1..]);
        match (tailoring, c) {
            // `0307; ; 0307; 0307; tr/az After_I` — the dot is dropped, because
            // the `I` it sits on already lowercases to a dotted `i`.
            (Tailoring::Turkic, '\u{0307}') if after_i(before) => {}
            // `0049; 0131; 0049; 0049; tr/az Not_Before_Dot` — an `I` with no dot
            // above to absorb becomes dotless `ı`. When a dot does follow, the
            // default `I` → `i` mapping applies and the dot is dropped above.
            (Tailoring::Turkic, 'I') if !before_dot(after) => out.push('\u{0131}'),
            // `0130; 0069; 0130; 0130; tr/az` — unconditional in Turkic: the
            // dot of `İ` is inherent to `i`, so no U+0307 is emitted.
            (Tailoring::Turkic, '\u{0130}') => out.push('i'),
            // `0049/004A/012E; … 0307; lt More_Above` — Lithuanian keeps the dot
            // of `i`/`j`/`į` visible under a further accent, so it is spelled out.
            (Tailoring::Lithuanian, 'I' | 'J' | '\u{012E}') if more_above(after) => {
                out.extend(to_lowercase(c));
                out.push('\u{0307}');
            }
            // `00CC/00CD/0128; …; lt` — unconditional in Lithuanian: decompose so
            // the retained dot sits between the `i` and its accent.
            (Tailoring::Lithuanian, '\u{00CC}') => out.push_str("i\u{0307}\u{0300}"),
            (Tailoring::Lithuanian, '\u{00CD}') => out.push_str("i\u{0307}\u{0301}"),
            (Tailoring::Lithuanian, '\u{0128}') => out.push_str("i\u{0307}\u{0303}"),
            // `03A3; 03C2; 03A3; 03A3; Final_Sigma` — language-insensitive.
            (_, '\u{03A3}') => out.push(if final_sigma(before, after) {
                '\u{03C2}'
            } else {
                '\u{03C3}'
            }),
            _ => out.extend(to_lowercase(c)),
        }
    }
    out
}

/// Shared uppercasing pass. The only *conditional* uppercase entry in
/// SpecialCasing.txt is Lithuanian `After_Soft_Dotted`; the Turkic uppercase
/// entries are unconditional, so only the Lithuanian path needs the string
/// buffered for backward context.
#[cfg(feature = "alloc")]
fn uppercase_tailored(s: &str, tailoring: Tailoring) -> alloc::string::String {
    let mut out = alloc::string::String::new();
    match tailoring {
        // No conditional uppercase entry applies; a per-character pass suffices.
        Tailoring::None => return uppercase(s.chars()).collect(),
        Tailoring::Turkic => {
            for c in s.chars() {
                match c {
                    // `0069; 0069; 0130; 0130; tr/az` — `i` keeps its dot: `i` → `İ`.
                    // The converse `0131` → `0049` needs no arm; SpecialCasing.txt
                    // notes it is already in UnicodeData.txt.
                    'i' => out.push('\u{0130}'),
                    _ => out.extend(to_uppercase(c)),
                }
            }
        }
        Tailoring::Lithuanian => {
            let chars: alloc::vec::Vec<char> = s.chars().collect();
            for (i, &c) in chars.iter().enumerate() {
                // `0307; 0307; ; ; lt After_Soft_Dotted` — the explicit dot above
                // is dropped once the soft-dotted letter carrying it has been
                // uppercased (capital `I`/`J` have no dot for it to duplicate).
                if c == '\u{0307}' && after_soft_dotted(&chars[..i]) {
                    continue;
                }
                out.extend(to_uppercase(c));
            }
        }
    }
    out
}

/// Lower-case a whole string, applying the context-sensitive Greek
/// **Final_Sigma** rule that the per-character [`to_lowercase`] cannot: a capital
/// sigma `Σ` becomes final `ς` at the end of a word (preceded by a cased letter,
/// not followed by one) and `σ` elsewhere. Requires the `alloc` feature.
///
/// ```
/// use intl::unicode::lowercase_str;
/// assert_eq!(lowercase_str("ὈΔΥΣΣΕΎΣ"), "ὀδυσσεύς"); // final Σ → ς, medial → σ
/// assert_eq!(lowercase_str("HELLO"), "hello");
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn lowercase_str(s: &str) -> alloc::string::String {
    lowercase_tailored(s, Tailoring::None)
}

/// `true` if `lang` is a Turkic locale (Turkish / Azerbaijani) using the
/// dotted/dotless-i casing rules.
#[cfg(feature = "alloc")]
fn is_turkic(lang: &str) -> bool {
    let l = lang.as_bytes();
    if l.len() < 2 {
        return false;
    }
    let prefix = [l[0] | 0x20, l[1] | 0x20];
    let lang_ok = prefix == *b"tr" || prefix == *b"az";
    let boundary = l.len() == 2 || l[2] == b'-' || l[2] == b'_';
    lang_ok && boundary
}

/// `true` if `lang` is Lithuanian (retained-dot casing rules).
#[cfg(feature = "alloc")]
fn is_lithuanian(lang: &str) -> bool {
    let l = lang.as_bytes();
    l.len() >= 2
        && (l[0] | 0x20) == b'l'
        && (l[1] | 0x20) == b't'
        && (l.len() == 2 || l[2] == b'-' || l[2] == b'_')
}

/// The tailoring SpecialCasing.txt defines for `lang`, if any.
#[cfg(feature = "alloc")]
fn tailoring_for(lang: &str) -> Tailoring {
    if is_turkic(lang) {
        Tailoring::Turkic
    } else if is_lithuanian(lang) {
        Tailoring::Lithuanian
    } else {
        Tailoring::None
    }
}

/// Lower-case a string with `lang`'s locale rules: Turkic (`tr`/`az`) `I`→`ı`
/// and `İ`→`i`; Lithuanian (`lt`) keeps the dot above on `i`/`j` under accents;
/// otherwise it matches [`lowercase_str`]. Greek Final_Sigma applies in every
/// locale. Needs `alloc`.
///
/// ```
/// use intl::unicode::lowercase_str_lang;
/// assert_eq!(lowercase_str_lang("TITLE", "tr"), "tıtle"); // dotless ı
/// assert_eq!(lowercase_str_lang("TITLE", "en"), "title");
/// assert_eq!(lowercase_str_lang("İ", "tr"), "i");
/// assert_eq!(lowercase_str_lang("I\u{307}", "tr"), "i"); // After_I
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn lowercase_str_lang(s: &str, lang: &str) -> alloc::string::String {
    lowercase_tailored(s, tailoring_for(lang))
}

/// Upper-case a string with `lang`'s locale rules: for Turkic locales
/// (`tr`/`az`) `i`→`İ` and `ı`→`I`; for Lithuanian (`lt`) an explicit dot above
/// on a soft-dotted letter is dropped; otherwise the default full uppercase.
/// Requires the `alloc` feature.
///
/// ```
/// use intl::unicode::uppercase_str_lang;
/// assert_eq!(uppercase_str_lang("title", "tr"), "TİTLE"); // dotted İ
/// assert_eq!(uppercase_str_lang("title", "en"), "TITLE");
/// assert_eq!(uppercase_str_lang("i\u{307}", "lt"), "I"); // After_Soft_Dotted
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn uppercase_str_lang(s: &str, lang: &str) -> alloc::string::String {
    uppercase_tailored(s, tailoring_for(lang))
}

/// Title-case a string: the first cased character of each word (per UAX #29
/// word segmentation) is title-cased and the rest are lower-cased
/// (`"loud HOUSE" → "Loud House"`). Requires the `alloc` feature.
#[cfg(feature = "alloc")]
#[must_use]
pub fn titlecase(s: &str) -> alloc::string::String {
    use super::category::Group;
    // `words` partitions `s`, so `i` tracks the position of `c` in the whole
    // string — Final_Sigma's context is the original string, not the word.
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    let mut out = alloc::string::String::new();
    let mut i = 0;
    for word in super::segment::words(s) {
        let mut titled = false;
        for c in word.chars() {
            // The first cased letter is title-cased; everything else lower-cased.
            let is_cased = matches!(
                super::generated::general_category::general_category(c as u32).group(),
                Group::Letter
            );
            if !titled && is_cased {
                out.extend(to_titlecase(c));
                titled = true;
            } else if c == '\u{03A3}' && final_sigma(&chars[..i], &chars[i + 1..]) {
                // Every non-initial character is lower-cased, so the
                // language-insensitive `Final_Sigma` entry applies here as well.
                out.push('\u{03C2}');
            } else {
                out.extend(to_lowercase(c));
            }
            i += 1;
        }
    }
    out
}
