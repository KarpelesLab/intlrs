//! UAX #9 conformance against BidiTest.txt — the exhaustive by-bidi-class suite
//! (complements BidiCharacterTest.txt, which is by specific codepoints). Each
//! bidi class is mapped to a representative character, the algorithm is run for
//! every paragraph direction in the line's bitset, and the resolved levels +
//! visual order are checked against @Levels / @Reorder.
#![cfg(all(feature = "bidi", feature = "alloc"))]
use intl::unicode::bidi::{process, Direction};
use std::fs;

fn class_char(c: &str) -> Option<char> {
    Some(match c {
        "L" => 'A',
        "R" => '\u{05D0}',
        "AL" => '\u{0627}',
        "EN" => '0',
        "ES" => '+',
        "ET" => '$',
        "AN" => '\u{0660}',
        "CS" => ',',
        "NSM" => '\u{0300}',
        "BN" => '\u{00AD}',
        "B" => '\u{2029}',
        "S" => '\u{0009}',
        "WS" => ' ',
        "ON" => '!',
        "LRE" => '\u{202A}',
        "LRO" => '\u{202D}',
        "RLE" => '\u{202B}',
        "RLO" => '\u{202E}',
        "PDF" => '\u{202C}',
        "LRI" => '\u{2066}',
        "RLI" => '\u{2067}',
        "FSI" => '\u{2068}',
        "PDI" => '\u{2069}',
        _ => return None,
    })
}

#[test]
fn bidi_test_txt_conformance() {
    let dir = fs::read_dir("data/ucd").unwrap();
    let ver = dir
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().into_string().unwrap())
        .next()
        .unwrap();
    let Ok(text) = fs::read_to_string(format!("data/ucd/{ver}/BidiTest.txt")) else {
        eprintln!("BidiTest.txt absent — skipping");
        return;
    };
    let mut levels: Vec<Option<u8>> = Vec::new();
    let mut reorder: Vec<usize> = Vec::new();
    let (mut checked, mut failures) = (0u64, 0u64);
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("@Levels:") {
            levels = rest
                .split_whitespace()
                .map(|t| {
                    if t == "x" {
                        None
                    } else {
                        Some(t.parse().unwrap())
                    }
                })
                .collect();
        } else if let Some(rest) = line.strip_prefix("@Reorder:") {
            reorder = rest
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect();
        } else if let Some((classes, bitset)) = line.split_once(';') {
            let s: String = classes
                .split_whitespace()
                .map(|c| class_char(c).unwrap())
                .collect();
            let bits: u8 = bitset.trim().parse().unwrap();
            for (bit, base) in [
                (1, None),
                (2, Some(Direction::LeftToRight)),
                (4, Some(Direction::RightToLeft)),
            ] {
                if bits & bit == 0 {
                    continue;
                }
                checked += 1;
                let info = process(&s, base);
                if info.levels != levels || info.visual_order != reorder {
                    if failures < 5 {
                        eprintln!("FAIL [{classes}] base={base:?}\n  exp levels {levels:?} order {reorder:?}\n  got levels {:?} order {:?}", info.levels, info.visual_order);
                    }
                    failures += 1;
                }
            }
        }
    }
    eprintln!(
        "BidiTest.txt: {}/{} cases pass",
        checked - failures,
        checked
    );
    assert_eq!(failures, 0, "{failures} of {checked} BidiTest cases failed");
}
