//! Numeric_Type and Numeric_Value lookups.

use intl::unicode::{NumericType, numeric_type, numeric_value};

#[test]
fn ascii_numeric() {
    let v = numeric_value('7').unwrap();
    assert_eq!(v.to_i64(), Some(7));
    assert!(v.is_integer());
    assert_eq!(numeric_type('7'), Some(NumericType::Decimal));

    assert_eq!(numeric_value('A'), None);
    assert_eq!(numeric_type('A'), None);
}

#[cfg(feature = "latin1")]
#[test]
fn latin1_numeric() {
    // ½ U+00BD: rational 1/2, type Numeric.
    let half = numeric_value('½').unwrap();
    assert_eq!((half.numerator, half.denominator), (1, 2));
    assert!(!half.is_integer());
    assert_eq!(half.to_i64(), None);
    assert_eq!(half.as_f64(), 0.5);
    assert_eq!(numeric_type('½'), Some(NumericType::Numeric));

    // ² U+00B2 superscript two: value 2, type Digit.
    assert_eq!(numeric_value('²').unwrap().to_i64(), Some(2));
    assert_eq!(numeric_type('²'), Some(NumericType::Digit));
}

#[cfg(feature = "bmp")]
#[test]
fn bmp_numeric() {
    assert_eq!(numeric_value('三').unwrap().to_i64(), Some(3)); // CJK three U+4E09
    assert_eq!(numeric_type('三'), Some(NumericType::Numeric));
    assert_eq!(numeric_value('Ⅻ').unwrap().to_i64(), Some(12)); // Roman numeral U+216B
}
