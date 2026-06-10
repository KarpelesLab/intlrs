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
    push_fmt2(&mut out, pat, a, b);
    out
}

/// Append `pat` with `{0}`→`a` and `{1}`→`b` to `out` (single pass, no alloc).
fn push_fmt2(out: &mut String, pat: &str, a: &str, b: &str) {
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
}

/// Split a connector pattern into the literal text before `{0}`, between `{0}`
/// and `{1}`, and after `{1}`. If a placeholder is absent the corresponding
/// slot is empty and its text is folded into an adjacent segment, which keeps
/// the linear assembly byte-identical to substituting it with an empty string.
fn split2(pat: &str) -> (&str, &str, &str) {
    let (pre, after0) = match pat.find("{0}") {
        Some(i) => (&pat[..i], &pat[i + 3..]),
        None => ("", pat),
    };
    let (mid, post) = match after0.find("{1}") {
        Some(i) => (&after0[..i], &after0[i + 3..]),
        None => (after0, ""),
    };
    (pre, mid, post)
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
            // Decompose each connector pattern into the literal segments around
            // its `{0}` (accumulated head) and `{1}` (next item) placeholders:
            //   pat = pre + "{0}" + mid + "{1}" + post
            // The recursive `fmt2(pattern, acc, item)` fold nests each head
            // inside the next pattern's `{0}` slot, so the final string is:
            //   pre_end · pre_mid·(n-3) · [start with i0,i1]
            //     · (mid_mid · i_k · post_mid for k=2..=n-2)
            //     · mid_end · i_{n-1} · post_end
            // We emit it left-to-right in a single pre-sized buffer (linear),
            // byte-identical to the previous O(N²) accumulator fold.
            let (pre_s, mid_s, post_s) = split2(p.start);
            let (pre_m, mid_m, post_m) = split2(p.middle);
            let (pre_e, mid_e, post_e) = split2(p.end);

            // Pre-size: every literal once, the per-middle literals (n-3 times),
            // and every item exactly once.
            let items_len: usize = items.iter().map(|s| s.len()).sum();
            let cap = pre_e.len()
                + post_e.len()
                + mid_e.len()
                + pre_s.len()
                + mid_s.len()
                + post_s.len()
                + (pre_m.len() + mid_m.len() + post_m.len()) * n.saturating_sub(3)
                + items_len;
            let mut out = String::with_capacity(cap);

            // Leading nested prefixes: end, then one per middle iteration.
            out.push_str(pre_e);
            for _ in 0..n - 3 {
                out.push_str(pre_m);
            }
            // Innermost head: the `start` pattern over the first two items.
            out.push_str(pre_s);
            out.push_str(items[0]);
            out.push_str(mid_s);
            out.push_str(items[1]);
            out.push_str(post_s);
            // Each middle item, in order, closing one nesting level at a time.
            for item in &items[2..n - 1] {
                out.push_str(mid_m);
                out.push_str(item);
                out.push_str(post_m);
            }
            // Final item via the `end` pattern.
            out.push_str(mid_e);
            out.push_str(items[n - 1]);
            out.push_str(post_e);
            out
        }
    }
}
