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

use super::generated::case as gen;

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
    CaseMapIter::new(c, gen::to_upper(c as u32))
}

/// The full lowercase mapping of `c`.
#[inline]
#[must_use]
pub fn to_lowercase(c: char) -> CaseMapIter {
    CaseMapIter::new(c, gen::to_lower(c as u32))
}

/// The full titlecase mapping of `c`.
#[inline]
#[must_use]
pub fn to_titlecase(c: char) -> CaseMapIter {
    CaseMapIter::new(c, gen::to_title(c as u32))
}

/// The full case folding of `c` (UCD statuses C + F), for caseless matching.
#[inline]
#[must_use]
pub fn case_fold(c: char) -> CaseMapIter {
    CaseMapIter::new(c, gen::fold(c as u32))
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
            if let Some(m) = self.cur.as_mut() {
                if let Some(c) = m.next() {
                    return Some(c);
                }
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
