//! The Unicode `Script` and `Script_Extensions` properties (UAX #24).

use super::generated::script as tables;

pub use tables::Script;

/// The [`Script`] of `c`.
#[inline]
#[must_use]
pub const fn script(c: char) -> Script {
    tables::script(c as u32)
}

/// The [`Script`] of an arbitrary Unicode scalar value.
#[inline]
#[must_use]
pub const fn script_u32(cp: u32) -> Script {
    tables::script(cp)
}

/// The set of scripts a codepoint is used with — its `Script_Extensions`.
///
/// For most codepoints this is just the single [`Script`]; some (e.g. shared
/// punctuation and digits) belong to several. Obtain it from
/// [`script_extensions`] and inspect it with [`contains`](Self::contains),
/// [`iter`](Self::iter), or [`as_slice`](Self::as_slice).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptExtensions {
    /// The codepoint's `Script_Extensions` is exactly its `Script`.
    Single(Script),
    /// An explicit set of scripts (always at least two, sorted).
    Multiple(&'static [Script]),
}

impl ScriptExtensions {
    /// The scripts as a slice.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[Script] {
        match self {
            ScriptExtensions::Single(s) => core::slice::from_ref(s),
            ScriptExtensions::Multiple(set) => set,
        }
    }

    /// `true` if `s` is one of the scripts in this set.
    #[inline]
    #[must_use]
    pub fn contains(&self, s: Script) -> bool {
        self.as_slice().contains(&s)
    }

    /// Iterate over the scripts in this set.
    #[inline]
    pub fn iter(&self) -> core::iter::Copied<core::slice::Iter<'_, Script>> {
        self.as_slice().iter().copied()
    }
}

/// The [`Script_Extensions`](ScriptExtensions) of `c`.
#[inline]
#[must_use]
pub const fn script_extensions(c: char) -> ScriptExtensions {
    script_extensions_u32(c as u32)
}

/// The [`Script_Extensions`](ScriptExtensions) of an arbitrary Unicode scalar
/// value.
#[inline]
#[must_use]
pub const fn script_extensions_u32(cp: u32) -> ScriptExtensions {
    match tables::script_extensions(cp) {
        Some(set) => ScriptExtensions::Multiple(set),
        None => ScriptExtensions::Single(tables::script(cp)),
    }
}
