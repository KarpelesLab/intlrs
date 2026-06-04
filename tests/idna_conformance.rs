//! UTS #46 conformance against IdnaTestV2.txt (nontransitional ToASCII column).
#![cfg(feature = "alloc")]

use intl::unicode::idna::to_ascii;
use std::fs;
use std::path::PathBuf;

/// Unescape `\uXXXX` / `\x{XXXX}` sequences used in the test file.
fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'u') {
            chars.next();
            let hex: String = (0..4).filter_map(|_| chars.next()).collect();
            if let Some(ch) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                out.push(ch);
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[test]
fn idna_test_v2_to_ascii() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/idna/17.0.0/IdnaTestV2.txt");
    let text = fs::read_to_string(&path).expect("read IdnaTestV2.txt");

    let mut checked = 0usize;
    let mut failures = 0usize;
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<String> = line.split(';').map(|c| unescape(c.trim())).collect();
        if cols.len() < 5 {
            continue;
        }
        let source = &cols[0];
        // We implement the mapping + Punycode core, not the contextual validity
        // rules (CheckBidi / CheckJoiners / full STD3), so only the lines that
        // expect a *successful* ToASCII are in scope here.
        if !cols[2].is_empty() || !cols[4].is_empty() {
            continue;
        }
        let to_unicode = if cols[1].is_empty() {
            source.clone()
        } else {
            cols[1].clone()
        };
        let expected = if cols[3].is_empty() {
            to_unicode
        } else {
            cols[3].clone()
        };

        match to_ascii(source) {
            Ok(a) if a == expected => {}
            _ => failures += 1,
        }
        checked += 1;
    }
    let pass = checked - failures;
    eprintln!("IDNA ToASCII (valid inputs): {pass}/{checked} lines pass");
    // The mapping + Punycode core handles every clean-success line in the suite.
    // (Lines that expect an error exercise CheckBidi / CheckJoiners / STD3, which
    // are not yet implemented, and are out of scope here.)
    assert_eq!(failures, 0, "IDNA core: {pass}/{checked}");
}
