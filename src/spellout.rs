//! Spelling integers out in words via the CLDR rule-based number format (RBNF).
//! Requires the `alloc` feature. Locale-driven (a curated set of locales), not
//! hardcoded for any language.
//!
//! ```
//! use intl::spellout::spell_cardinal;
//! assert_eq!(spell_cardinal("en", 1234).as_deref(), Some("one thousand two hundred thirty-four"));
//! assert_eq!(spell_cardinal("en", -42).as_deref(), Some("minus forty-two"));
//! assert_eq!(spell_cardinal("fr", 80).as_deref(), Some("quatre-vingts"));
//! assert_eq!(spell_cardinal("xx", 1), None); // unsupported locale
//! ```

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A named RBNF rule set: an ordered list of `(key, text)` rules.
struct RuleSet {
    name: String,
    rules: Vec<(String, String)>,
}

fn rd_str(b: &[u8], o: &mut usize) -> String {
    let n = b[*o] as usize;
    *o += 1;
    let s = core::str::from_utf8(&b[*o..*o + n])
        .unwrap_or("")
        .to_string();
    *o += n;
    s
}

fn parse_payload(b: &[u8]) -> (String, Vec<RuleSet>) {
    let mut o = 0;
    let start = rd_str(b, &mut o);
    let rs_count = b[o] as usize;
    o += 1;
    let mut sets = Vec::with_capacity(rs_count);
    for _ in 0..rs_count {
        let name = rd_str(b, &mut o);
        let rule_count = u16::from_le_bytes([b[o], b[o + 1]]) as usize;
        o += 2;
        let mut rules = Vec::with_capacity(rule_count);
        for _ in 0..rule_count {
            let k = rd_str(b, &mut o);
            let t = rd_str(b, &mut o);
            rules.push((k, t));
        }
        sets.push(RuleSet { name, rules });
    }
    (start, sets)
}

/// Resolve `lang` (with `pt-BR` → `pt` fallback) to its RBNF rule sets and the
/// default (`_start`) rule-set name.
fn resolve(lang: &str) -> Option<(String, alloc::vec::Vec<RuleSet>)> {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let bytes = loop {
        if let Some(b) = crate::cldr::rbnf_payload(&norm[..end]) {
            break b;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return None,
        }
    };
    Some(parse_payload(bytes))
}

/// Spell `value` out as a cardinal number in `lang`, e.g.
/// `spell_cardinal("en", 1234)` → `"one thousand two hundred thirty-four"`.
/// Returns `None` for an unsupported locale.
#[must_use]
pub fn spell_cardinal(lang: &str, value: i64) -> Option<String> {
    let (start, sets) = resolve(lang)?;
    Some(spell(&sets, &start, value))
}

/// Spell `value` out as an ordinal number in `lang`, e.g.
/// `spell_ordinal("en", 21)` → `"twenty-first"`. Where a language defines only
/// gendered ordinals (Spanish, Italian, …), the masculine form is used. Returns
/// `None` for a locale without ordinal spell-out rules.
///
/// ```
/// use intl::spellout::spell_ordinal;
/// assert_eq!(spell_ordinal("en", 1).as_deref(), Some("first"));
/// assert_eq!(spell_ordinal("en", 23).as_deref(), Some("twenty-third"));
/// assert_eq!(spell_ordinal("es", 1).as_deref(), Some("primero"));
/// assert_eq!(spell_ordinal("xx", 1), None); // unsupported locale
/// ```
#[must_use]
pub fn spell_ordinal(lang: &str, value: i64) -> Option<String> {
    let (_, sets) = resolve(lang)?;
    let ruleset = ["spellout-ordinal", "spellout-ordinal-masculine"]
        .into_iter()
        .find(|r| sets.iter().any(|s| s.name == *r))?;
    Some(spell(&sets, ruleset, value))
}

/// Largest power of `radix` not exceeding `n` (the divisor of an RBNF rule).
/// `radix < 2` would never make progress, so it is treated as no division.
fn pow_le(n: i64, radix: i64) -> i64 {
    if radix < 2 {
        return 1;
    }
    let mut p = 1;
    while p <= n / radix {
        p *= radix;
    }
    p
}

/// Recursion-depth cap. Bounds the call-stack so a non-reducing rule cannot
/// overflow it (kept small enough for a ~1 MB stack, e.g. Windows). Real
/// spell-out of an `i64` nests only ~50 deep.
const MAX_DEPTH: u32 = 160;

fn spell(sets: &[RuleSet], ruleset: &str, value: i64) -> String {
    // Two guards make this total on adversarial input: `depth` bounds the call
    // stack (a non-reducing rule can't overflow it), and `budget` bounds the
    // total node count (a branching `←…→` rule can't explode combinatorially).
    // Spelling any real `i64` stays well within both.
    let mut budget: u32 = 50_000;
    spell_d(sets, ruleset, value, 0, &mut budget)
}

fn spell_d(sets: &[RuleSet], ruleset: &str, value: i64, depth: u32, budget: &mut u32) -> String {
    if depth >= MAX_DEPTH || *budget == 0 {
        return value.to_string();
    }
    *budget -= 1;
    let Some(rs) = sets.iter().find(|s| s.name == ruleset) else {
        return value.to_string();
    };
    if value < 0
        && let Some((_, text)) = rs.rules.iter().find(|(k, _)| k == "-x")
    {
        // `unsigned_abs()` is a u64; clamp before the i64 cast so i64::MIN
        // (whose magnitude is i64::MAX + 1) cannot wrap back to a negative
        // value and re-enter the `-x` rule forever.
        let abs = value.unsigned_abs().min(i64::MAX as u64) as i64;
        return render(sets, &rs.name, text, abs, 0, depth, budget);
    }
    // The applicable rule has the greatest numeric base ≤ value. A rule key may
    // be `base/radix` (e.g. `60/20` for French vigesimal); the radix governs the
    // divisor.
    let mut best: Option<(i64, i64, &str)> = None;
    for (k, t) in &rs.rules {
        let (base_str, radix) = match k.split_once('/') {
            Some((b, r)) => (b, r.parse::<i64>().unwrap_or(10).max(2)),
            None => (k.as_str(), 10),
        };
        if let Ok(base) = base_str.parse::<i64>()
            && base <= value
            && best.is_none_or(|(b, ..)| base > b)
        {
            best = Some((base, radix, t));
        }
    }
    let Some((base, radix, text)) = best else {
        return value.to_string();
    };
    let divisor = if base >= 1 { pow_le(base, radix) } else { 1 };
    render(sets, &rs.name, text, value, divisor, depth, budget)
}

/// Read an RBNF substitution token starting at `chars[i]` (a `←`, `→`, or `=`).
/// Returns the target rule-set name and the index just past the token.
fn parse_token(chars: &[char], i: usize, delim: char, current: &str) -> (String, usize) {
    if chars.get(i + 1) == Some(&delim) {
        return (current.to_string(), i + 2);
    }
    if chars.get(i + 1) == Some(&'%') {
        let mut j = i + 2;
        let mut name = String::new();
        while j < chars.len() && chars[j] != delim {
            name.push(chars[j]);
            j += 1;
        }
        // A leading `%` remains for `%%private` rule-set references; drop it.
        return (name.trim_start_matches('%').to_string(), j + 1);
    }
    (current.to_string(), i + 1)
}

fn render(
    sets: &[RuleSet],
    current: &str,
    text: &str,
    value: i64,
    divisor: i64,
    depth: u32,
    budget: &mut u32,
) -> String {
    let quot = if divisor == 0 { value } else { value / divisor };
    let rem = if divisor == 0 { value } else { value % divisor };
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut skipping = false; // inside an omitted optional `[ ]`
    while i < chars.len() {
        let c = chars[i];
        match c {
            ';' => break,
            '[' => {
                // Omit the bracketed part when the value is a multiple of the divisor.
                skipping = divisor != 0 && value % divisor == 0;
                i += 1;
            }
            ']' => {
                skipping = false;
                i += 1;
            }
            _ if skipping => i += 1,
            '←' => {
                let (rs, ni) = parse_token(&chars, i, '←', current);
                out.push_str(&spell_d(sets, &rs, quot, depth + 1, budget));
                i = ni;
            }
            '→' => {
                let (rs, ni) = parse_token(&chars, i, '→', current);
                out.push_str(&spell_d(sets, &rs, rem, depth + 1, budget));
                i = ni;
            }
            '=' => {
                let (rs, ni) = parse_token(&chars, i, '=', current);
                out.push_str(&spell_d(sets, &rs, value, depth + 1, budget));
                i = ni;
            }
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}
