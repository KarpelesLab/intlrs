//! Readable collation spot-checks. The exhaustive UCA conformance run lives in
//! `collation_conformance.rs`.
#![cfg(feature = "collation")]

use core::cmp::Ordering;
use intl::unicode::collate::{AlternateHandling, Collator, compare};

#[test]
fn basic_order() {
    assert_eq!(compare("apple", "apply"), Ordering::Less);
    assert_eq!(compare("apple", "apple"), Ordering::Equal);
    assert_eq!(compare("b", "a"), Ordering::Greater);
}

#[test]
fn accents_sort_after_base() {
    // "é" differs from "e" only at the secondary level, so "cafe" < "café",
    // but both sort before "caff" (primary 'e' < 'f').
    assert_eq!(compare("cafe", "café"), Ordering::Less);
    assert_eq!(compare("café", "caff"), Ordering::Less);
}

#[test]
fn case_differs_at_tertiary() {
    // Lowercase sorts before uppercase (a tertiary distinction).
    assert_eq!(compare("a", "A"), Ordering::Less);
    // ...but only after primary/secondary agree.
    assert_eq!(compare("A", "b"), Ordering::Less);
}

#[test]
fn variable_handling() {
    // Default (shifted): leading punctuation is ignored at the primary level, so
    // "b" and "-c" compare as 'b' vs 'c' → "b" < "-c".
    assert_eq!(compare("b", "-c"), Ordering::Less);

    // Non-ignorable: the hyphen is a primary character that sorts before
    // letters, so "-c" < "b".
    let ni = Collator::new(AlternateHandling::NonIgnorable);
    assert_eq!(ni.compare("b", "-c"), Ordering::Greater);
}

#[test]
fn sort_key_matches_compare() {
    let c = Collator::default();
    assert!(c.sort_key("apple") < c.sort_key("apply"));
}

/// Signature orderings for locales newly bundled from the official CLDR-48
/// collation rules (see `data/cldr/48/collation.json`). Each asserts the
/// characteristic tailored order that root DUCET does *not* produce.
#[test]
fn newly_added_cldr_tailorings() {
    use intl::unicode::collate::Tailoring;
    let lt = |t: &Tailoring, a: &str, b: &str| {
        assert_eq!(t.compare(a, b), Ordering::Less, "expected {a} < {b}");
    };

    // Polish: ogonek/acute letters each sort right after their base letter.
    let pl = Tailoring::for_locale("pl").unwrap();
    lt(&pl, "a", "ą");
    lt(&pl, "ą", "b");
    lt(&pl, "c", "ć");
    lt(&pl, "z", "ź");
    lt(&pl, "ź", "ż");

    // Galician: ñ after n (imported from Spanish).
    let gl = Tailoring::for_locale("gl").unwrap();
    lt(&gl, "n", "ñ");

    // Northern Sotho / Tswana: circumflex vowels and š after their bases.
    for loc in ["nso", "tn"] {
        let t = Tailoring::for_locale(loc).unwrap();
        lt(&t, "e", "ê");
        lt(&t, "o", "ô");
        lt(&t, "s", "š");
    }

    // Wolof: à, é/ë, ñ/ŋ, ó after their base letters.
    let wo = Tailoring::for_locale("wo").unwrap();
    lt(&wo, "a", "à");
    lt(&wo, "e", "é");
    lt(&wo, "é", "ë");
    lt(&wo, "n", "ñ");
    lt(&wo, "ñ", "ŋ");

    // Yoruba: dotted-below vowels and the "gb" digraph.
    let yo = Tailoring::for_locale("yo").unwrap();
    lt(&yo, "e", "ẹ");
    lt(&yo, "g", "gb");
    lt(&yo, "s", "ṣ");

    // Igbo: digraphs (gb/gh/gw, kp/kw, nw/ny) and dotted-below vowels.
    let ig = Tailoring::for_locale("ig").unwrap();
    lt(&ig, "g", "gb");
    lt(&ig, "gb", "gh");
    lt(&ig, "n", "nw");
    lt(&ig, "i", "ị");

    // Ewe: dz digraph and the open vowels ɛ/ɔ after e/o.
    let ee = Tailoring::for_locale("ee").unwrap();
    lt(&ee, "d", "dz");
    lt(&ee, "e", "ɛ");
    lt(&ee, "o", "ɔ");
    lt(&ee, "n", "ŋ");

    // Belarusian / Kyrgyz (Cyrillic): ё after е, ў after у.
    let be = Tailoring::for_locale("be").unwrap();
    lt(&be, "Е", "ё");
    lt(&be, "у", "ў");
    let ky = Tailoring::for_locale("ky").unwrap();
    lt(&ky, "е", "ё");

    // Macedonian (Cyrillic): ѓ and ќ as their own letters.
    let mk = Tailoring::for_locale("mk").unwrap();
    lt(&mk, "ԃ", "ѓ");
    lt(&mk, "ћ", "ќ");
}

/// CJK collations: Japanese kana and Korean Hangul (the headline additions).
/// Verified against V8 `Intl.Collator`.
#[test]
fn cjk_tailorings() {
    use intl::unicode::collate::Tailoring;
    let ja = Tailoring::for_locale("ja").unwrap();
    // Hiragana and katakana of the same syllable collate equal (V8 `あ` == `ア`).
    assert_eq!(ja.compare("あ", "ア"), Ordering::Equal);
    assert_eq!(ja.compare("か", "カ"), Ordering::Equal);
    assert_eq!(ja.compare("ゔ", "ヴ"), Ordering::Equal); // voiced u
    // Kana still order among themselves (root gojūon order) and voicing is a
    // finer distinction than the base syllable.
    assert_eq!(ja.compare("か", "き"), Ordering::Less);
    assert_eq!(ja.compare("か", "が"), Ordering::Less); // voiced sorts after
    assert_eq!(ja.compare("は", "ぱ"), Ordering::Less);
    // A katakana word interleaves correctly with a hiragana one.
    assert_eq!(ja.compare("カガ", "かき"), Ordering::Less);

    let ko = Tailoring::for_locale("ko").unwrap();
    // Hangul syllables sort in Unicode (gaŭ) order; jamo likewise.
    assert_eq!(ko.compare("가", "나"), Ordering::Less);
    assert_eq!(ko.compare("각", "간"), Ordering::Less);
    assert_eq!(ko.compare("\u{1100}", "\u{1102}"), Ordering::Less); // ᄀ < ᄂ
    assert_eq!(ko.compare("강", "가나"), Ordering::Greater);
}

/// Chinese (`zh`) pinyin collation — the default `Intl.Collator('zh')` order
/// (feature `collation-zh`). Verified against V8 `Intl.Collator('zh')`.
#[cfg(feature = "collation-zh")]
#[test]
fn zh_pinyin_collation() {
    use intl::unicode::collate::Tailoring;
    let zh = Tailoring::for_locale("zh").unwrap();
    // Han sort by pinyin: 阿(ā) < 你(nǐ) < 中(zhōng); 的(de) < 心(xīn).
    assert_eq!(zh.compare("阿", "你"), Ordering::Less);
    assert_eq!(zh.compare("你", "中"), Ordering::Less);
    assert_eq!(zh.compare("的", "心"), Ordering::Less);
    assert_eq!(zh.compare("我", "你"), Ordering::Greater); // wǒ > nǐ
    // `[reorder Hani]`: digits < Han < Latin < other scripts.
    assert_eq!(zh.compare("中", "9"), Ordering::Greater); // Han after digits
    assert_eq!(zh.compare("中", "a"), Ordering::Less); // Han before Latin
    assert_eq!(zh.compare("中", "я"), Ordering::Less); // Han before Cyrillic
    // Multi-char words and mixed Han/Latin sort like V8.
    assert_eq!(zh.compare("北京", "上海"), Ordering::Less); // běijīng < shànghǎi
    assert_eq!(zh.compare("中国", "中文"), Ordering::Less); // guó < wén
    assert_eq!(zh.compare("中文", "a中"), Ordering::Less); // Han word before Latin lead
    // `zh-Hans` / `zh-CN` resolve to the same pinyin collator.
    assert_eq!(
        Tailoring::for_locale("zh-Hans")
            .unwrap()
            .compare("的", "心"),
        Ordering::Less
    );
    // A CJK Compatibility Ideograph (U+FA0C 兀) normalizes to its URO form and
    // sorts by that pinyin reading, not its code point.
    assert_eq!(zh.compare("\u{FA0C}", "中"), Ordering::Less);
    // Non-Han text is unaffected (root order among Latin).
    assert_eq!(zh.compare("apple", "banana"), Ordering::Less);
}

/// Locales whose CLDR rule needs the extended parser syntax: `[before]`
/// (reset-before), `[import]` (splice another locale), and the newly bundled
/// `af`/`hr`. Verified against V8 `Intl.Collator`.
#[test]
fn extended_syntax_tailorings() {
    use intl::unicode::collate::Tailoring;
    let lt = |t: &Tailoring, a: &str, b: &str| {
        assert_eq!(t.compare(a, b), Ordering::Less, "expected {a} < {b}");
    };

    // Icelandic (`[before 1]`): þ sorts *after* z (not near y), æ ä ö ø å after þ.
    let is = Tailoring::for_locale("is").unwrap();
    lt(&is, "a", "á");
    lt(&is, "z", "þ");
    lt(&is, "þ", "æ");
    lt(&is, "æ", "ä");
    lt(&is, "ø", "å");

    // Turkish (`[before 1]`): dotless ı sorts between h and i.
    let tr = Tailoring::for_locale("tr").unwrap();
    lt(&tr, "h", "ı");
    lt(&tr, "ı", "i");
    lt(&tr, "i", "İ");
    lt(&tr, "c", "ç");

    // Estonian (`[before 1]`): š/z/ž after s (before t); õ/ä/ö/ü after w — so
    // ä sorts *after* a plain t, which the old single-anchor rule got wrong.
    let et = Tailoring::for_locale("et").unwrap();
    lt(&et, "s", "š");
    lt(&et, "š", "z");
    lt(&et, "z", "ž");
    lt(&et, "t", "õ"); // õ after w, hence after t
    lt(&et, "w", "õ");
    lt(&et, "õ", "ä");

    // Kazakh (`[before 1]` + Cyrillic): і resets before ь, ё/ү after their bases.
    let kk = Tailoring::for_locale("kk").unwrap();
    lt(&kk, "е", "ё");
    lt(&kk, "ұ", "ү");
    lt(&kk, "і", "ь");

    // Afrikaans (bundled): ŉ (n preceded by apostrophe) collates just after n.
    let af = Tailoring::for_locale("af").unwrap();
    lt(&af, "n", "ŉ");
    lt(&af, "ŉ", "o");

    // Croatian (bundled, full CLDR digraphs).
    let hr = Tailoring::for_locale("hr").unwrap();
    lt(&hr, "c", "č");
    lt(&hr, "d", "dž");
    lt(&hr, "dž", "đ");
    lt(&hr, "l", "lj");

    // Bosnian is defined purely as `[import hr]`; it must reproduce hr's order.
    let bs = Tailoring::for_locale("bs").unwrap();
    for w in ["c", "č", "dž", "đ", "lj", "nj", "š", "ž"] {
        assert_eq!(bs.compare("c", w), hr.compare("c", w), "bs != hr at {w}");
    }
}

/// Parser-level unit tests for the newly supported syntax.
#[test]
fn parser_extensions() {
    use intl::unicode::collate::Tailoring;

    // `\u`/`\U` escapes: `ñ` is ñ; the rule equals the literal one.
    let esc = Tailoring::parse("&n < \\u00F1").unwrap();
    assert_eq!(esc.compare("n", "ñ"), Ordering::Less);
    assert_eq!(esc.compare("ñ", "o"), Ordering::Less);

    // Quoting: an operator character can be tailored as a literal via '<'.
    let q = Tailoring::parse("&a < '<'").unwrap();
    assert_eq!(q.compare("a", "<"), Ordering::Less);

    // Star / range shorthand: `&x <* abc` == `&x < a < b < c`.
    let star = Tailoring::parse("&a <* xyz").unwrap();
    assert_eq!(star.compare("a", "x"), Ordering::Less);
    assert_eq!(star.compare("x", "y"), Ordering::Less);
    assert_eq!(star.compare("y", "z"), Ordering::Less);

    // `[import es]` splices Spanish (ñ after n); ignored options are skipped.
    let imp = Tailoring::parse("[reorder Latn] [import es]").unwrap();
    assert_eq!(imp.compare("n", "ñ"), Ordering::Less);

    // `[before 1]`: reset before the anchor places the letter just below it.
    let bef = Tailoring::parse("&[before 1] i < ı").unwrap();
    assert_eq!(bef.compare("h", "ı"), Ordering::Less);
    assert_eq!(bef.compare("ı", "i"), Ordering::Less);

    // Comments and whitespace are stripped.
    let c = Tailoring::parse("# leading comment\n&a < b # trailing").unwrap();
    assert_eq!(c.compare("a", "b"), Ordering::Less);
}
