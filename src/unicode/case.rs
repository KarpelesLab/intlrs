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
