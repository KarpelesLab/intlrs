//! Official UCA conformance (UTS #10): each `CollationTest_*.txt` file is a
//! list of strings in non-decreasing collation order. We verify that every
//! consecutive pair compares as non-decreasing under the matching collator.
//! Requires the `alloc` feature (which implies `full`).
#![cfg(feature = "collation")]

use intl::unicode::collate::{AlternateHandling, Collator};
use std::fs;
use std::path::PathBuf;

fn run(file: &str, alternate: AlternateHandling) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data/uca/17.0.0/CollationTest")
        .join(file);
    // The ~40 MB CollationTest files are not committed; fetched on demand (see
    // the `conformance` CI job). Skip gracefully when they are absent.
    let Ok(text) = fs::read_to_string(&path) else {
        eprintln!("skipping {file}: not present (fetch UCA CollationTest to run)");
        return;
    };
    let collator = Collator::new(alternate);

    let mut prev_str = String::new();
    let mut prev_key: Vec<u16> = Vec::new();
    let mut checked = 0usize;

    for (i, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() || line.starts_with('@') {
            continue;
        }
        let cps: Vec<u32> = line
            .split(';')
            .next()
            .unwrap()
            .split_whitespace()
            .map(|h| u32::from_str_radix(h, 16).unwrap())
            .collect();
        // A Rust string cannot contain surrogate code points, so the conformance
        // lines that test lone surrogates are inapplicable to a `&str` API.
        if cps.iter().any(|&c| (0xD800..=0xDFFF).contains(&c)) {
            continue;
        }
        let cur: String = cps.iter().map(|&c| char::from_u32(c).unwrap()).collect();
        let cur_key = collator.sort_key(&cur);

        if checked > 0 {
            assert!(
                prev_key <= cur_key,
                "{file} line {}: {:04X?} must sort <= {:04X?}\n  prev key {:04X?}\n  cur  key {:04X?}",
                i + 1,
                prev_str.chars().map(|c| c as u32).collect::<Vec<_>>(),
                cur.chars().map(|c| c as u32).collect::<Vec<_>>(),
                prev_key,
                cur_key,
            );
        }
        prev_str = cur;
        prev_key = cur_key;
        checked += 1;
    }
    assert!(checked > 10_000, "{file}: only {checked} lines");
    eprintln!("{file}: {checked} lines in order");
}

#[test]
fn non_ignorable_conformance() {
    run(
        "CollationTest_NON_IGNORABLE.txt",
        AlternateHandling::NonIgnorable,
    );
}

#[test]
fn shifted_conformance() {
    run("CollationTest_SHIFTED.txt", AlternateHandling::Shifted);
}
