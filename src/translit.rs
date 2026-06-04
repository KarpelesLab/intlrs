//! Transliteration / transforms. Requires the `alloc` feature.
//!
//! Currently provides the **Latin → ASCII** fold ([`latin_ascii`]): strip
//! diacritics and map the non-decomposing Latin letters and common typographic
//! punctuation to plain ASCII. This is the ICU `Latin-ASCII` transform — the
//! workhorse for slugs, ASCII fallbacks, and search-key folding. Characters
//! outside the Latin script that have no ASCII form are left unchanged.

use crate::unicode::{general_category, nfd, Group};
use alloc::string::String;

/// Fold Latin text to ASCII: decompose (NFD), drop combining marks, and map the
/// non-decomposing Latin letters (`ø→o`, `æ→ae`, `ß→ss`, `þ→th`, …) and common
/// typographic punctuation (curly quotes, dashes, ellipsis, NBSP). Non-Latin
/// characters with no ASCII equivalent pass through unchanged.
///
/// ```
/// use intl::translit::latin_ascii;
/// assert_eq!(latin_ascii("café"), "cafe");
/// assert_eq!(latin_ascii("Straße"), "Strasse");
/// assert_eq!(latin_ascii("naïve Æsir"), "naive AEsir");
/// assert_eq!(latin_ascii("Łódź"), "Lodz");
/// ```
#[must_use]
pub fn latin_ascii(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in nfd(s.chars()) {
        // Drop the combining marks left behind by decomposition.
        if matches!(general_category(c).group(), Group::Mark) {
            continue;
        }
        match c {
            '\0'..='\u{7F}' => out.push(c),
            // Non-decomposing Latin letters.
            'Ø' => out.push('O'),
            'ø' => out.push('o'),
            'Đ' | 'Ð' => out.push('D'),
            'đ' | 'ð' => out.push('d'),
            'Ł' => out.push('L'),
            'ł' => out.push('l'),
            'Ħ' => out.push('H'),
            'ħ' => out.push('h'),
            'Ŧ' => out.push('T'),
            'ŧ' => out.push('t'),
            'ı' => out.push('i'),
            'İ' => out.push('I'),
            'ŉ' => out.push('n'),
            'Þ' => out.push_str("Th"),
            'þ' => out.push_str("th"),
            'Æ' => out.push_str("AE"),
            'æ' => out.push_str("ae"),
            'Œ' => out.push_str("OE"),
            'œ' => out.push_str("oe"),
            'ß' => out.push_str("ss"),
            'ẞ' => out.push_str("SS"),
            'Ŋ' => out.push_str("NG"),
            'ŋ' => out.push_str("ng"),
            'Ĳ' => out.push_str("IJ"),
            'ĳ' => out.push_str("ij"),
            // Common typographic punctuation.
            '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{2032}' => out.push('\''),
            '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{2033}' => out.push('"'),
            '\u{2013}' | '\u{2014}' | '\u{2212}' => out.push('-'),
            '\u{2026}' => out.push_str("..."),
            '\u{00A0}' | '\u{2007}' | '\u{2009}' | '\u{202F}' => out.push(' '),
            '\u{00AB}' | '\u{00BB}' => out.push('"'),
            // Anything else (e.g. CJK, unmapped symbols) is preserved as-is.
            _ => out.push(c),
        }
    }
    out
}
