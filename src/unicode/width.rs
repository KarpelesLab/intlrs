//! The Unicode `East_Asian_Width` property (UAX #11).

/// The `East_Asian_Width` of a codepoint (UAX #11).
///
/// Codepoints that are unassigned — or that fall outside the compiled range
/// tier — report [`EastAsianWidth::Neutral`], the property's default value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EastAsianWidth {
    /// `N` — Neutral (non-East-Asian; also the default).
    Neutral = 0,
    /// `A` — Ambiguous (width depends on context).
    Ambiguous,
    /// `H` — Halfwidth.
    Halfwidth,
    /// `W` — Wide.
    Wide,
    /// `F` — Fullwidth.
    Fullwidth,
    /// `Na` — Narrow.
    Narrow,
}

impl EastAsianWidth {
    /// The property-value abbreviation, e.g. `"W"` for [`EastAsianWidth::Wide`].
    #[inline]
    #[must_use]
    pub const fn abbr(self) -> &'static str {
        match self {
            EastAsianWidth::Neutral => "N",
            EastAsianWidth::Ambiguous => "A",
            EastAsianWidth::Halfwidth => "H",
            EastAsianWidth::Wide => "W",
            EastAsianWidth::Fullwidth => "F",
            EastAsianWidth::Narrow => "Na",
        }
    }

    /// `true` if this width is rendered as two columns in a typical fixed-width
    /// East-Asian context: [`Wide`](EastAsianWidth::Wide) or
    /// [`Fullwidth`](EastAsianWidth::Fullwidth).
    ///
    /// [`Ambiguous`](EastAsianWidth::Ambiguous) is *not* counted as wide here;
    /// its width is context-dependent and the caller must decide.
    #[inline]
    #[must_use]
    pub const fn is_wide(self) -> bool {
        matches!(self, EastAsianWidth::Wide | EastAsianWidth::Fullwidth)
    }
}

/// The [`EastAsianWidth`] of `c`.
#[inline]
#[must_use]
pub const fn east_asian_width(c: char) -> EastAsianWidth {
    super::generated::east_asian_width::east_asian_width(c as u32)
}

/// The [`EastAsianWidth`] of an arbitrary Unicode scalar value.
#[inline]
#[must_use]
pub const fn east_asian_width_u32(cp: u32) -> EastAsianWidth {
    super::generated::east_asian_width::east_asian_width(cp)
}
