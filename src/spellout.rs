//! Spelling integers out in words via the CLDR rule-based number format (RBNF).
//! Requires the `alloc` feature. Locale-driven (a curated set of locales), not
//! hardcoded for any language.
//!
//! ```
//! use intl::spellout::spell_cardinal;
//! assert_eq!(spell_cardinal("en", 1234).as_deref(), Some("one thousand two hundred thirty-four"));
//! assert_eq!(spell_cardinal("en", -42).as_deref(), Some("minus forty-two"));
//! assert_eq!(spell_cardinal("fr", 80).as_deref(), Some("quatre-vingts"));
//! assert_eq!(spell_cardinal("ru", 2000).as_deref(), Some("две тысячи"));
//! assert_eq!(spell_cardinal("ja", 1234).as_deref(), Some("千二百三十四"));
//! assert_eq!(spell_cardinal("xx", 1), None); // unsupported locale
//! ```
//!
//! The vendored locales are `en`, `de`, `fr`, `es`, `it`, `nl`, `pt`, `sv`,
//! `ru`, `uk`, `pl` and `ja`. Each carries CLDR's default (`%spellout-numbering`
//! or `%spellout-cardinal`) rule set plus its gender / number variants
//! (`%spellout-cardinal-feminine`, `%spellout-ordinal-masculine`, …), which
//! [`spell_rule_set`] reaches by name. Grammatical-case variants
//! (`-genitive`, `-dative`, …) and `%spellout-numbering-year` are not vendored,
//! except where another rule set refers to them.
//!
//! The engine implements the RBNF rule syntax CLDR uses for integers: rule
//! selection by base value (with `base/radix` and ICU's roll-back rule), the
//! `←←` / `→→` / `==` substitutions with named (`%…`) and private (`%%…`)
//! rule-set references or a decimal pattern (`=#,##0=`), `[optional]` and
//! `[then|else]` parts, `$(cardinal,…)$` / `$(ordinal,…)$` plural selection,
//! and the leading-apostrophe escape.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::plural::{PluralCategory, PluralOperands};

/// A named RBNF rule set (CLDR name with its `%` / `%%` prefix): an ordered list
/// of `(key, text)` rules.
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

/// A resolved locale: its data key (for plural rules and number symbols), the
/// default rule-set name, and its rule sets.
struct Rbnf {
    lang: String,
    start: String,
    sets: Vec<RuleSet>,
}

/// Resolve `lang` through the CLDR locale chain (`pt-BR` → `pt`) to its RBNF
/// rule sets.
fn resolve(lang: &str) -> Option<Rbnf> {
    let (key, bytes) = crate::cldr::fallback(lang, |k| {
        crate::cldr::rbnf_payload(k).map(|b| (k.to_string(), b))
    })?;
    let (start, sets) = parse_payload(bytes);
    Some(Rbnf {
        lang: key,
        start,
        sets,
    })
}

impl Rbnf {
    fn has(&self, name: &str) -> bool {
        self.sets.iter().any(|s| s.name == name)
    }

    fn spell(&self, ruleset: &str, value: i64) -> String {
        // Two guards make this total on adversarial input: `depth` bounds the
        // call stack (a non-reducing rule can't overflow it), and `budget`
        // bounds the total node count (a branching `←…→` rule can't explode
        // combinatorially). Spelling any real `i64` stays well within both.
        let mut cx = Cx {
            rb: self,
            budget: 50_000,
        };
        cx.spell(ruleset, value, 0)
    }
}

/// Spell `value` out as a cardinal number in `lang`, e.g.
/// `spell_cardinal("en", 1234)` → `"one thousand two hundred thirty-four"`.
/// Uses the locale's default rule set (`%spellout-numbering` where CLDR has
/// one, which for gendered languages is the masculine cardinal). Returns `None`
/// for an unsupported locale.
#[must_use]
pub fn spell_cardinal(lang: &str, value: i64) -> Option<String> {
    let rb = resolve(lang)?;
    Some(rb.spell(&rb.start, value))
}

/// Spell `value` out as an ordinal number in `lang`, e.g.
/// `spell_ordinal("en", 21)` → `"twenty-first"`. Where a language defines only
/// gendered ordinals (Spanish, Italian, Russian, …), the masculine form is
/// used. Returns `None` for a locale without ordinal spell-out rules.
///
/// ```
/// use intl::spellout::spell_ordinal;
/// assert_eq!(spell_ordinal("en", 1).as_deref(), Some("first"));
/// assert_eq!(spell_ordinal("en", 23).as_deref(), Some("twenty-third"));
/// assert_eq!(spell_ordinal("es", 1).as_deref(), Some("primero"));
/// assert_eq!(spell_ordinal("ru", 2).as_deref(), Some("второй"));
/// assert_eq!(spell_ordinal("xx", 1), None); // unsupported locale
/// ```
#[must_use]
pub fn spell_ordinal(lang: &str, value: i64) -> Option<String> {
    let rb = resolve(lang)?;
    let ruleset = ["%spellout-ordinal", "%spellout-ordinal-masculine"]
        .into_iter()
        .find(|r| rb.has(r))?;
    Some(rb.spell(ruleset, value))
}

/// Spell `value` with a named public RBNF rule set of `lang` — the analog of
/// ICU's `RuleBasedNumberFormat::format(number, ruleSetName)` / PHP's
/// `NumberFormatter::DEFAULT_RULESET`. The name may be given with or without
/// its leading `%` (`"%spellout-cardinal-feminine"` or
/// `"spellout-cardinal-feminine"`). Returns `None` for an unsupported locale,
/// a rule set the locale does not have (or that is not vendored), or a private
/// (`%%…`) rule set.
///
/// ```
/// use intl::spellout::spell_rule_set;
/// assert_eq!(spell_rule_set("ru", "spellout-cardinal-feminine", 2).as_deref(), Some("две"));
/// assert_eq!(spell_rule_set("pl", "%spellout-cardinal-masculine-personal", 2).as_deref(), Some("dwaj"));
/// assert_eq!(spell_rule_set("ja", "spellout-cardinal-financial", 12).as_deref(), Some("拾弐"));
/// assert_eq!(spell_rule_set("ru", "spellout-cardinal-bogus", 2), None);
/// ```
#[must_use]
pub fn spell_rule_set(lang: &str, rule_set: &str, value: i64) -> Option<String> {
    if rule_set.starts_with("%%") {
        return None;
    }
    let mut name = String::with_capacity(rule_set.len() + 1);
    name.push('%');
    name.push_str(rule_set.strip_prefix('%').unwrap_or(rule_set));
    let rb = resolve(lang)?;
    rb.has(&name).then(|| rb.spell(&name, value))
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

/// One numeric rule: base value, divisor and text.
struct Rule<'a> {
    base: i64,
    divisor: i64,
    text: &'a str,
}

/// Formatting state: the locale and the remaining node budget.
struct Cx<'a> {
    rb: &'a Rbnf,
    budget: u32,
}

impl Cx<'_> {
    fn spell(&mut self, ruleset: &str, value: i64, depth: u32) -> String {
        if depth >= MAX_DEPTH || self.budget == 0 {
            return value.to_string();
        }
        self.budget -= 1;
        let rb = self.rb;
        let Some(rs) = rb.sets.iter().find(|s| s.name == ruleset) else {
            return value.to_string();
        };
        if value < 0
            && let Some((_, text)) = rs.rules.iter().find(|(k, _)| k == "-x")
        {
            // `unsigned_abs()` is a u64; clamp before the i64 cast so i64::MIN
            // (whose magnitude is i64::MAX + 1) cannot wrap back to a negative
            // value and re-enter the `-x` rule forever.
            let abs = value.unsigned_abs().min(i64::MAX as u64) as i64;
            return self.render(&rs.name, text, abs, 0, depth);
        }
        // Numeric rules in base order. A key may be `base/radix` (e.g. `60/20`
        // for French vigesimal); the radix governs the divisor.
        let mut rules: Vec<Rule<'_>> = rs
            .rules
            .iter()
            .filter_map(|(k, t)| {
                let (base_str, radix) = match k.split_once('/') {
                    Some((b, r)) => (b, r.parse::<i64>().unwrap_or(10).max(2)),
                    None => (k.as_str(), 10),
                };
                let base = base_str.parse::<i64>().ok()?;
                let divisor = if base >= 1 { pow_le(base, radix) } else { 1 };
                Some(Rule {
                    base,
                    divisor,
                    text: t,
                })
            })
            .collect();
        rules.sort_by_key(|r| r.base);
        // The applicable rule has the greatest base ≤ value. Without a `-x`
        // rule a negative value picks its rule by magnitude but is passed on
        // as is, so `0: =%spellout-cardinal-masculine=;` still reaches that
        // rule set's own `-x` rule (as ICU does).
        let magnitude = value.unsigned_abs().min(i64::MAX as u64) as i64;
        let Some(mut idx) = rules.iter().rposition(|r| r.base <= magnitude) else {
            return value.to_string();
        };
        // ICU's roll-back rule: a rule with a modulus (`→`) substitution whose
        // base is not a multiple of its divisor yields to the previous rule for
        // an exact multiple of the divisor (so 200 never reads "two hundred
        // zero" off a `101:` rule).
        let r = &rules[idx];
        if idx > 0 && r.text.contains('→') && magnitude % r.divisor == 0 && r.base % r.divisor != 0
        {
            idx -= 1;
        }
        let r = &rules[idx];
        // Only a whole-value `=…=` forwards the sign; a rule that divides the
        // value (`←…←` / `→…→`) works on its magnitude.
        let v = if r.text.contains(['←', '→']) {
            magnitude
        } else {
            value
        };
        self.render(&rs.name, r.text, v, r.divisor, depth)
    }

    fn render(
        &mut self,
        current: &str,
        text: &str,
        value: i64,
        divisor: i64,
        depth: u32,
    ) -> String {
        let quot = if divisor == 0 { value } else { value / divisor };
        let rem = if divisor == 0 { value } else { value % divisor };
        // A leading apostrophe only protects leading whitespace; drop it.
        let text = text.strip_prefix('\'').unwrap_or(text);
        let text = text.split(';').next().unwrap_or("");
        let chars = select_optional(text, divisor != 0 && value % divisor == 0);
        let mut out = String::new();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            match c {
                '←' | '→' | '=' => {
                    let (target, ni) = parse_token(&chars, i, c);
                    let n = match c {
                        '←' => quot,
                        '→' => rem,
                        _ => value,
                    };
                    match target {
                        Target::Current => out.push_str(&self.spell(current, n, depth + 1)),
                        Target::Named(rs) => out.push_str(&self.spell(&rs, n, depth + 1)),
                        Target::Pattern(grouped) => self.decimal(&mut out, n, grouped),
                    }
                    i = ni;
                }
                '$' if chars.get(i + 1) == Some(&'(') => {
                    i = self.plural(&chars, i + 2, quot, &mut out);
                }
                _ => {
                    out.push(c);
                    i += 1;
                }
            }
        }
        out
    }

    /// Expand a `$(cardinal,one{…}other{…})$` / `$(ordinal,…)$` substitution
    /// whose body starts at `chars[i]`, selecting on `n` (the rule's quotient,
    /// as ICU does). Returns the index just past the closing `)$`.
    fn plural(&self, chars: &[char], mut i: usize, n: i64, out: &mut String) -> usize {
        let mut kind = String::new();
        while i < chars.len() && chars[i] != ',' && chars[i] != ')' {
            kind.push(chars[i]);
            i += 1;
        }
        let ops = PluralOperands::from_int(n);
        let lang = self.rb.lang.as_str();
        let cat = if kind == "ordinal" {
            crate::plural::ordinal_category(lang, &ops)
        } else {
            crate::plural::plural_category(lang, &ops)
        };
        let want = match cat {
            PluralCategory::Zero => "zero",
            PluralCategory::One => "one",
            PluralCategory::Two => "two",
            PluralCategory::Few => "few",
            PluralCategory::Many => "many",
            PluralCategory::Other => "other",
        };
        let (mut hit, mut other) = (None, None);
        if chars.get(i) == Some(&',') {
            i += 1;
        }
        while i < chars.len() && chars[i] != ')' {
            let mut key = String::new();
            while i < chars.len() && chars[i] != '{' && chars[i] != ')' {
                key.push(chars[i]);
                i += 1;
            }
            if chars.get(i) != Some(&'{') {
                break;
            }
            let body_start = i + 1;
            i = body_start;
            while i < chars.len() && chars[i] != '}' {
                i += 1;
            }
            let body = (body_start, i.min(chars.len()));
            i += 1; // past `}`
            let key = key.trim();
            if key == want {
                hit = Some(body);
            } else if key == "other" {
                other = Some(body);
            }
        }
        if let Some((a, b)) = hit.or(other) {
            out.extend(&chars[a..b]);
        }
        // Past `)` and the closing `$`.
        i += 1;
        if chars.get(i) == Some(&'$') {
            i += 1;
        }
        i
    }

    /// Format `n` for a decimal-pattern substitution (`=#,##0=`, `=0=`): ASCII
    /// digits, grouped by three with the locale's separator when the pattern
    /// has a `,`.
    fn decimal(&self, out: &mut String, n: i64, grouped: bool) {
        if n < 0 {
            out.push('-');
        }
        let digits = n.unsigned_abs().to_string();
        let (sep, min_grouping) = group_symbol(&self.rb.lang);
        if !grouped || digits.len() < 3 + min_grouping {
            out.push_str(&digits);
            return;
        }
        for (k, d) in digits.chars().enumerate() {
            if k > 0 && (digits.len() - k).is_multiple_of(3) {
                out.push_str(sep);
            }
            out.push(d);
        }
    }
}

/// The locale's grouping separator and minimum grouping digits (`,` / 1 when
/// the `number` feature's symbol data is not compiled in).
fn group_symbol(lang: &str) -> (&'static str, usize) {
    #[cfg(feature = "number")]
    if let Some(s) = crate::cldr::number_spec(lang, "latn") {
        return (s.group, usize::from(s.min_grouping.max(1)));
    }
    let _ = lang;
    (",", 1)
}

/// Resolve the `[optional]` and `[then|else]` parts of a rule's text. `omit`
/// (the value is an exact multiple of the divisor) drops an optional part, or
/// picks the `else` branch of a two-way one.
fn select_optional(text: &str, omit: bool) -> Vec<char> {
    let mut out = Vec::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '[' {
            out.push(c);
            continue;
        }
        let inner: String = chars.by_ref().take_while(|&c| c != ']').collect();
        let (then, els) = inner.split_once('|').unwrap_or((&inner, ""));
        out.extend(if omit { els } else { then }.chars());
    }
    out
}

/// What a substitution token formats with.
enum Target {
    /// The rule's own rule set.
    Current,
    /// A rule set by name (with its `%` / `%%` prefix).
    Named(String),
    /// A decimal pattern; `true` when it groups digits (`#,##0`).
    Pattern(bool),
}

/// Read an RBNF substitution token starting at `chars[i]` (a `←`, `→`, or `=`).
/// Returns its target and the index just past the token.
fn parse_token(chars: &[char], i: usize, delim: char) -> (Target, usize) {
    match chars.get(i + 1) {
        Some(&c) if c == delim => {
            // `→→→` (by-digit fraction form) carries a third delimiter.
            let end = if chars.get(i + 2) == Some(&delim) {
                i + 3
            } else {
                i + 2
            };
            (Target::Current, end)
        }
        Some('%' | '#' | '0') => {
            let mut j = i + 1;
            let mut inner = String::new();
            while j < chars.len() && chars[j] != delim {
                inner.push(chars[j]);
                j += 1;
            }
            let target = if inner.starts_with('%') {
                Target::Named(inner)
            } else {
                let int_part = inner.split('.').next().unwrap_or("");
                Target::Pattern(int_part.contains(','))
            };
            (target, j + 1)
        }
        _ => (Target::Current, i + 1),
    }
}
