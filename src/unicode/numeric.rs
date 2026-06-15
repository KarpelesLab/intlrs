//! The Unicode `Numeric_Type` and `Numeric_Value` properties.

use super::generated::numeric as tables;

/// The kind of numeric value a codepoint carries (`Numeric_Type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NumericType {
    /// `Decimal` — a decimal digit usable in positional notation (`Nd`); the
    /// value is always an integer `0..=9`.
    Decimal,
    /// `Digit` — a standalone digit (e.g. superscripts), integer `0..=9`.
    Digit,
    /// `Numeric` — any other numeric value, including fractions and large CJK
    /// number ideographs.
    Numeric,
}

/// An exact Unicode `Numeric_Value`, as a rational `numerator / denominator`.
///
/// Most codepoints are integers (`denominator == 1`); vulgar fractions such as
/// `½` use a larger denominator. The value may be negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumericValue {
    /// The signed numerator.
    pub numerator: i64,
    /// The positive denominator (`1` for integers).
    pub denominator: u32,
}

impl NumericValue {
    /// `true` if the value is an integer (denominator `1`).
    #[inline]
    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.denominator == 1
    }

    /// The integer value, or `None` if it is a proper fraction.
    #[inline]
    #[must_use]
    pub const fn to_i64(self) -> Option<i64> {
        if self.denominator == 1 {
            Some(self.numerator)
        } else {
            None
        }
    }

    /// The value as an `f64`.
    #[inline]
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.numerator as f64 / f64::from(self.denominator)
    }
}

/// The [`NumericType`] of `c`, or `None` if `c` has no numeric value.
#[inline]
#[must_use]
pub const fn numeric_type(c: char) -> Option<NumericType> {
    tables::numeric_type(c as u32)
}

/// The exact [`NumericValue`] of `c`, or `None` if `c` has no numeric value.
///
/// # Examples
/// ```
/// use intl::unicode::numeric_value;
/// assert_eq!(numeric_value('7').unwrap().to_i64(), Some(7));
/// assert_eq!(numeric_value('½').unwrap().as_f64(), 0.5);
/// assert_eq!(numeric_value('A'), None);
/// ```
#[inline]
#[must_use]
pub const fn numeric_value(c: char) -> Option<NumericValue> {
    tables::numeric_value(c as u32)
}

/// The [`NumericValue`] of an arbitrary Unicode scalar value.
#[inline]
#[must_use]
pub const fn numeric_value_u32(cp: u32) -> Option<NumericValue> {
    tables::numeric_value(cp)
}
