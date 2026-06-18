//! Locale display names.
#![cfg(feature = "displaynames")]
use intl::display::{language_name as ln, region_name as rn};

#[test]
fn names() {
    assert_eq!(ln("en", "fr"), Some("French"));
    assert_eq!(ln("en", "de"), Some("German"));
    assert_eq!(ln("fr", "de"), Some("allemand"));
    assert_eq!(ln("de", "en"), Some("Englisch"));
    assert_eq!(rn("en", "JP"), Some("Japan"));
    assert_eq!(rn("en", "US"), Some("United States"));
    assert_eq!(rn("de", "US"), Some("Vereinigte Staaten"));
    // case-insensitive code; region fallback to English for unknown display locale
    assert_eq!(ln("EN", "FR"), Some("French"));
    assert_eq!(ln("xx", "fr"), Some("French")); // unknown display locale -> en
    assert_eq!(ln("en", "zzz"), None);
}

#[test]
fn overlong_multibyte_display_locale_falls_back_to_prefix() {
    // A display locale "de" followed by a long multibyte subtag that overflows
    // the 40-byte normalization buffer, splitting a 2-byte char at the cut.
    // Truncating on a char boundary keeps a valid prefix so the `-` strip
    // resolves "de" (German), instead of failing UTF-8 validation and silently
    // falling through to English.
    let tag = format!("de-{}", "à".repeat(40));
    assert_eq!(rn(&tag, "US"), Some("Vereinigte Staaten"));
    assert_eq!(ln(&tag, "en"), Some("Englisch"));
}
