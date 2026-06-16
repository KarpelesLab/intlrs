//! Full, unconditional case mapping and case folding.
//!
//! `to_uppercase`, `to_lowercase`, `to_titlecase`, and `case_fold` return an
//! iterator over the mapped characters, because a single character can map to
//! several (e.g. `ß` uppercases to `SS`). Only the *unconditional* Unicode
//! mappings are applied — language- and context-sensitive special cases
//! (Turkish dotless i, Greek final sigma, …) are intentionally not handled here.
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
    use super::generated::binary_props::{case_ignorable, cased};
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    let mut out = alloc::string::String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c == '\u{03A3}' {
            // Final_Sigma: a cased letter precedes (skipping case-ignorables) and
            // none follows.
            let before = chars[..i]
                .iter()
                .rev()
                .find(|&&p| !case_ignorable(p as u32))
                .is_some_and(|&p| cased(p as u32));
            let after = chars[i + 1..]
                .iter()
                .find(|&&n| !case_ignorable(n as u32))
                .is_some_and(|&n| cased(n as u32));
            out.push(if before && !after {
                '\u{03C2}'
            } else {
                '\u{03C3}'
            });
        } else {
            out.extend(to_lowercase(c));
        }
    }
    out
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

/// Lithuanian lowercasing: keep an explicit `U+0307` COMBINING DOT ABOVE on a
/// lowercased `i`/`j`/`į` whenever an above-class accent follows (so the dot is
/// visible under the accent), and expand the precomposed `Ì`/`Í`/`Ĩ` likewise.
#[cfg(feature = "alloc")]
fn lithuanian_lower(s: &str) -> alloc::string::String {
    use super::normalize::canonical_combining_class;
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    let mut out = alloc::string::String::new();
    for (idx, &c) in chars.iter().enumerate() {
        match c {
            'I' | 'J' | '\u{012E}' => {
                out.extend(to_lowercase(c));
                // More_Above: a class-230 (Above) mark follows in the combining
                // sequence (before any starter).
                let more_above = chars[idx + 1..]
                    .iter()
                    .find_map(|&n| match canonical_combining_class(n) {
                        0 => Some(false),
                        230 => Some(true),
                        _ => None,
                    })
                    .unwrap_or(false);
                if more_above {
                    out.push('\u{0307}');
                }
            }
            '\u{00CC}' => out.push_str("i\u{0307}\u{0300}"), // Ì
            '\u{00CD}' => out.push_str("i\u{0307}\u{0301}"), // Í
            '\u{0128}' => out.push_str("i\u{0307}\u{0303}"), // Ĩ
            _ => out.extend(to_lowercase(c)),
        }
    }
    out
}

/// Lower-case a string with `lang`'s locale rules: Turkic (`tr`/`az`) `I`→`ı`
/// and `İ`→`i`; Lithuanian (`lt`) keeps the dot above on `i`/`j` under accents;
/// otherwise it matches [`lowercase_str`] (incl. Greek Final_Sigma). Needs `alloc`.
///
/// ```
/// use intl::unicode::lowercase_str_lang;
/// assert_eq!(lowercase_str_lang("TITLE", "tr"), "tıtle"); // dotless ı
/// assert_eq!(lowercase_str_lang("TITLE", "en"), "title");
/// assert_eq!(lowercase_str_lang("İ", "tr"), "i");
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn lowercase_str_lang(s: &str, lang: &str) -> alloc::string::String {
    if is_lithuanian(lang) {
        return lithuanian_lower(s);
    }
    if !is_turkic(lang) {
        return lowercase_str(s);
    }
    let mut out = alloc::string::String::new();
    for c in s.chars() {
        match c {
            'I' => out.push('\u{0131}'),        // I → ı (dotless)
            '\u{0130}' => out.push('i'),        // İ → i
            '\u{03A3}' => out.push('\u{03C3}'), // keep Σ handling simple in Turkic
            _ => out.extend(to_lowercase(c)),
        }
    }
    out
}

/// Upper-case a string with `lang`'s locale rules: for Turkic locales
/// (`tr`/`az`) `i`→`İ` and `ı`→`I`; otherwise the default full uppercase.
/// Requires the `alloc` feature.
///
/// ```
/// use intl::unicode::uppercase_str_lang;
/// assert_eq!(uppercase_str_lang("title", "tr"), "TİTLE"); // dotted İ
/// assert_eq!(uppercase_str_lang("title", "en"), "TITLE");
/// ```
#[cfg(feature = "alloc")]
#[must_use]
pub fn uppercase_str_lang(s: &str, lang: &str) -> alloc::string::String {
    if !is_turkic(lang) {
        return uppercase(s.chars()).collect();
    }
    let mut out = alloc::string::String::new();
    for c in s.chars() {
        match c {
            'i' => out.push('\u{0130}'), // i → İ (dotted)
            '\u{0131}' => out.push('I'), // ı → I
            _ => out.extend(to_uppercase(c)),
        }
    }
    out
}

/// Title-case a string: the first cased character of each word (per UAX #29
/// word segmentation) is title-cased and the rest are lower-cased
/// (`"loud HOUSE" → "Loud House"`). Requires the `alloc` feature.
#[cfg(feature = "alloc")]
#[must_use]
pub fn titlecase(s: &str) -> alloc::string::String {
    use super::category::Group;
    let mut out = alloc::string::String::new();
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
            } else {
                out.extend(to_lowercase(c));
            }
        }
    }
    out
}
