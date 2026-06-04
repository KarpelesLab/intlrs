//! Locale-aware list formatting (CLDR / UTS #35): joining items with the right
//! connectors, e.g. `"a, b, and c"` (English "and") or `"a, b o c"` (Spanish
//! "or"). Requires the `alloc` feature.
//!
//! ```
//! use intl::list::{format_list, ListStyle};
//! assert_eq!(format_list("en", &["a", "b", "c"], ListStyle::And), "a, b, and c");
//! assert_eq!(format_list("en", &["a", "b"], ListStyle::Or), "a or b");
//! assert_eq!(format_list("de", &["a", "b", "c"], ListStyle::And), "a, b und c");
//! ```

use alloc::string::{String, ToString};

pub use crate::cldr::{ListPatterns, ListSpec};

/// Whether a list is conjunctive ("and") or disjunctive ("or").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListStyle {
    /// Conjunction — "a, b, and c".
    And,
    /// Disjunction — "a, b, or c".
    Or,
}

/// Substitute `{0}`→`a` and `{1}`→`b` in a connector pattern (single pass).
fn fmt2(pat: &str, a: &str, b: &str) -> String {
    let mut out = String::with_capacity(pat.len() + a.len() + b.len());
    let mut rest = pat;
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix("{0}") {
            out.push_str(a);
            rest = r;
        } else if let Some(r) = rest.strip_prefix("{1}") {
            out.push_str(b);
            rest = r;
        } else {
            let ch = rest.chars().next().unwrap();
            out.push(ch);
            rest = &rest[ch.len_utf8()..];
        }
    }
    out
}

fn spec(lang: &str) -> ListSpec {
    use crate::cldr::list_spec;
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
    loop {
        if let Some(s) = list_spec(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return list_spec("en").expect("root list spec present"),
        }
    }
}

/// Join `items` into a single string using the list conventions of `lang`.
#[must_use]
pub fn format_list(lang: &str, items: &[&str], style: ListStyle) -> String {
    let s = spec(lang);
    let p = match style {
        ListStyle::And => s.and,
        ListStyle::Or => s.or,
    };
    match items.len() {
        0 => String::new(),
        1 => items[0].to_string(),
        2 => fmt2(p.two, items[0], items[1]),
        n => {
            let mut acc = fmt2(p.start, items[0], items[1]);
            for item in &items[2..n - 1] {
                acc = fmt2(p.middle, &acc, item);
            }
            fmt2(p.end, &acc, items[n - 1])
        }
    }
}
