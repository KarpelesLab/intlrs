//! Age and Block character properties (UCD). Assertions are gated by range tier:
//! a codepoint outside the compiled tier reads its neutral default (None /
//! "No_Block").
use intl::unicode::{age, block};

#[test]
fn ascii() {
    assert_eq!(age('A'), Some((1, 1))); // ASCII: Unicode 1.1
    assert_eq!(block('A'), "Basic Latin");
}

#[cfg(feature = "bmp")]
#[test]
fn bmp() {
    assert_eq!(age('é'), Some((1, 1)));
    assert_eq!(age('€'), Some((2, 1))); // EURO SIGN: 2.1
    assert_eq!(age('\u{20BF}'), Some((10, 0))); // BITCOIN SIGN: 10.0
    assert_eq!(age('\u{0378}'), None); // unassigned in the BMP
    assert!(age('A').unwrap() < age('\u{20BF}').unwrap());

    assert_eq!(block('é'), "Latin-1 Supplement");
    assert_eq!(block('Ω'), "Greek and Coptic");
    assert_eq!(block('日'), "CJK Unified Ideographs");
    assert_eq!(block('\u{0590}'), "Hebrew");
}

#[cfg(feature = "full")]
#[test]
fn supplementary() {
    assert_eq!(block('\u{1F600}'), "Emoticons");
    assert_eq!(block('\u{E0000}'), "Tags");
    assert_eq!(age('\u{1F600}'), Some((6, 1)));
}

#[cfg(feature = "bmp")]
#[test]
fn joining_types() {
    use intl::unicode::{joining_type, JoiningType::*};
    assert_eq!(joining_type('\u{0628}'), DualJoining); // ARABIC BEH
    assert_eq!(joining_type('\u{0627}'), RightJoining); // ARABIC ALEF
    assert_eq!(joining_type('\u{0640}'), JoinCausing); // ARABIC TATWEEL
    assert_eq!(joining_type('\u{0610}'), Transparent); // ARABIC SIGN (combining)
    assert_eq!(joining_type('A'), NonJoining);
    assert_eq!(joining_type(' '), NonJoining);
}

#[cfg(feature = "bmp")]
#[test]
fn indic_categories() {
    use intl::unicode::{indic_syllabic_category as isc, IndicSyllabicCategory::*};
    assert_eq!(isc('\u{0915}'), Consonant); // DEVANAGARI KA
    assert_eq!(isc('\u{094D}'), Virama); // DEVANAGARI VIRAMA
    assert_eq!(isc('\u{0905}'), VowelIndependent); // DEVANAGARI A
    assert_eq!(isc('A'), Other);
}

#[cfg(feature = "bmp")]
#[test]
fn indic_positional() {
    use intl::unicode::{indic_positional_category as ipc, IndicPositionalCategory::*};
    assert_eq!(ipc('\u{093F}'), Left); // DEVANAGARI VOWEL SIGN I (pre-base)
    assert_eq!(ipc('\u{0940}'), Right); // DEVANAGARI VOWEL SIGN II
    assert_eq!(ipc('A'), NotApplicable);
}

#[cfg(feature = "alloc")]
#[test]
fn hangul_names() {
    use intl::unicode::hangul_syllable_name as hn;
    assert_eq!(hn('가').as_deref(), Some("HANGUL SYLLABLE GA")); // U+AC00
    assert_eq!(hn('한').as_deref(), Some("HANGUL SYLLABLE HAN"));
    assert_eq!(hn('글').as_deref(), Some("HANGUL SYLLABLE GEUL"));
    assert_eq!(hn('\u{D7A3}').as_deref(), Some("HANGUL SYLLABLE HIH")); // last syllable
    assert_eq!(hn('A'), None);
    assert_eq!(hn('\u{ABFF}'), None); // just before the block
}
