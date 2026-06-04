//! UAX #31 identifier checks.

use intl::unicode::{is_identifier, is_xid_continue, is_xid_start};

#[test]
fn xid_properties() {
    assert!(is_xid_start('A') && is_xid_start('a') && !is_xid_start('_'));
    assert!(is_xid_continue('A') && is_xid_continue('0'));
    assert!(!is_xid_start('0')); // digits continue but don't start
    assert!(!is_xid_continue('-'));
}

#[test]
fn identifiers() {
    assert!(is_identifier("foo"));
    assert!(is_identifier("foo_bar2"));
    assert!(!is_identifier("2foo"));
    assert!(!is_identifier("foo-bar"));
    assert!(!is_identifier(""));
}

#[cfg(feature = "bmp")]
#[test]
fn unicode_identifiers() {
    assert!(is_identifier("café"));
    assert!(is_identifier("Δelta"));
    assert!(is_identifier("переменная")); // Cyrillic
    assert!(!is_identifier("π²")); // superscript 2 is not XID_Continue
}
