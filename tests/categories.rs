//! Property lookups across the codepoint tiers. Tier-specific assertions are
//! `#[cfg]`-gated so the suite passes under any feature selection.

use intl::unicode::{general_category, CharExt, GeneralCategory as GC, Group, UNICODE_VERSION};

#[test]
fn ascii_categories() {
    assert_eq!(general_category('A'), GC::UppercaseLetter);
    assert_eq!(general_category('Z'), GC::UppercaseLetter);
    assert_eq!(general_category('a'), GC::LowercaseLetter);
    assert_eq!(general_category('0'), GC::DecimalNumber);
    assert_eq!(general_category('9'), GC::DecimalNumber);
    assert_eq!(general_category(' '), GC::SpaceSeparator);
    assert_eq!(general_category('\n'), GC::Control);
    assert_eq!(general_category('\t'), GC::Control);
    assert_eq!(general_category('!'), GC::OtherPunctuation);
    assert_eq!(general_category('+'), GC::MathSymbol);
    assert_eq!(general_category('_'), GC::ConnectorPunctuation);
}

#[test]
fn ascii_predicates() {
    assert!('A'.is_uppercase());
    assert!(!'A'.is_lowercase());
    assert!('a'.is_lowercase());
    assert!('A'.is_letter() && 'A'.is_alphabetic());
    assert!('5'.is_numeric() && '5'.is_decimal_digit());
    assert!(' '.is_whitespace() && '\t'.is_whitespace() && '\n'.is_whitespace());
    assert!('\n'.is_control());
    assert!('!'.is_punctuation());
    assert!('+'.is_symbol());
    assert!('A'.is_assigned());
    assert!(!'5'.is_alphabetic());
}

#[test]
fn category_helpers() {
    assert_eq!(GC::UppercaseLetter.abbr(), "Lu");
    assert_eq!(GC::Unassigned.abbr(), "Cn");
    assert_eq!(GC::UppercaseLetter.group(), Group::Letter);
    assert_eq!(GC::DecimalNumber.group(), Group::Number);
    assert_eq!(GC::Control.group(), Group::Other);
    assert!(GC::TitlecaseLetter.is_cased_letter());
    assert!(!GC::ModifierLetter.is_cased_letter());
    assert!(GC::PrivateUse.is_assigned()); // Co is assigned
    assert!(!GC::Unassigned.is_assigned());
}

#[test]
fn unicode_version() {
    assert_eq!(UNICODE_VERSION, (17, 0, 0));
}

#[cfg(feature = "latin1")]
#[test]
fn latin1_categories() {
    assert_eq!(general_category('é'), GC::LowercaseLetter); // U+00E9
    assert_eq!(general_category('À'), GC::UppercaseLetter); // U+00C0
    assert_eq!(general_category('\u{00A0}'), GC::SpaceSeparator); // NBSP
    assert_eq!(general_category('±'), GC::MathSymbol); // U+00B1
    assert!('\u{00A0}'.is_whitespace());
    assert!('é'.is_alphabetic() && 'é'.is_lowercase());
}

#[cfg(feature = "bmp")]
#[test]
fn bmp_categories() {
    assert_eq!(general_category('Ω'), GC::UppercaseLetter); // U+03A9
    assert_eq!(general_category('中'), GC::OtherLetter); // U+4E2D (CJK)
    assert_eq!(general_category('٣'), GC::DecimalNumber); // U+0663 Arabic-Indic 3
    assert!('中'.is_alphabetic() && '中'.is_letter() && '中'.is_assigned());
    assert!('٣'.is_numeric());
}

#[cfg(feature = "bmp")]
#[test]
fn unassigned_in_bmp() {
    // U+0378 is a reserved (unassigned) codepoint inside the BMP.
    assert_eq!(general_category('\u{0378}'), GC::Unassigned);
    assert!(!'\u{0378}'.is_assigned());
    assert!(!'\u{0378}'.is_alphabetic());
}

#[cfg(feature = "full")]
#[test]
fn supplementary_categories() {
    assert_eq!(general_category('\u{10000}'), GC::OtherLetter); // Linear B
    assert_eq!(general_category('😀'), GC::OtherSymbol); // U+1F600
    assert!('\u{10000}'.is_assigned() && '\u{10000}'.is_letter());
}

#[cfg(feature = "bmp")]
#[test]
fn extra_binary_props() {
    use intl::unicode::{is_dash, is_default_ignorable, is_diacritic, is_hex_digit, is_math};
    assert!(is_math('+') && is_math('=') && !is_math('a'));
    assert!(is_dash('-') && is_dash('—'));
    assert!(is_hex_digit('F') && is_hex_digit('9') && !is_hex_digit('g'));
    assert!(is_default_ignorable('\u{200B}')); // zero width space
    assert!(is_diacritic('\u{0301}')); // combining acute
}
