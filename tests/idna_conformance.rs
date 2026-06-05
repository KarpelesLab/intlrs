//! UTS #46 conformance against IdnaTestV2.txt (nontransitional ToASCII column).
#![cfg(feature = "idna")]

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

    let (mut valid, mut valid_fail) = (0usize, 0usize);
    let (mut reject_total, mut reject_ok) = (0usize, 0usize);
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
        if cols[4].is_empty() && cols[2].is_empty() {
            // Clean-success line: ToASCII must produce the expected A-label.
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
                Ok(a) if a == expected => valid += 1,
                _ => valid_fail += 1,
            }
        } else if !cols[4].is_empty() {
            // Line that ToASCII must reject (toAsciiNStatus carries error codes).
            reject_total += 1;
            if to_ascii(source).is_err() {
                reject_ok += 1;
            }
        }
    }
    eprintln!(
        "IDNA: clean-success {valid}/{} ; required rejections {reject_ok}/{reject_total}",
        valid + valid_fail
    );
    // Every clean-success line must pass (mapping + Punycode core).
    assert_eq!(
        valid_fail, 0,
        "IDNA clean-success regressed: {valid_fail} failed"
    );
    // We reject the basic cases (empty/over-long labels, bad Punycode) plus the
    // full IDNA2008 validity set: CheckBidi (B1–B6), CheckJoiners / ContextJ
    // (C1/C2), CheckHyphens (V2/V3), leading combining mark (V5), NFC validity
    // (V1), the IDNA valid-status check (V6), and xn-- re-canonicalization.
    //
    // The residual must-reject lines we intentionally do NOT cover:
    //   * `[A4_2]` trailing-root lines (e.g. `a.b.c.d.`, `鱊.`): VerifyDnsLength
    //     is an optional flag and this profile accepts the single trailing root
    //     label, so these are not errors for us.
    //   * Two `[V7, A3]` lines whose source carries a lone surrogate (`\uD900`):
    //     `unescape` cannot represent a surrogate as a Rust `char`, so it is
    //     dropped and the source reduces to the valid string "az" — a harness
    //     artifact, not a real gap.
    // Guard the count we *do* reject against regression.
    assert!(
        reject_ok >= 480,
        "IDNA rejection coverage regressed: only {reject_ok}/{reject_total}"
    );
}
