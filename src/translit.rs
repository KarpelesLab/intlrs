//! Transliteration / transforms. Requires the `alloc` feature.
//!
//! Currently provides the **Latin → ASCII** fold ([`latin_ascii`]): strip
//! diacritics and map the non-decomposing Latin letters and common typographic
//! punctuation to plain ASCII. This is the ICU `Latin-ASCII` transform — the
//! workhorse for slugs, ASCII fallbacks, and search-key folding. Characters
//! outside the Latin script that have no ASCII form are left unchanged.

use crate::unicode::{general_category, nfd, Group};
use alloc::string::String;

/// Remove diacritics from `c`'s text: decompose (NFD), drop the combining
/// marks, and recompose (NFC), so accented letters lose their accents but every
/// base letter (including non-Latin scripts) is preserved. Unlike
/// [`latin_ascii`] it does not transliterate (`ß`, `ø`, `æ` stay as they are).
/// Useful for accent-insensitive search/matching.
///
/// ```
/// use intl::translit::remove_diacritics;
/// assert_eq!(remove_diacritics("café Müller"), "cafe Muller");
/// assert_eq!(remove_diacritics("naïve"), "naive");
/// assert_eq!(remove_diacritics("ψυχή"), "ψυχη"); // Greek tonos removed
/// ```
#[must_use]
pub fn remove_diacritics(s: &str) -> String {
    let stripped = nfd(s.chars()).filter(|&c| !matches!(general_category(c).group(), Group::Mark));
    crate::unicode::nfc(stripped).collect()
}

/// Transliterate Cyrillic script to Latin using **ISO 9:1995** — the single,
/// language-independent, reversible standard (so Russian, Ukrainian, Serbian,
/// Bulgarian, … all map consistently). The output uses Latin letters with
/// diacritics (`ж→ž`, `ч→č`, `ш→š`); chain with [`latin_ascii`] for plain ASCII.
/// Non-Cyrillic characters pass through unchanged.
///
/// ```
/// use intl::translit::{cyrillic_to_latin, latin_ascii};
/// assert_eq!(cyrillic_to_latin("Москва"), "Moskva");
/// assert_eq!(cyrillic_to_latin("Привет"), "Privet");
/// assert_eq!(latin_ascii(&cyrillic_to_latin("Чехов")), "Cehov"); // č -> c
/// ```
#[must_use]
pub fn cyrillic_to_latin(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'а' => out.push('a'),
            'б' => out.push('b'),
            'в' => out.push('v'),
            'г' => out.push('g'),
            'д' => out.push('d'),
            'е' => out.push('e'),
            'ё' => out.push('ë'),
            'ж' => out.push('ž'),
            'з' => out.push('z'),
            'и' => out.push('i'),
            'й' => out.push('j'),
            'к' => out.push('k'),
            'л' => out.push('l'),
            'м' => out.push('m'),
            'н' => out.push('n'),
            'о' => out.push('o'),
            'п' => out.push('p'),
            'р' => out.push('r'),
            'с' => out.push('s'),
            'т' => out.push('t'),
            'у' => out.push('u'),
            'ф' => out.push('f'),
            'х' => out.push('h'),
            'ц' => out.push('c'),
            'ч' => out.push('č'),
            'ш' => out.push('š'),
            'щ' => out.push('ŝ'),
            'ъ' => out.push('ʺ'),
            'ы' => out.push('y'),
            'ь' => out.push('ʹ'),
            'э' => out.push('è'),
            'ю' => out.push('û'),
            'я' => out.push('â'),
            // Non-Russian common Cyrillic letters.
            'і' => out.push('ì'),
            'ї' => out.push('ï'),
            'є' => out.push('ê'),
            'ґ' => out.push('g'),
            'ђ' => out.push('đ'),
            'ј' => out.push('j'),
            'љ' => out.push_str("lj"),
            'њ' => out.push_str("nj"),
            'ћ' => out.push('ć'),
            'џ' => out.push_str("dž"),
            'ѕ' => out.push('ẑ'),
            // Uppercase: transliterate the lowercased form, then upper-case it.
            'А'..='Я' | 'Ё' | 'І' | 'Ї' | 'Є' | 'Ґ' | 'Ђ' | 'Ј' | 'Љ' | 'Њ' | 'Ћ' | 'Џ' | 'Ѕ' =>
            {
                let lower = c.to_lowercase().next().unwrap_or(c);
                let t = cyrillic_to_latin(lower.encode_utf8(&mut [0u8; 4]));
                let mut chars = t.chars();
                if let Some(first) = chars.next() {
                    out.extend(first.to_uppercase());
                    out.push_str(chars.as_str());
                }
            }
            _ => out.push(c),
        }
    }
    out
}

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
