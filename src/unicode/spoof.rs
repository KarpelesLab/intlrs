//! Confusable / spoof detection (UTS #39). Requires the `alloc` feature.

use super::generated::confusables as gen;
use super::normalize::nfd;
use super::predicates::is_default_ignorable;
use super::script::{script, Script};
use alloc::string::String;
use alloc::vec::Vec;

/// The UTS #39 *confusable skeleton* of `s`: drop `Default_Ignorable_Code_Point`
/// characters, NFD, replace each character by its confusable prototype, then NFD
/// again. Two strings are visually confusable iff their skeletons are equal — see
/// [`confusable`].
///
/// Stripping default-ignorables (e.g. ZWSP U+200B, ZWJ/ZWNJ, variation selectors)
/// is required by UTS #39: such characters are invisible in rendering, so an
/// attacker could otherwise hide them inside a homograph (`"pay\u{200B}pal"`) to
/// evade detection.
///
/// ```
/// use intl::unicode::spoof::skeleton;
/// // Cyrillic "а" and Latin "a" share a skeleton.
/// assert_eq!(skeleton("pаypal"), skeleton("paypal"));
/// // An interspersed zero-width space is ignored.
/// assert_eq!(skeleton("pay\u{200B}pal"), skeleton("paypal"));
/// ```
#[must_use]
pub fn skeleton(s: &str) -> String {
    let mut mapped: Vec<char> = Vec::new();
    for c in nfd(s.chars().filter(|&c| !is_default_ignorable(c))) {
        match gen::confusable_prototype(c as u32) {
            Some(proto) => mapped.extend_from_slice(proto),
            None => mapped.push(c),
        }
    }
    nfd(mapped.into_iter()).collect()
}

/// `true` if `a` and `b` are confusable (have the same [`skeleton`]) yet are not
/// the same string.
#[must_use]
pub fn confusable(a: &str, b: &str) -> bool {
    a != b && skeleton(a) == skeleton(b)
}

/// `true` if every character of `s` could belong to a single script under
/// `Script_Extensions` resolution (UTS #39 "Single Script"). Characters that are
/// `Common` or `Inherited` are compatible with any script. An empty string is
/// single-script.
///
/// A `false` result flags a mixed-script string — a common spoofing signal.
#[must_use]
pub fn is_single_script(s: &str) -> bool {
    let mut resolved: Option<Script> = None;
    for c in s.chars() {
        let sc = script(c);
        if matches!(sc, Script::Common | Script::Inherited) {
            continue;
        }
        match resolved {
            None => resolved = Some(sc),
            Some(r) if r == sc => {}
            Some(_) => return false,
        }
    }
    true
}
