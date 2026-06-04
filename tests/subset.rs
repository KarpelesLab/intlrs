//! Range-subset semantics: codepoints outside the compiled tier must resolve to
//! the neutral default (`Unassigned`), while codepoints inside it resolve
//! normally. These assertions are `#[cfg]`-gated to the narrow tier builds that
//! exercise them, e.g.:
//!
//! ```text
//! cargo test --test subset --no-default-features --features latin1
//! cargo test --test subset --no-default-features --features ascii
//! ```

/// With Latin-1 compiled but not the rest of the BMP, a CJK codepoint is
/// reported `Unassigned` even though it is assigned in Unicode.
#[cfg(all(feature = "latin1", not(feature = "bmp")))]
#[test]
fn cjk_unassigned_when_only_latin1() {
    use intl::unicode::{general_category, GeneralCategory as GC};
    assert_eq!(general_category('中'), GC::Unassigned); // U+4E2D not compiled
    assert_eq!(general_category('é'), GC::LowercaseLetter); // U+00E9 compiled
    assert_eq!(general_category('A'), GC::UppercaseLetter);
}

/// With only ASCII compiled, even Latin-1 codepoints fall through to
/// `Unassigned`.
#[cfg(all(feature = "ascii", not(feature = "latin1")))]
#[test]
fn latin1_unassigned_when_only_ascii() {
    use intl::unicode::{general_category, GeneralCategory as GC};
    assert_eq!(general_category('é'), GC::Unassigned); // U+00E9 not compiled
    assert_eq!(general_category('A'), GC::UppercaseLetter); // U+0041 compiled
    assert_eq!(general_category('0'), GC::DecimalNumber);
}

/// Without `full`, supplementary-plane codepoints are `Unassigned`.
#[cfg(all(feature = "bmp", not(feature = "full")))]
#[test]
fn supplementary_unassigned_without_full() {
    use intl::unicode::{general_category, GeneralCategory as GC};
    assert_eq!(general_category('\u{10000}'), GC::Unassigned); // not compiled
    assert_eq!(general_category('中'), GC::OtherLetter); // BMP, compiled
}
