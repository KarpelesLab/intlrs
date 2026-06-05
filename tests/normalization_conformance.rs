//! Official UAX #15 conformance: every line of `NormalizationTest.txt` is
//! checked against all four normalization forms. Requires the `full` tier (the
//! test data spans all planes); run with `--features full` or `--all-features`.
#![cfg(all(feature = "full", feature = "normalization"))]

use intl::unicode::{is_nfc, is_nfd, is_nfkc, is_nfkd, nfc, nfd, nfkc, nfkd};
use std::fs;
use std::path::PathBuf;

fn field(f: &str) -> String {
    f.split_whitespace()
        .map(|h| char::from_u32(u32::from_str_radix(h, 16).unwrap()).unwrap())
        .collect()
}

#[test]
fn normalization_test_txt_conformance() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/ucd/17.0.0/NormalizationTest.txt");
    let text = fs::read_to_string(&path).expect("read NormalizationTest.txt");

    let nfc_s = |s: &str| nfc(s.chars()).collect::<String>();
    let nfd_s = |s: &str| nfd(s.chars()).collect::<String>();
    let nfkc_s = |s: &str| nfkc(s.chars()).collect::<String>();
    let nfkd_s = |s: &str| nfkd(s.chars()).collect::<String>();

    let mut checked = 0usize;
    for (i, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap().trim();
        if line.is_empty() || line.starts_with('@') {
            continue;
        }
        let cols: Vec<String> = line.split(';').map(|c| field(c.trim())).collect();
        if cols.len() < 5 {
            continue;
        }
        let (c1, c2, c3, c4, c5) = (&cols[0], &cols[1], &cols[2], &cols[3], &cols[4]);
        let ln = i + 1;

        // c2 == toNFC(c1) == toNFC(c2) == toNFC(c3); c4 == toNFC(c4) == toNFC(c5)
        for x in [c1, c2, c3] {
            assert_eq!(&nfc_s(x), c2, "NFC mismatch at line {ln}");
        }
        for x in [c4, c5] {
            assert_eq!(&nfc_s(x), c4, "NFC mismatch at line {ln}");
        }
        // c3 == toNFD(c1) == toNFD(c2) == toNFD(c3); c5 == toNFD(c4) == toNFD(c5)
        for x in [c1, c2, c3] {
            assert_eq!(&nfd_s(x), c3, "NFD mismatch at line {ln}");
        }
        for x in [c4, c5] {
            assert_eq!(&nfd_s(x), c5, "NFD mismatch at line {ln}");
        }
        // c4 == toNFKC(c1..c5)
        for x in [c1, c2, c3, c4, c5] {
            assert_eq!(&nfkc_s(x), c4, "NFKC mismatch at line {ln}");
        }
        // c5 == toNFKD(c1..c5)
        for x in [c1, c2, c3, c4, c5] {
            assert_eq!(&nfkd_s(x), c5, "NFKD mismatch at line {ln}");
        }

        // Quick-check: each already-normalized column must report true for its
        // own form.
        assert!(is_nfc(c2.chars()), "is_nfc(NFC) false at line {ln}");
        assert!(is_nfd(c3.chars()), "is_nfd(NFD) false at line {ln}");
        assert!(is_nfkc(c4.chars()), "is_nfkc(NFKC) false at line {ln}");
        assert!(is_nfkd(c5.chars()), "is_nfkd(NFKD) false at line {ln}");

        checked += 1;
    }
    assert!(checked > 10_000, "expected many test lines, got {checked}");
    eprintln!("normalization conformance: {checked} lines passed");
}
