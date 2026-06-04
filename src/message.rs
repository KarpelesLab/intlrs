//! A pragmatic subset of ICU MessageFormat (UTS #35 messages). Requires the
//! `alloc` feature.
//!
//! Supports literal text, simple argument substitution (`{name}`), `plural` and
//! `selectordinal` (with `=N` exact matches and `#` for the formatted number),
//! and `select`. Nested sub-messages and the locale's plural rules / number
//! formatting are used throughout. ICU quoting/escaping is not implemented.
//!
//! ```
//! use intl::message::{format_message, Arg};
//! let pat = "{n, plural, one {# item} other {# items}}";
//! assert_eq!(format_message("en", pat, &[("n", Arg::Num(1.0))]), "1 item");
//! assert_eq!(format_message("en", pat, &[("n", Arg::Num(5.0))]), "5 items");
//!
//! let g = "{g, select, female {She} male {He} other {They}} replied";
//! assert_eq!(format_message("en", g, &[("g", Arg::Str("female"))]), "She replied");
//! ```

use crate::number::format_decimal;
use crate::plural::{ordinal_category, plural_category, PluralCategory, PluralOperands};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A MessageFormat argument value.
#[derive(Debug, Clone, Copy)]
pub enum Arg<'a> {
    /// A number (formatted via [`crate::number`]; selects plural categories).
    Num(f64),
    /// A string (substituted verbatim; selects `select` cases).
    Str(&'a str),
}

/// Format `pattern` with the named `args` in the conventions of `lang`.
#[must_use]
pub fn format_message(lang: &str, pattern: &str, args: &[(&str, Arg)]) -> String {
    let c: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    parse_message(&c, &mut i, lang, args, None)
}

fn cat_name(c: PluralCategory) -> &'static str {
    match c {
        PluralCategory::Zero => "zero",
        PluralCategory::One => "one",
        PluralCategory::Two => "two",
        PluralCategory::Few => "few",
        PluralCategory::Many => "many",
        PluralCategory::Other => "other",
    }
}

fn operands(v: f64) -> PluralOperands {
    if v % 1.0 == 0.0 && v.abs() < 1e15 {
        PluralOperands::from_int(v as i64)
    } else {
        PluralOperands::parse(&alloc::format!("{v}")).unwrap_or(PluralOperands::from_int(v as i64))
    }
}

fn skip_ws(c: &[char], i: &mut usize) {
    while *i < c.len() && c[*i].is_whitespace() {
        *i += 1;
    }
}

/// Read a token up to whitespace or one of `, { }`.
fn read_token(c: &[char], i: &mut usize) -> String {
    skip_ws(c, i);
    let mut s = String::new();
    while *i < c.len() && !matches!(c[*i], ',' | '{' | '}') && !c[*i].is_whitespace() {
        s.push(c[*i]);
        *i += 1;
    }
    s
}

/// Parse message text until the end or an unmatched `}`. `hash` is the number
/// substituted for `#` (inside a plural sub-message).
fn parse_message(
    c: &[char],
    i: &mut usize,
    lang: &str,
    args: &[(&str, Arg)],
    hash: Option<f64>,
) -> String {
    let mut out = String::new();
    while *i < c.len() && c[*i] != '}' {
        match c[*i] {
            '{' => {
                *i += 1;
                out.push_str(&parse_arg(c, i, lang, args));
            }
            '#' if hash.is_some() => {
                out.push_str(&format_decimal(lang, hash.unwrap()));
                *i += 1;
            }
            ch => {
                out.push(ch);
                *i += 1;
            }
        }
    }
    out
}

fn lookup<'a>(name: &str, args: &'a [(&str, Arg<'a>)]) -> Option<Arg<'a>> {
    args.iter().find(|(n, _)| *n == name).map(|(_, v)| *v)
}

/// Parse one `{...}` argument (the leading `{` already consumed) and return its
/// rendered text. Leaves `*i` just past the matching `}`.
fn parse_arg(c: &[char], i: &mut usize, lang: &str, args: &[(&str, Arg)]) -> String {
    let name = read_token(c, i);
    skip_ws(c, i);
    if *i >= c.len() || c[*i] == '}' {
        *i += 1; // consume '}'
        return match lookup(&name, args) {
            Some(Arg::Num(n)) => format_decimal(lang, n),
            Some(Arg::Str(s)) => s.to_string(),
            None => String::new(),
        };
    }
    *i += 1; // consume ','
    let kind = read_token(c, i);
    skip_ws(c, i);
    if *i < c.len() && c[*i] == ',' {
        *i += 1;
    }
    let value = lookup(&name, args);
    match kind.as_str() {
        "plural" | "selectordinal" => {
            parse_plural(c, i, lang, args, value, kind == "selectordinal")
        }
        "select" => parse_select(c, i, lang, args, value),
        _ => {
            scan_to_close(c, i);
            String::new()
        }
    }
}

/// Collect `selector {submessage}` pairs until the argument's closing `}`,
/// recording the start index of each sub-message.
fn collect_cases(c: &[char], i: &mut usize) -> Vec<(String, usize)> {
    let mut cases = Vec::new();
    loop {
        skip_ws(c, i);
        if *i >= c.len() || c[*i] == '}' {
            *i += 1; // consume the argument's closing '}'
            break;
        }
        let selector = read_token(c, i);
        skip_ws(c, i);
        if *i >= c.len() || c[*i] != '{' {
            break;
        }
        let start = *i + 1;
        skip_braced(c, i); // advance past the balanced {...}
        cases.push((selector, start));
    }
    cases
}

/// Render the chosen sub-message (parsing from its recorded start index).
fn render_case(
    c: &[char],
    cases: &[(String, usize)],
    selector: &str,
    lang: &str,
    args: &[(&str, Arg)],
    hash: Option<f64>,
) -> String {
    let start = cases
        .iter()
        .find(|(s, _)| s == selector)
        .or_else(|| cases.iter().find(|(s, _)| s == "other"))
        .map(|(_, start)| *start);
    match start {
        Some(s) => {
            let mut j = s;
            parse_message(c, &mut j, lang, args, hash)
        }
        None => String::new(),
    }
}

fn parse_plural(
    c: &[char],
    i: &mut usize,
    lang: &str,
    args: &[(&str, Arg)],
    value: Option<Arg>,
    ordinal: bool,
) -> String {
    let num = match value {
        Some(Arg::Num(n)) => n,
        _ => 0.0,
    };
    let cases = collect_cases(c, i);
    // An exact `=N` selector wins over the plural category.
    let exact = alloc::format!("={}", trim_num(num));
    let selector = if cases.iter().any(|(s, _)| *s == exact) {
        exact
    } else {
        let cat = if ordinal {
            ordinal_category(lang, &operands(num))
        } else {
            plural_category(lang, &operands(num))
        };
        cat_name(cat).to_string()
    };
    render_case(c, &cases, &selector, lang, args, Some(num))
}

fn parse_select(
    c: &[char],
    i: &mut usize,
    lang: &str,
    args: &[(&str, Arg)],
    value: Option<Arg>,
) -> String {
    let key = match value {
        Some(Arg::Str(s)) => s.to_string(),
        _ => "other".to_string(),
    };
    let cases = collect_cases(c, i);
    render_case(c, &cases, &key, lang, args, None)
}

/// Render an integer-valued `f64` without a trailing `.0` (for `=N` matching).
fn trim_num(n: f64) -> String {
    if n % 1.0 == 0.0 && n.abs() < 1e15 {
        (n as i64).to_string()
    } else {
        alloc::format!("{n}")
    }
}

/// Advance `*i` from a `{` past the matching `}` (balanced).
fn skip_braced(c: &[char], i: &mut usize) {
    let mut depth = 0;
    while *i < c.len() {
        match c[*i] {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    *i += 1;
                    return;
                }
            }
            _ => {}
        }
        *i += 1;
    }
}

/// Skip the remainder of a `{...}` argument body up to and past its close.
fn scan_to_close(c: &[char], i: &mut usize) {
    let mut depth = 1;
    while *i < c.len() && depth > 0 {
        match c[*i] {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        *i += 1;
    }
}
