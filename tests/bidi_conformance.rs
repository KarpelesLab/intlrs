//! UAX #9 conformance against BidiCharacterTest.txt.
#![cfg(all(feature = "bidi", feature = "alloc"))]

use intl::unicode::bidi::{process, Direction};
use std::fs;
use std::path::PathBuf;

#[test]
fn bidi_character_test() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/ucd/17.0.0/BidiCharacterTest.txt");
    let Ok(text) = fs::read_to_string(&path) else {
        eprintln!("BidiCharacterTest.txt absent; skipping");
        return;
    };

    let mut checked = 0usize;
    let mut failures = 0usize;
    'lines: for raw in text.lines() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(';').collect();
        if cols.len() < 5 {
            continue;
        }
        // Codepoints -> chars (skip lines containing non-scalar values).
        let mut chars = String::new();
        for cp in cols[0].split_whitespace() {
            match u32::from_str_radix(cp, 16).ok().and_then(char::from_u32) {
                Some(c) => chars.push(c),
                None => continue 'lines,
            }
        }
        let base = match cols[1].trim() {
            "0" => Some(Direction::LeftToRight),
            "1" => Some(Direction::RightToLeft),
            _ => None,
        };
        let exp_para: u8 = cols[2].trim().parse().unwrap();
        let exp_levels: Vec<Option<u8>> = cols[3]
            .split_whitespace()
            .map(|t| {
                if t == "x" {
                    None
                } else {
                    Some(t.parse().unwrap())
                }
            })
            .collect();
        let exp_order: Vec<usize> = cols[4]
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();

        let info = process(&chars, base);
        let ok = info.paragraph_level == exp_para
            && info.levels == exp_levels
            && info.visual_order == exp_order;
        if !ok {
            failures += 1;
        }
        checked += 1;
    }
    let pass = checked - failures;
    eprintln!("bidi: {pass}/{checked} lines pass");
    // Full conformance: every BidiCharacterTest line must pass.
    assert_eq!(
        failures, 0,
        "{failures} of {checked} BidiCharacterTest lines failed"
    );
}
