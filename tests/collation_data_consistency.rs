//! Gate: every locale bundled in the committed CLDR collation table must produce
//! a tailoring that honors its own rule — no `<` relation may invert to Greater,
//! and no `=` relation may be non-Equal. (Equal-where-distinct, from canonically
//! decomposing letters or case-variant tertiary collapse, is a tolerated
//! granularity loss, not a wrong order.) This catches the class of bug where an
//! auto-vendored rule silently sorts text incorrectly.
//!
//! The lexer below deliberately mirrors the runtime parser's grammar rather than
//! scanning for `<`/`=` characters: the bundled table is the *unfiltered* CLDR
//! rule set, so it is full of `[before]` resets, `[reorder]`/`[import]` options,
//! `<*` star ranges, `'…'` quoting and `\uXXXX` escapes. A cruder tokenizer would
//! pair an anchor like `[before` against a letter and report inversions that
//! aren't there. Relations whose target the gate cannot verify from the rule text
//! alone are skipped rather than guessed at — see [`Rel::checkable`].
#![cfg(feature = "collation")]
use core::cmp::Ordering;
use intl::unicode::collate::Tailoring;

/// One `<`/`<<`/`<<<`/`=` step: `prev` relates to `target` at `level`
/// (0 = identity `=`, 1..=3 = primary/secondary/tertiary).
#[derive(Debug)]
struct Rel {
    level: u8,
    prev: String,
    target: String,
    /// False when the rule text alone does not pin down the resulting order, so
    /// a comparison would report a phantom inversion:
    ///
    /// * the first target after `&[before N]` sorts *below* its anchor by
    ///   definition, so `anchor < target` is expected to be Greater;
    /// * an expansion target (`&t<<<þ/h` — þ collates as `t` then `h`) compares
    ///   greater than its anchor at every level, by design;
    /// * a prefix-context target (`x|y`) carries an ordering the runtime does not
    ///   model at all.
    checkable: bool,
}

/// Lex a CLDR tailoring rule into its ordering steps.
fn relations(rule: &str) -> Vec<Rel> {
    let ch: Vec<char> = rule.chars().collect();
    let mut out = Vec::new();
    // The element the next relation is measured against, and whether the pending
    // reset was a `[before N]`.
    let mut prev: Option<String> = None;
    let mut after_before_reset = false;
    // Set while reading the element that follows a relation.
    let mut pending: Option<(u8, bool)> = None; // (level, star range)
    let mut i = 0;

    while i < ch.len() {
        match ch[i] {
            '#' => {
                while i < ch.len() && ch[i] != '\n' {
                    i += 1;
                }
            }
            c if c.is_whitespace() => i += 1,
            '&' => {
                i += 1;
                prev = None;
                after_before_reset = false;
                // A `[before N]` between the `&` and its anchor.
                let mut j = i;
                while j < ch.len() && ch[j].is_whitespace() {
                    j += 1;
                }
                if ch[j..].starts_with(&['[']) {
                    let (content, next) = bracket(&ch, j);
                    if content.trim_start().starts_with("before") {
                        after_before_reset = true;
                    }
                    i = next;
                }
                // The anchor itself is read by the element arm below.
                pending = Some((u8::MAX, false));
            }
            '[' => {
                // `[reorder …]`, `[import …]`, `[normalization …]`, … — options
                // that carry no pairwise ordering. `[import]` splices in another
                // locale's rules, which that locale's own entry already gates.
                let (_, next) = bracket(&ch, i);
                i = next;
                // An `&[last regular]`-style anchor leaves nothing to compare
                // against; drop the pending reset rather than pair with a bracket.
                if pending == Some((u8::MAX, false)) {
                    pending = None;
                    prev = None;
                }
            }
            '<' => {
                let mut level = 0u8;
                while i < ch.len() && ch[i] == '<' {
                    level += 1;
                    i += 1;
                }
                let star = i < ch.len() && ch[i] == '*';
                if star {
                    i += 1;
                }
                pending = Some((level.min(3), star));
            }
            '=' => {
                i += 1;
                let star = i < ch.len() && ch[i] == '*';
                if star {
                    i += 1;
                }
                pending = Some((0, star));
            }
            _ => {
                let (elems, expansion, context, next) = element(&ch, i);
                i = next;
                let Some((level, star)) = pending.take() else {
                    continue;
                };
                if level == u8::MAX {
                    // Reset anchor: it becomes `prev`, no relation of its own.
                    prev = elems.into_iter().next_back();
                    continue;
                }
                // A star run relates each element to the one before it; a plain
                // relation has exactly one element.
                for el in elems {
                    if let Some(p) = prev.take() {
                        out.push(Rel {
                            level,
                            prev: p,
                            target: el.clone(),
                            checkable: !expansion && !context && !after_before_reset,
                        });
                    }
                    after_before_reset = false;
                    prev = Some(el);
                    if !star {
                        break;
                    }
                }
            }
        }
    }
    out
}

/// The contents of the `[…]` starting at `open`, and the index just past its
/// matching `]` (nested brackets — `[optimize [a-z]]` — are balanced).
fn bracket(ch: &[char], open: usize) -> (String, usize) {
    let mut depth = 0usize;
    let mut i = open;
    let mut content = String::new();
    while i < ch.len() {
        match ch[i] {
            '[' => {
                depth += 1;
                if depth > 1 {
                    content.push('[');
                }
            }
            ']' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return (content, i);
                }
                content.push(']');
                continue;
            }
            c => content.push(c),
        }
        i += 1;
    }
    (content, i)
}

/// Read one element (or, for a star range, the expanded run of elements) starting
/// at `i`. Returns the elements, whether the element carried a `/` expansion or a
/// `|` prefix context, and the index just past it.
fn element(ch: &[char], mut i: usize) -> (Vec<String>, bool, bool, usize) {
    let mut cur = String::new();
    let mut run: Vec<String> = Vec::new();
    let mut expansion = false;
    let mut context = false;
    let mut range_pending = false;

    while i < ch.len() {
        match ch[i] {
            '&' | '<' | '=' | '[' | '#' => break,
            c if c.is_whitespace() => break,
            '/' | '|' => {
                // The rest of this token is an expansion / prefix context: consume
                // it so it is not mistaken for the next element.
                if ch[i] == '/' {
                    expansion = true;
                } else {
                    context = true;
                }
                i += 1;
                while i < ch.len()
                    && !matches!(ch[i], '&' | '<' | '=' | '[' | '#')
                    && !ch[i].is_whitespace()
                {
                    i += 1;
                }
                break;
            }
            '\'' => {
                i += 1;
                if i < ch.len() && ch[i] == '\'' {
                    cur.push('\'');
                    i += 1;
                    continue;
                }
                while i < ch.len() && ch[i] != '\'' {
                    cur.push(ch[i]);
                    i += 1;
                }
                i += 1; // closing quote
            }
            '\\' if i + 1 < ch.len() && (ch[i + 1] == 'u' || ch[i + 1] == 'U') => {
                let width = if ch[i + 1] == 'u' { 4 } else { 8 };
                let hex: String = ch[i + 2..(i + 2 + width).min(ch.len())].iter().collect();
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(c) => cur.push(c),
                    None => cur.push('\u{fffd}'),
                }
                i += 2 + width;
            }
            '-' if !cur.is_empty() => {
                // `<*a-z`: a range between the previous char and the next one.
                range_pending = true;
                run.push(core::mem::take(&mut cur));
                i += 1;
            }
            c => {
                // In a star run each character is its own element, except when a
                // range is open.
                if range_pending {
                    let from = run.pop().unwrap_or_default();
                    let start = from.chars().next_back().unwrap_or(c) as u32;
                    for cp in start..=(c as u32) {
                        if let Some(r) = char::from_u32(cp) {
                            run.push(r.to_string());
                        }
                    }
                    range_pending = false;
                } else {
                    cur.push(c);
                }
                i += 1;
            }
        }
    }
    if !cur.is_empty() {
        run.push(cur);
    }
    // A plain (non-star) relation's element is the whole token; a star run's
    // elements are its characters. The caller knows which, and a single-element
    // token reads the same either way — so only split multi-char tokens when a
    // range was expanded above.
    if run.len() == 1 {
        return (run, expansion, context, i);
    }
    (run, expansion, context, i)
}

/// Parse the committed `collation.json` (one `"lang": "rule",` per line).
fn bundled_rules(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim().trim_end_matches(',');
        let Some(rest) = line.strip_prefix('"') else {
            continue;
        };
        let Some(colon) = rest.find("\": \"") else {
            continue;
        };
        let lang = rest[..colon].to_string();
        let val = &rest[colon + 4..];
        let Some(val) = val.strip_suffix('"') else {
            continue;
        };
        out.push((lang, unescape(val)));
    }
    out
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut ch = s.chars();
    while let Some(c) = ch.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match ch.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('u') => {
                let hex: String = ch.by_ref().take(4).collect();
                if let Some(c) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    out.push(c);
                }
            }
            Some(c) => out.push(c),
            None => break,
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
    let rules = bundled_rules(&text);
    let mut failures = Vec::new();

    for (lang, rule) in &rules {
        let Some(t) = Tailoring::parse(rule) else {
            failures.push(format!("{lang}: rule failed to parse"));
            continue;
        };
        for rel in relations(rule) {
            if !rel.checkable {
                continue;
            }
            let ord = t.compare(&rel.prev, &rel.target);
            let inverted = match rel.level {
                0 => ord != Ordering::Equal,
                _ => ord == Ordering::Greater,
            };
            if inverted {
                failures.push(format!(
                    "{lang}: {:?} (rel {}) {:?} -> {ord:?}",
                    rel.prev, rel.level, rel.target
                ));
                break;
            }
        }
    }

    assert!(
        rules.len() >= 20,
        "expected the bundled CLDR rule set, got {}",
        rules.len()
    );
    assert!(
        failures.is_empty(),
        "{} of {} bundled CLDR collation rules sort incorrectly:\n{}",
        failures.len(),
        rules.len(),
        failures.join("\n")
    );
}
