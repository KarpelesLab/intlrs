//! English number spell-out (RBNF).
#![cfg(feature = "alloc")]
use intl::spellout::{spell_cardinal as c, spell_ordinal as o};

#[test]
fn cardinals() {
    assert_eq!(c(0), "zero");
    assert_eq!(c(7), "seven");
    assert_eq!(c(15), "fifteen");
    assert_eq!(c(42), "forty-two");
    assert_eq!(c(100), "one hundred");
    assert_eq!(c(123), "one hundred twenty-three");
    assert_eq!(c(1000), "one thousand");
    assert_eq!(c(1234), "one thousand two hundred thirty-four");
    assert_eq!(c(1000000), "one million");
    assert_eq!(c(2000005), "two million five");
    assert_eq!(c(-42), "minus forty-two");
}

#[test]
fn ordinals() {
    assert_eq!(o(1), "first");
    assert_eq!(o(2), "second");
    assert_eq!(o(3), "third");
    assert_eq!(o(5), "fifth");
    assert_eq!(o(8), "eighth");
    assert_eq!(o(12), "twelfth");
    assert_eq!(o(20), "twentieth");
    assert_eq!(o(21), "twenty-first");
    assert_eq!(o(100), "one hundredth");
    assert_eq!(o(1000000), "one millionth");
}
