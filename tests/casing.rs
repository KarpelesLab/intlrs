//! The conditional mappings of `SpecialCasing.txt`: `Final_Sigma`,
//! `After_Soft_Dotted`, `More_Above` (lt), `After_I` and `Not_Before_Dot`
//! (tr/az). Expectations were cross-checked against ICU 77 via node's
//! `toLocaleLowerCase` / `toLocaleUpperCase`.
#![cfg(all(feature = "case", feature = "alloc"))]
use intl::unicode::lowercase_str;

#[test]
fn final_sigma() {
    // ΟΔΟΣ -> ο δ ο + FINAL sigma (U+03C2).
    assert_eq!(lowercase_str("ΟΔΟΣ"), "\u{3bf}\u{3b4}\u{3bf}\u{3c2}");
    // ΣΟΦΟΣ -> word-initial Σ is σ, word-final Σ is ς.
    assert_eq!(
        lowercase_str("ΣΟΦΟΣ"),
        "\u{3c3}\u{3bf}\u{3c6}\u{3bf}\u{3c2}"
    );
    // A medial Σ between letters stays σ: ΟΣΟ -> οσο.
    assert_eq!(lowercase_str("ΟΣΟ"), "\u{3bf}\u{3c3}\u{3bf}");
    // A lone Σ (no cased letter before) -> σ.
    assert_eq!(lowercase_str("Σ"), "\u{3c3}");
    // ASCII unaffected.
    assert_eq!(lowercase_str("HELLO WORLD"), "hello world");
    // Full word: ὈΔΥΣΣΕΎΣ -> medial σσ, final ς.
    assert_eq!(lowercase_str("ὈΔΥΣΣΕΎΣ"), "ὀδυσσεύς");
}

#[test]
fn turkic() {
    use intl::unicode::{lowercase_str_lang as lo, uppercase_str_lang as up};
    // Turkish: dotless/dotted i.
    assert_eq!(lo("TITLE", "tr"), "tıtle");
    assert_eq!(lo("İSTANBUL", "tr"), "istanbul");
    assert_eq!(up("title", "tr"), "TİTLE");
    assert_eq!(up("ırmak", "az"), "IRMAK");
    // Non-Turkic keeps default behavior.
    assert_eq!(lo("TITLE", "en"), "title");
    assert_eq!(up("title", "en"), "TITLE");
    // "tra"-style false prefixes are not Turkic.
    assert_eq!(lo("TITLE", "translit"), "title");
}

#[test]
fn lithuanian() {
    use intl::unicode::lowercase_str_lang as lo;
    // I + combining acute (above) -> i + DOT ABOVE + acute (retained dot).
    assert_eq!(lo("I\u{0301}", "lt"), "i\u{0307}\u{0301}");
    assert_eq!(lo("J\u{0300}", "lt"), "j\u{0307}\u{0300}");
    // No following above-accent -> plain lowercase, no extra dot.
    assert_eq!(lo("I", "lt"), "i");
    assert_eq!(lo("LIETUVA", "lt"), "lietuva");
    // Precomposed Ì/Í/Ĩ expand with the retained dot.
    assert_eq!(lo("Ì", "lt"), "i\u{0307}\u{0300}");
    assert_eq!(lo("Ĩ", "lt"), "i\u{0307}\u{0303}");
    // Non-Lithuanian: default lowercasing (no inserted dot).
    assert_eq!(lo("I\u{0301}", "en"), "i\u{0301}");
}

/// `More_Above`: a class-230 (Above) character follows, with no intervening
/// character of class 0 or 230.
#[test]
fn lithuanian_more_above() {
    use intl::unicode::lowercase_str_lang as lo;
    // All three tailored letters: I, J, Į.
    assert_eq!(lo("I\u{0301}", "lt"), "i\u{0307}\u{0301}");
    assert_eq!(lo("J\u{0301}", "lt"), "j\u{0307}\u{0301}");
    assert_eq!(lo("\u{012E}\u{0301}", "lt"), "\u{012F}\u{0307}\u{0301}");
    // A class-202 (below) mark may intervene; the Above mark still counts.
    assert_eq!(
        lo("I\u{0327}\u{0301}", "lt"),
        "i\u{0307}\u{0327}\u{0301}" // cedilla (ccc 202) does not break the run
    );
    // ...but it alone is not "above", so no dot is introduced.
    assert_eq!(lo("I\u{0327}", "lt"), "i\u{0327}");
    // A starter (ccc 0) ends the combining sequence before any Above mark.
    assert_eq!(lo("IA\u{0301}", "lt"), "ia\u{0301}");
    // End of string: nothing follows at all.
    assert_eq!(lo("I", "lt"), "i");
    // U+0307 itself is class 230, so it triggers More_Above and is *kept*:
    // node: 'İ'.toLocaleLowerCase('lt') === 'i̇̇'
    assert_eq!(lo("I\u{0307}", "lt"), "i\u{0307}\u{0307}");
    // Any Above mark counts, not just Lithuanian accents (U+033D is ccc 230).
    assert_eq!(lo("I\u{033D}", "lt"), "i\u{0307}\u{033D}");
    // İ has its own unconditional lt-independent mapping; More_Above must not
    // add a second dot: node gives 'i̇́'.
    assert_eq!(lo("\u{0130}\u{0301}", "lt"), "i\u{0307}\u{0301}");
}

/// `After_Soft_Dotted`: a Soft_Dotted character precedes, with no intervening
/// character of class 0 or 230. Only uppercasing (and titlecasing) removes the
/// dot; lowercasing leaves it alone.
#[test]
fn lithuanian_after_soft_dotted() {
    use intl::unicode::{lowercase_str_lang as lo, uppercase_str_lang as up};
    // The reported case: node 'i̇'.toLocaleUpperCase('lt') === 'I'.
    assert_eq!(up("i\u{0307}", "lt"), "I");
    assert_eq!(up("j\u{0307}", "lt"), "J");
    assert_eq!(up("\u{012F}\u{0307}", "lt"), "\u{012E}");
    // A trailing accent survives; only the dot goes.
    assert_eq!(up("i\u{0307}\u{0301}", "lt"), "I\u{0301}");
    // A class-202 mark may intervene between the letter and the dot.
    assert_eq!(up("i\u{0327}\u{0307}", "lt"), "I\u{0327}");
    // A class-230 mark in between breaks the context: the dot is kept.
    assert_eq!(up("i\u{0300}\u{0307}", "lt"), "I\u{0300}\u{0307}");
    // Soft_Dotted is a property, not the letters i/j: Cyrillic і/ј and the
    // modifier letter ʲ are Soft_Dotted, dotless ı is not.
    assert_eq!(up("\u{0456}\u{0307}", "lt"), "\u{0406}");
    assert_eq!(up("\u{02B2}\u{0307}", "lt"), "\u{02B2}");
    assert_eq!(up("\u{1D422}\u{0307}", "lt"), "\u{1D422}"); // astral Soft_Dotted
    assert_eq!(up("\u{0131}\u{0307}", "lt"), "I\u{0307}");
    // Not after a Soft_Dotted character at all.
    assert_eq!(up("a\u{0307}", "lt"), "A\u{0307}");
    // Start of string: nothing precedes.
    assert_eq!(up("\u{0307}", "lt"), "\u{0307}");
    // Only the first dot is absorbed; the second follows a class-230 char.
    assert_eq!(up("i\u{0307}\u{0307}", "lt"), "I\u{0307}");
    // Lowercasing keeps the dot (the lt entry maps 0307 to itself for `lc`).
    assert_eq!(lo("i\u{0307}", "lt"), "i\u{0307}");
    // Non-Lithuanian locales keep the dot when uppercasing.
    assert_eq!(up("i\u{0307}", "en"), "I\u{0307}");
    assert_eq!(up("i\u{0307}", "tr"), "\u{0130}\u{0307}");
}

/// `After_I` (remove U+0307 after `I`) and `Not_Before_Dot` (`I` → `ı` unless a
/// dot above follows), both for `tr` and `az`.
#[test]
fn turkic_after_i_and_not_before_dot() {
    use intl::unicode::lowercase_str_lang as lo;
    for lang in ["tr", "az"] {
        // The reported case: node 'İ'.toLocaleLowerCase('tr') === 'i'.
        assert_eq!(lo("I\u{0307}", lang), "i");
        // Not_Before_Dot holds -> dotless ı.
        assert_eq!(lo("I", lang), "\u{0131}");
        assert_eq!(lo("I\u{0301}", lang), "\u{0131}\u{0301}"); // acute, not a dot
        // A class-202 mark may intervene in both directions.
        assert_eq!(lo("I\u{0327}\u{0307}", lang), "i\u{0327}");
        // A starter breaks both conditions.
        assert_eq!(lo("IX\u{0307}", lang), "\u{0131}x\u{0307}");
        // A class-230 mark breaks both conditions.
        assert_eq!(lo("I\u{0307}\u{0307}", lang), "i\u{0307}");
        assert_eq!(lo("I\u{0307}\u{0328}\u{0307}", lang), "i\u{0328}\u{0307}");
        // The accent after the absorbed dot survives.
        assert_eq!(lo("I\u{0307}\u{0301}", lang), "i\u{0301}");
        // Start of string: no preceding I, so the dot stays.
        assert_eq!(lo("\u{0307}", lang), "\u{0307}");
        assert_eq!(lo("\u{0307}I", lang), "\u{0307}\u{0131}");
        // After_I means U+0049 specifically: J and Į do not absorb the dot.
        assert_eq!(lo("J\u{0307}", lang), "j\u{0307}");
        assert_eq!(lo("\u{012E}\u{0307}", lang), "\u{012F}\u{0307}");
    }
    // Only Turkic: elsewhere both characters map independently.
    assert_eq!(lo("I\u{0307}", "en"), "i\u{0307}");
    assert_eq!(lo("I\u{0307}", "lt"), "i\u{0307}\u{0307}");
}

/// `Final_Sigma` is language-insensitive, so it also applies under the Turkic
/// and Lithuanian tailorings, and to the characters a titlecasing lowercases.
#[test]
fn final_sigma_is_language_insensitive() {
    use intl::unicode::{lowercase_str_lang as lo, titlecase};
    // node: 'ΑΣ'.toLocaleLowerCase('tr') === 'ας'
    for lang in ["tr", "az", "lt", "en", "el"] {
        assert_eq!(lo("\u{0391}\u{03A3}", lang), "\u{03B1}\u{03C2}");
        assert_eq!(
            lo("\u{0391}\u{03A3}\u{0391}", lang),
            "\u{03B1}\u{03C3}\u{03B1}"
        );
    }
    // Titlecasing lowercases every non-initial character, so the rule applies.
    assert_eq!(titlecase("ὈΔΥΣΣΕΎΣ"), "Ὀδυσσεύς");
    // Per word: medial Σ -> σ, word-final Σ -> ς.
    assert_eq!(
        titlecase("ΟΣΟ ΟΔΟΣ"),
        "\u{039F}\u{03C3}\u{03BF} \u{039F}\u{03B4}\u{03BF}\u{03C2}"
    );
}

/// Locale-tag matching: subtags and the `_` separator, but not longer tags that
/// merely start with the same letters.
#[test]
fn tailoring_locale_matching() {
    use intl::unicode::{lowercase_str_lang as lo, uppercase_str_lang as up};
    assert_eq!(lo("I\u{0307}", "tr-TR"), "i");
    assert_eq!(lo("I\u{0307}", "az_Latn_AZ"), "i");
    assert_eq!(up("i\u{0307}", "lt-LT"), "I");
    assert_eq!(up("i\u{0307}", "lt_LT"), "I");
    // Not Turkic / not Lithuanian.
    assert_eq!(lo("I\u{0307}", "translit"), "i\u{0307}");
    assert_eq!(up("i\u{0307}", "ltz"), "I\u{0307}");
}
