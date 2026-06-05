//! Confusable / spoof detection (UTS #39). Requires the `alloc` feature.

use super::generated::confusables as gen;
use super::normalize::nfd;
use super::predicates::is_default_ignorable;
use super::script::{script_extensions, Script, ScriptExtensions};
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

/// `true` if `s` is *single-script* under UTS #39 "Single Script" resolution:
/// the intersection of the `Script_Extensions` sets of all its characters is
/// non-empty, i.e. there exists at least one script every character can be
/// written in.
///
/// Resolution uses each character's full `Script_Extensions` set, not just its
/// primary `Script`. So U+30FC (KATAKANA-HIRAGANA PROLONGED SOUND MARK), whose
/// primary `Script` is `Common` but whose `Script_Extensions` is `{Hira, Kana,
/// ...}`, constrains the running script set rather than being ignored.
/// Characters whose `Script_Extensions` is exactly `{Common}` or `{Inherited}`
/// (shared punctuation, digits, combining marks, …) are compatible with every
/// script and impose no constraint. An empty string is single-script.
///
/// A `false` result flags a mixed-script string — a common spoofing signal.
///
/// This implements only the core `Script_Extensions` intersection. The UTS #39
/// *augmented* profile additionally treats certain CJK combinations
/// (Han+Hiragana+Katakana, Han+Hangul, Han+Bopomofo) as mutually compatible;
/// that augmentation is **not** applied here, so e.g. mixed Han/Kana text is
/// reported as multi-script.
///
/// ```
/// use intl::unicode::spoof::is_single_script;
/// assert!(is_single_script("hello"));
/// // Latin + Cyrillic 'у' (U+0443) — mixed script.
/// assert!(!is_single_script("paуpal"));
/// assert!(is_single_script(""));
/// // Shared punctuation and digits keep Latin text single-script.
/// assert!(is_single_script("abc-123"));
/// ```
#[must_use]
pub fn is_single_script(s: &str) -> bool {
    // Running intersection of Script_Extensions sets across the string.
    // `None` means "still unconstrained" (compatible with every script).
    let mut acc: Option<Vec<Script>> = None;
    for c in s.chars() {
        match script_extensions(c) {
            // Exactly {Common} or {Inherited}: compatible with any script.
            ScriptExtensions::Single(Script::Common)
            | ScriptExtensions::Single(Script::Inherited) => continue,
            sx => {
                let set = sx.as_slice();
                acc = Some(match acc {
                    None => set.to_vec(),
                    Some(prev) => prev.into_iter().filter(|s| set.contains(s)).collect(),
                });
                if acc.as_ref().is_some_and(Vec::is_empty) {
                    return false; // empty intersection => mixed script
                }
            }
        }
    }
    true
}
