//! In-process fuzzing: run every public algorithm over a large corpus of
//! pseudo-random strings, asserting it never panics and that key invariants
//! hold (round-trip, idempotence, ordering consistency). Deterministic, so it
//! runs in normal CI; complements the official conformance suites.
#![cfg(all(
    feature = "collation",
    feature = "idna",
    feature = "confusables",
    feature = "bidi"
))]

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

        // Bidi (the algorithm with the explicit-level stack), IDNA, and spoof
        // analysis must never panic on arbitrary text.
        let info = intl::unicode::bidi::process(&s, None);
        assert_eq!(
            info.levels.len(),
            s.chars().count(),
            "bidi level count: {s:?}"
        );
        assert_eq!(
            intl::unicode::lowercase_str(&s).chars().count(),
            intl::unicode::lowercase(s.chars()).count(),
            "lowercase_str length matches char map: {s:?}"
        );
        let _ = intl::unicode::lowercase_str_lang(&s, "tr");
        let _ = intl::unicode::lowercase_str_lang(&s, "lt");
        let _ = intl::unicode::uppercase_str_lang(&s, "az");
        let _ = intl::translit::latin_ascii(&s);
        let _ = intl::translit::remove_diacritics(&s);
        let _ = intl::translit::cyrillic_to_latin(&s);
        let _ = intl::translit::greek_to_latin(&s);
        if let Some(t) = intl::translit::Transform::parse(&s) {
            let _ = t.apply(&s);
        }
        let _ = intl::unicode::idna::to_ascii(&s);
        let _ = intl::unicode::idna::to_unicode(&s);
        let _ = intl::unicode::spoof::skeleton(&s);
        let _ = intl::unicode::spoof::confusable(&s, &t);
    }
}

/// The locale-aware APIs must never panic, whatever locale tag or value they get
/// — including malformed BCP-47 tags and adversarial numbers. (This is the class
/// of bug — `.unwrap()` on uncontrolled data — behind real production outages.)
#[test]
fn fuzz_locale_apis() {
    use intl::datetime::{format_date, DateStyle, DateTime};
    use intl::number::{
        format_compact, format_currency, format_decimal, format_scientific, parse_decimal,
    };
    use intl::timezone::PosixTz;

    let mut rng = Rng(0xD1B54A32D192ED03);
    for _ in 0..20_000 {
        let llen = (rng.next() % 10) as usize;
        let lang = rng.string(llen);
        // A random finite-ish f64 plus some pathological values.
        let bits = rng.next();
        let value = match bits % 8 {
            0 => 0.0,
            1 => f64::from_bits(rng.next()), // NaN/Inf/subnormal included
            2 => -(rng.next() as f64),
            _ => (rng.next() as i64 as f64) / 1000.0,
        };

        let (clen, plen, nlen, oplen) = (
            (rng.next() % 5) as usize,
            (rng.next() % 12) as usize,
            (rng.next() % 6) as usize,
            (rng.next() % 8) as usize,
        );
        let sig = (rng.next() % 8) as usize;
        let (code, pstr, nstr, opstr) = (
            rng.string(clen),
            rng.string(plen),
            rng.string(nlen),
            rng.string(oplen),
        );

        let _ = format_decimal(&lang, value);
        let _ = format_scientific(&lang, value, sig);
        let _ = format_compact(&lang, value);
        let _ = format_currency(&lang, value, &code);
        let _ = parse_decimal(&lang, &pstr);
        let _ = intl::number::to_numbering_system(&lang, &nstr);
        let _ = intl::number::format_ordinal(&lang, rng.next() as i64);
        let _ = intl::spellout::spell_cardinal(&lang, rng.next() as i64);
        let _ = intl::spellout::spell_ordinal(&lang, rng.next() as i64);
        let _ = intl::unit::format_duration(&lang, rng.next() as i64, intl::unit::UnitWidth::Long);

        // Locale parsing/negotiation on random (often malformed) tags.
        let _ = intl::locale::Locale::parse(&lang);
        if let Some(ops) = intl::plural::PluralOperands::parse(&opstr) {
            let _ = intl::plural::plural_category(&lang, &ops);
        }

        // Date/time formatting and ISO/POSIX-TZ parsing on random input. Time
        // fields span the *full* u8 range (not just valid 0–23/0–59) to catch
        // arithmetic overflow on unvalidated DateTime values.
        let dt = DateTime {
            year: rng.next() as i32,
            month: rng.next() as u8,
            day: rng.next() as u8,
            hour: rng.next() as u8,
            minute: rng.next() as u8,
            second: rng.next() as u8,
        };
        let _ = format_date(&lang, &dt, DateStyle::Long);
        let _ = intl::datetime::format_time(&lang, &dt, DateStyle::Medium);
        let _ = intl::datetime::format_datetime(&lang, &dt, DateStyle::Full, DateStyle::Short);
        let _ = intl::datetime::format_skeleton(&lang, &dt, &pstr);
        let _ = dt.to_iso8601();
        let _ = dt.weekday();
        // Non-Gregorian renderers take a user-supplied (possibly out-of-range) month.
        let (y, mo, d) = (
            rng.next() as i64 % 5000,
            rng.next() as i64 % 16,
            rng.next() as i64 % 40,
        );
        let _ = intl::datetime::format_islamic_date(&lang, y, mo, d, DateStyle::Medium);
        let _ = intl::datetime::format_persian_date(&lang, y, mo, d, DateStyle::Medium);
        let _ = dt.add_seconds(rng.next() as i64);
        let _ = DateTime::parse_iso8601(&lang);
        if let Some(tz) = PosixTz::parse(&lang) {
            let _ = tz.offset_seconds(&dt);
        }
    }
}
