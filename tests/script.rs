//! Script and Script_Extensions lookups.

use intl::unicode::{Script, ScriptExtensions, script, script_extensions};

#[test]
fn ascii_script() {
    assert_eq!(script('A'), Script::Latin);
    assert_eq!(script('z'), Script::Latin);
    assert_eq!(script('0'), Script::Common);
    assert_eq!(Script::Latin.long_name(), "Latin");
    assert_eq!(Script::Unknown.long_name(), "Unknown");
}

#[test]
fn ascii_script_extensions() {
    // A plain Latin letter has no explicit extensions: just its own script.
    let scx = script_extensions('A');
    assert_eq!(scx, ScriptExtensions::Single(Script::Latin));
    assert_eq!(scx.as_slice(), &[Script::Latin]);
    assert!(scx.contains(Script::Latin));
    assert!(!scx.contains(Script::Greek));
    assert_eq!(scx.iter().count(), 1);
}

#[cfg(feature = "latin1")]
#[test]
fn latin1_script_extensions() {
    // U+00B7 MIDDLE DOT is Common but used across many scripts.
    let scx = script_extensions('·');
    assert!(matches!(scx, ScriptExtensions::Multiple(_)));
    assert!(scx.contains(Script::Latin));
    assert!(scx.contains(Script::Greek));
    assert!(scx.iter().count() > 2);
}

#[cfg(feature = "bmp")]
#[test]
fn bmp_script() {
    assert_eq!(script('中'), Script::Han); // U+4E2D
    assert_eq!(script('٠'), Script::Arabic); // U+0660
    assert_eq!(script('\u{0301}'), Script::Inherited); // combining acute accent
    assert_eq!(script('Ω'), Script::Greek);
}
