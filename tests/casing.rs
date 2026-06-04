//! Context-sensitive lowercasing (Greek Final_Sigma).
#![cfg(feature = "alloc")]
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
