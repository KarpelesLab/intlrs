//! Gate: every locale bundled in the committed CLDR collation table must produce
//! a tailoring that honors its own rule — no `<` relation may invert to Greater,
//! and no `=` relation may be non-Equal. (Equal-where-distinct, from canonically
//! decomposing letters or case-variant tertiary collapse, is a tolerated
//! granularity loss, not a wrong order.) This catches the class of bug where an
//! auto-vendored rule silently sorts text incorrectly.
#![cfg(feature = "collation")]
use core::cmp::Ordering;
use intl::unicode::collate::Tailoring;

fn tokens(rule: &str) -> Vec<(u8, String)> {
    let ch: Vec<char> = rule.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < ch.len() {
        let c = ch[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        let rel = match c {
            '&' => {
                i += 1;
                0
            }
            '=' => {
                i += 1;
                4
            }
            '<' => {
                let mut n = 0u8;
                while i < ch.len() && ch[i] == '<' {
                    n += 1;
                    i += 1;
                }
                n.min(3)
            }
            _ => {
                i += 1;
                continue;
            }
        };
        let s = i;
        while i < ch.len() && !matches!(ch[i], '&' | '<' | '=' | ' ' | '\t') {
            i += 1;
        }
        let el: String = ch[s..i].iter().collect();
        if !el.is_empty() {
            out.push((rel, el));
        }
    }
    out
}

#[test]
fn bundled_cldr_rules_have_no_inversions() {
    let text = match std::fs::read_to_string("data/cldr/48/collation.json") {
        Ok(t) => t,
        Err(_) => return, // data dir not present (e.g. from a published tarball)
    };
    let mut failures = Vec::new();
    let mut checked = 0;
    for entry in text.trim_matches(|c| c == '{' || c == '}').split("\",\"") {
        let Some((k, v)) = entry.split_once("\":\"") else {
            continue;
        };
        let lang = k.trim_matches('"');
        let rule = v.trim_matches('"');
        checked += 1;
        let t = Tailoring::parse(rule).unwrap_or_else(|| panic!("{lang}: rule failed to parse"));
        let mut prev: Option<String> = None;
        for (rel, el) in tokens(rule) {
            if rel == 0 {
                prev = Some(el);
                continue;
            }
            if let Some(p) = &prev {
                let ord = t.compare(p, &el);
                let inverted = (1..=3).contains(&rel) && ord == Ordering::Greater
                    || (rel == 4 && ord != Ordering::Equal);
                if inverted {
                    failures.push(format!("{lang}: {p:?} (rel {rel}) {el:?} -> {ord:?}"));
                    break;
                }
            }
            prev = Some(el);
        }
    }
    assert!(
        checked >= 20,
        "expected the bundled CLDR rule set, got {checked}"
    );
    assert!(
        failures.is_empty(),
        "bundled CLDR collation rules that sort incorrectly:\n{}",
        failures.join("\n")
    );
}
