//! Locale display names.
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
