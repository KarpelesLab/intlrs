//! In-process fuzzing: run every public algorithm over a large corpus of
//! pseudo-random strings, asserting it never panics and that key invariants
//! hold (round-trip, idempotence, ordering consistency). Deterministic, so it
//! runs in normal CI; complements the official conformance suites.
#![cfg(feature = "alloc")]

use intl::unicode::{
    case_fold, collate, graphemes, line_breaks, nfc, nfd, nfkc, nfkd, sentences, to_lowercase,
    to_uppercase, words,
};

/// Tiny deterministic xorshift PRNG.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// A random scalar value, biased toward "interesting" ranges (ASCII, Latin
    /// combining marks, CJK, Hangul, emoji, format controls).
    fn scalar(&mut self) -> char {
        let r = self.next();
        let cp = match r % 16 {
            0..=4 => r % 0x80,           // ASCII
            5..=7 => 0x300 + r % 0x200,  // combining marks / Greek/Cyrillic
            8..=9 => 0x1100 + r % 0x100, // Hangul jamo
            10 => 0xAC00 + r % 0x2BA4,   // Hangul syllables
            11 => 0x4E00 + r % 0x5000,   // CJK
            12 => 0x1F300 + r % 0x400,   // emoji
            13 => 0x200C + r % 4,        // ZWNJ/ZWJ
            _ => r % 0x11000,            // anything in low planes
        } as u32;
        char::from_u32(cp).unwrap_or('\u{FFFD}')
    }
    fn string(&mut self, len: usize) -> String {
        (0..len).map(|_| self.scalar()).collect()
    }
}

#[test]
fn fuzz_invariants() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    for _ in 0..20_000 {
        let len = (rng.next() % 12) as usize;
        let s = rng.string(len);

        // Normalization: forms reconstruct, are idempotent, and respect the
        // NFKC ⊇ NFC relationship by canonical equivalence (NFD of either equal).
        let nfc_s: String = nfc(s.chars()).collect();
        let nfd_s: String = nfd(s.chars()).collect();
        let nfkc_s: String = nfkc(s.chars()).collect();
        let nfkd_s: String = nfkd(s.chars()).collect();
        assert_eq!(
            nfc(nfc_s.chars()).collect::<String>(),
            nfc_s,
            "NFC idempotent: {s:?}"
        );
        assert_eq!(
            nfd(nfd_s.chars()).collect::<String>(),
            nfd_s,
            "NFD idempotent: {s:?}"
        );
        assert_eq!(
            nfkc(nfkc_s.chars()).collect::<String>(),
            nfkc_s,
            "NFKC idempotent: {s:?}"
        );
        assert_eq!(
            nfkd(nfkd_s.chars()).collect::<String>(),
            nfkd_s,
            "NFKD idempotent: {s:?}"
        );
        // NFD(NFC(x)) == NFD(x): NFC preserves canonical equivalence.
        assert_eq!(
            nfd(nfc_s.chars()).collect::<String>(),
            nfd_s,
            "NFC keeps canonical equivalence: {s:?}"
        );

        // Segmentation: the pieces partition the input exactly.
        assert_eq!(
            graphemes(&s).collect::<String>(),
            s,
            "graphemes reconstruct: {s:?}"
        );
        assert_eq!(words(&s).collect::<String>(), s, "words reconstruct: {s:?}");
        assert_eq!(
            sentences(&s).collect::<String>(),
            s,
            "sentences reconstruct: {s:?}"
        );
        assert_eq!(
            line_breaks(&s).map(|b| b.text).collect::<String>(),
            s,
            "line breaks reconstruct: {s:?}"
        );

        // Case mapping never panics; fold is idempotent.
        let _: String = to_uppercase('x')
            .chain(s.chars().flat_map(to_uppercase))
            .collect();
        let _: String = s.chars().flat_map(to_lowercase).collect();
        let folded: String = s.chars().flat_map(case_fold).collect();
        let folded2: String = folded.chars().flat_map(case_fold).collect();
        assert_eq!(folded, folded2, "case fold idempotent: {s:?}");

        // Collation is reflexive and antisymmetric.
        assert_eq!(
            collate::compare(&s, &s),
            core::cmp::Ordering::Equal,
            "collate reflexive: {s:?}"
        );
        let tlen = (rng.next() % 8) as usize;
        let t = rng.string(tlen);
        assert_eq!(
            collate::compare(&s, &t),
            collate::compare(&t, &s).reverse(),
            "collate antisymmetric: {s:?} vs {t:?}"
        );
    }
}
