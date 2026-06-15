//! Confusable / spoof detection (UTS #39). Requires the `alloc` feature.

use super::generated::confusables as tables;
use super::normalize::nfd;
use super::predicates::is_default_ignorable;
use super::script::{Script, ScriptExtensions, script_extensions};
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
        match tables::confusable_prototype(c as u32) {
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
/// Resolution uses the UTS #39 *augmented* script sets, so the CJK writing
/// systems are handled: Han is treated as compatible with Japanese (Han +
/// Hiragana + Katakana), Korean (Han + Hangul), and Chinese (Han + Bopomofo).
/// Thus `日本語` (Han) mixed with kana stays single-script, and Han mixed with
/// Hangul stays single-script — but Hiragana mixed with Hangul (Japanese vs
/// Korean) is *not*, because those share no augmented script.
///
/// ```
/// use intl::unicode::spoof::is_single_script;
/// assert!(is_single_script("hello"));
/// // Latin + Cyrillic 'у' (U+0443) — mixed script.
/// assert!(!is_single_script("paуpal"));
/// assert!(is_single_script(""));
/// // Shared punctuation and digits keep Latin text single-script.
/// assert!(is_single_script("abc-123"));
/// // Han + Hiragana is Japanese — single script.
/// assert!(is_single_script("漢は"));
/// // Hiragana + Hangul is Japanese vs Korean — mixed script.
/// assert!(!is_single_script("は한"));
/// ```
#[must_use]
pub fn is_single_script(s: &str) -> bool {
    // Running intersection of the augmented script sets across the string.
    // `None` means "still unconstrained" (compatible with every script).
    let mut acc: Option<Vec<ScriptTok>> = None;
    for c in s.chars() {
        // A character whose Script_Extensions is exactly {Common} or {Inherited}
        // (shared punctuation, digits, combining marks, …) is compatible with
        // every script and imposes no constraint.
        if matches!(
            script_extensions(c),
            ScriptExtensions::Single(Script::Common) | ScriptExtensions::Single(Script::Inherited)
        ) {
            continue;
        }
        let aug = augmented_scripts(c);
        acc = Some(match acc {
            None => aug,
            Some(prev) => prev.into_iter().filter(|t| aug.contains(t)).collect(),
        });
        if acc.as_ref().is_some_and(Vec::is_empty) {
            return false; // empty intersection => mixed script
        }
    }
    true
}

/// A token in an augmented script set (UTS #39 §5.1). Regular scripts are
/// carried as [`ScriptTok::Scr`]; the three CJK "augmented" writing systems get
/// their own tokens so that, e.g., Han + Hiragana resolves to a single script
/// (both contain `Jpan`) while Hiragana + Hangul does not.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ScriptTok {
    /// Japanese: Han, Hiragana, or Katakana.
    Jpan,
    /// Korean: Han or Hangul.
    Kore,
    /// Chinese (Han-Bopomofo): Han or Bopomofo.
    Hanb,
    /// Any other script, unaugmented.
    Scr(Script),
}

/// The augmented script set of `c` (UTS #39 §5.1): its `Script_Extensions`, with
/// Han mapped to {Japanese, Korean, Han-Bopomofo}, Hiragana/Katakana to
/// Japanese, Hangul to Korean, and Bopomofo to Han-Bopomofo. Han thus stays
/// compatible with each individual CJK system without making those systems
/// compatible with each other.
fn augmented_scripts(c: char) -> Vec<ScriptTok> {
    let mut out: Vec<ScriptTok> = Vec::new();
    let mut push = |t: ScriptTok| {
        if !out.contains(&t) {
            out.push(t);
        }
    };
    for s in script_extensions(c).iter() {
        match s {
            Script::Han => {
                push(ScriptTok::Jpan);
                push(ScriptTok::Kore);
                push(ScriptTok::Hanb);
            }
            Script::Hiragana | Script::Katakana => push(ScriptTok::Jpan),
            Script::Hangul => push(ScriptTok::Kore),
            Script::Bopomofo => push(ScriptTok::Hanb),
            other => push(ScriptTok::Scr(other)),
        }
    }
    out
}
