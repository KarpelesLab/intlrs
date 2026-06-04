//! Unicode identifier properties and syntax (UAX #31).

use super::generated::binary_props as gen;

/// `XID_Start`: characters that can begin a default Unicode identifier.
#[inline]
#[must_use]
pub const fn is_xid_start(c: char) -> bool {
    gen::xid_start(c as u32)
}

/// `XID_Continue`: characters that can continue a default Unicode identifier
/// (a superset of [`is_xid_start`]).
#[inline]
#[must_use]
pub const fn is_xid_continue(c: char) -> bool {
    gen::xid_continue(c as u32)
}

/// `true` if `s` is a default Unicode identifier (UAX #31 R1): a non-empty
/// string whose first character has `XID_Start` and whose remaining characters
/// all have `XID_Continue`.
///
/// ```
/// use intl::unicode::is_identifier;
/// assert!(is_identifier("naïve"));
/// assert!(is_identifier("Δx"));
/// assert!(!is_identifier("1st"));   // starts with a digit
/// assert!(!is_identifier("a-b"));   // hyphen is not XID_Continue
/// assert!(!is_identifier(""));
/// ```
#[must_use]
pub fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if is_xid_start(c) => chars.all(is_xid_continue),
        _ => false,
    }
}
