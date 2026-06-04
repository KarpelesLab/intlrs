//! Bidirectional text properties (UAX #9).
//!
//! Currently provides the `Bidi_Class` property and paragraph base-direction
//! detection (rules P2–P3). The full reordering algorithm (the X/W/N/I/L rules
//! that resolve embedding levels and visual order) is not yet implemented.

use super::generated::bidi as gen;

/// The `Bidi_Class` of a codepoint (UAX #9). Order matches the generated table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum BidiClass {
    /// Left-to-Right
    L,
    /// Right-to-Left
    R,
    /// Right-to-Left Arabic
    AL,
    /// European Number
    EN,
    /// European Separator
    ES,
    /// European Terminator
    ET,
    /// Arabic Number
    AN,
    /// Common Separator
    CS,
    /// Nonspacing Mark
    NSM,
    /// Boundary Neutral
    BN,
    /// Paragraph Separator
    B,
    /// Segment Separator
    S,
    /// White Space
    WS,
    /// Other Neutral
    ON,
    /// Left-to-Right Embedding
    LRE,
    /// Left-to-Right Override
    LRO,
    /// Right-to-Left Embedding
    RLE,
    /// Right-to-Left Override
    RLO,
    /// Pop Directional Format
    PDF,
    /// Left-to-Right Isolate
    LRI,
    /// Right-to-Left Isolate
    RLI,
    /// First Strong Isolate
    FSI,
    /// Pop Directional Isolate
    PDI,
}

impl BidiClass {
    /// `true` if this is a strong right-to-left class (`R` or `AL`).
    #[inline]
    #[must_use]
    pub const fn is_rtl(self) -> bool {
        matches!(self, BidiClass::R | BidiClass::AL)
    }
}

/// The [`BidiClass`] of `c`.
#[inline]
#[must_use]
pub const fn bidi_class(c: char) -> BidiClass {
    gen::bidi_class(c as u32)
}

/// The [`BidiClass`] of an arbitrary Unicode scalar value.
#[inline]
#[must_use]
pub const fn bidi_class_u32(cp: u32) -> BidiClass {
    gen::bidi_class(cp)
}

/// A paragraph direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Left-to-right (base embedding level 0).
    LeftToRight,
    /// Right-to-left (base embedding level 1).
    RightToLeft,
}

/// The base paragraph direction of `s` per UAX #9 rules P2–P3: the direction of
/// the first strong character (`L`, `R`, or `AL`), skipping over isolated
/// sequences. Defaults to left-to-right when there is no strong character.
#[must_use]
pub fn base_direction(s: &str) -> Direction {
    let mut isolate_depth = 0u32;
    for c in s.chars() {
        match bidi_class(c) {
            BidiClass::LRI | BidiClass::RLI | BidiClass::FSI => isolate_depth += 1,
            BidiClass::PDI => isolate_depth = isolate_depth.saturating_sub(1),
            BidiClass::L if isolate_depth == 0 => return Direction::LeftToRight,
            BidiClass::R | BidiClass::AL if isolate_depth == 0 => return Direction::RightToLeft,
            _ => {}
        }
    }
    Direction::LeftToRight
}

/// `true` if `s` has a right-to-left base direction.
#[must_use]
pub fn is_rtl(s: &str) -> bool {
    base_direction(s) == Direction::RightToLeft
}
