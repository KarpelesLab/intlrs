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

/// One transform rule: `before { source } after > target` — replace `source`
/// with `target`, but only when preceded by `before` and followed by `after`
/// (either context may be empty). The `before`/`after` markers are optional.
#[derive(Debug, Clone)]
struct Rule {
    before: String,
    source: String,
    after: String,
    target: String,
}

/// A rule-based transform: an ordered set of `source > target` string rewrites,
/// applied left-to-right with **longest-source-first** at each position, with
/// optional **context** (`before { source } after > target`, the ICU syntax).
/// A lightweight subset of ICU transform rules (literal rewrites + context; no
/// sets or back-references). Build it once and reuse it.
///
/// ```
/// use intl::translit::Transform;
/// let leet = Transform::parse("a > 4; e > 3; o > 0; ck > k").unwrap();
/// assert_eq!(leet.apply("rocket"), "r0k3t");   // "ck" wins over "c","k"
///
/// // Context: 'n' before 'g' becomes 'ŋ'; 'n' elsewhere is unchanged.
/// let ctx = Transform::parse("n } g > ŋ").unwrap();
/// assert_eq!(ctx.apply("sing song / no"), "siŋg soŋg / no");
/// ```
#[derive(Debug, Clone)]
pub struct Transform {
    rules: alloc::vec::Vec<Rule>,
}

impl Transform {
    /// Parse a rule string: rules are separated by `;`, each
    /// `[before {] source [} after] > target` (whitespace around the parts is
    /// trimmed). Returns `None` if no valid rule is found. Rules are matched
    /// longest-source-first.
    #[must_use]
    pub fn parse(rules: &str) -> Option<Transform> {
        let mut parsed: alloc::vec::Vec<Rule> = alloc::vec::Vec::new();
        for rule in rules.split(';') {
            let Some((lhs, target)) = rule.split_once('>') else {
                continue;
            };
            // Split the left side into  before { source } after.
            let (before, rest) = match lhs.split_once('{') {
                Some((b, r)) => (b.trim(), r),
                None => ("", lhs),
            };
            let (source, after) = match rest.split_once('}') {
                Some((s, a)) => (s.trim(), a.trim()),
                None => (rest.trim(), ""),
            };
            if !source.is_empty() {
                parsed.push(Rule {
                    before: before.into(),
                    source: source.into(),
                    after: after.into(),
                    target: target.trim().into(),
                });
            }
        }
        if parsed.is_empty() {
            return None;
        }
        parsed.sort_by_key(|r| core::cmp::Reverse(r.source.chars().count()));
        Some(Transform { rules: parsed })
    }

    /// Apply the transform to `s`. The `before` context matches the already-
    /// converted output; `after` matches the remaining input.
    #[must_use]
    pub fn apply(&self, s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut rest = s;
        'outer: while !rest.is_empty() {
            for rule in &self.rules {
                if !out.ends_with(rule.before.as_str()) {
                    continue;
                }
                if let Some(after) = rest.strip_prefix(rule.source.as_str()) {
                    if after.starts_with(rule.after.as_str()) {
                        out.push_str(&rule.target);
                        rest = after;
                        continue 'outer;
                    }
                }
            }
            let c = rest.chars().next().unwrap();
            out.push(c);
            rest = &rest[c.len_utf8()..];
        }
        out
    }
}

/// Best-effort transliteration of `s` to plain ASCII: romanize Cyrillic (ISO 9)
/// and Greek (ELOT/ISO 843), then fold the result with [`latin_ascii`]. Latin
/// text is accent-folded; scripts with no romanization here (e.g. CJK) are left
/// unchanged. Handy for slugs and search keys over mixed-script input.
///
/// ```
/// use intl::translit::any_ascii;
/// assert_eq!(any_ascii("Москва café Αθήνα"), "Moskva cafe Athina");
/// assert_eq!(any_ascii("Straße"), "Strasse");
/// ```
#[must_use]
pub fn any_ascii(s: &str) -> String {
    latin_ascii(&greek_to_latin(&cyrillic_to_latin(s)))
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

/// Transliterate Greek script to Latin (ELOT 743 / ISO 843, ASCII-leaning):
/// `θ→th`, `χ→ch`, `ψ→ps`, `η→i`, `ω→o`, accents dropped. Non-Greek characters
/// pass through unchanged.
///
/// ```
/// use intl::translit::greek_to_latin;
/// assert_eq!(greek_to_latin("Αθήνα"), "Athina");
/// assert_eq!(greek_to_latin("ψυχή"), "psychi");
/// assert_eq!(greek_to_latin("Ελλάδα"), "Ellada");
/// ```
/// The ASCII-leaning Latin form of a lowercase Greek letter, or `None`.
fn greek_letter(c: char) -> Option<&'static str> {
    Some(match c {
        'α' => "a",
        'β' => "v",
        'γ' => "g",
        'δ' => "d",
        'ε' => "e",
        'ζ' => "z",
        'η' => "i",
        'θ' => "th",
        'ι' => "i",
        'κ' => "k",
        'λ' => "l",
        'μ' => "m",
        'ν' => "n",
        'ξ' => "x",
        'ο' => "o",
        'π' => "p",
        'ρ' => "r",
        'σ' | 'ς' => "s",
        'τ' => "t",
        'υ' => "y",
        'φ' => "f",
        'χ' => "ch",
        'ψ' => "ps",
        'ω' => "o",
        _ => return None,
    })
}

#[must_use]
pub fn greek_to_latin(s: &str) -> String {
    // NFD so accented vowels become base + combining mark; drop the marks.
    let chars: alloc::vec::Vec<char> = nfd(s.chars())
        .filter(|&c| !matches!(general_category(c).group(), Group::Mark))
        .collect();
    let mut out = String::with_capacity(s.len());
    for (i, &c) in chars.iter().enumerate() {
        if let Some(latin) = greek_letter(c) {
            out.push_str(latin);
        } else if c.is_uppercase() && greek_letter(c.to_lowercase().next().unwrap_or(c)).is_some() {
            let latin = greek_letter(c.to_lowercase().next().unwrap()).unwrap();
            // All-caps the digraph when a neighbor is upper-case (e.g. "ΘΕΟΣ"),
            // else title-case it ("Θεός" → "Theos").
            let all_caps = chars.get(i + 1).is_some_and(|n| n.is_uppercase())
                || (i > 0 && chars[i - 1].is_uppercase());
            if all_caps {
                out.extend(latin.chars().flat_map(char::to_uppercase));
            } else {
                let mut it = latin.chars();
                if let Some(f) = it.next() {
                    out.extend(f.to_uppercase());
                    out.push_str(it.as_str());
                }
            }
        } else {
            out.push(c);
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
