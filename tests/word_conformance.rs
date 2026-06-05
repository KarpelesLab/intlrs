//! Official UAX #29 word boundary conformance (WordBreakTest.txt).
#![cfg(all(feature = "full", feature = "segmentation"))]

use intl::unicode::words;
use std::fs;
use std::path::PathBuf;

#[test]
fn word_break_test_conformance() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data/ucd/17.0.0/auxiliary/WordBreakTest.txt");
    let text = fs::read_to_string(&path).expect("read WordBreakTest.txt");

    let mut checked = 0usize;
    for (i, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let mut expected: Vec<String> = Vec::new();
        let mut current = String::new();
        for tok in line.split_whitespace() {
            match tok {
                "\u{00F7}" => {
                    if !current.is_empty() {
                        expected.push(core::mem::take(&mut current));
                    }
                }
                "\u{00D7}" => {}
                hex => current.push(char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap()),
            }
        }
        let input: String = expected.concat();
        let got: Vec<&str> = words(&input).collect();
        assert_eq!(
            got,
            expected.iter().map(String::as_str).collect::<Vec<_>>(),
            "line {}: {:04X?}",
            i + 1,
            input.chars().map(|c| c as u32).collect::<Vec<_>>(),
        );
        checked += 1;
    }
    assert!(checked > 1000, "only {checked} lines");
    eprintln!("word conformance: {checked} lines passed");
}
