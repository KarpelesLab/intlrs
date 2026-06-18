//! ICU MessageFormat subset.
#![cfg(feature = "message")]
use intl::message::{Arg, format_message as fm};

#[test]
fn plural_and_select() {
    let p = "{n, plural, one {# item} other {# items}}";
    assert_eq!(fm("en", p, &[("n", Arg::Num(0.0))]), "0 items");
    assert_eq!(fm("en", p, &[("n", Arg::Num(1.0))]), "1 item");
    assert_eq!(fm("en", p, &[("n", Arg::Num(7.0))]), "7 items");

    // Exact =N match wins over the plural category.
    let e = "{n, plural, =0 {none} one {one} other {#}}";
    assert_eq!(fm("en", e, &[("n", Arg::Num(0.0))]), "none");
    assert_eq!(fm("en", e, &[("n", Arg::Num(1.0))]), "one");
    assert_eq!(fm("en", e, &[("n", Arg::Num(9.0))]), "9");

    // select by string.
    let g = "{g, select, female {She} male {He} other {They}} won";
    assert_eq!(fm("en", g, &[("g", Arg::Str("male"))]), "He won");
    assert_eq!(fm("en", g, &[("g", Arg::Str("x"))]), "They won");
}

#[test]
fn nesting_and_args() {
    assert_eq!(
        fm("en", "Hello, {name}!", &[("name", Arg::Str("Ada"))]),
        "Hello, Ada!"
    );
    // Polish few/many in a real message.
    let pl = "{n, plural, one {# plik} few {# pliki} many {# plików} other {# pliku}}";
    assert_eq!(fm("pl", pl, &[("n", Arg::Num(1.0))]), "1 plik");
    assert_eq!(fm("pl", pl, &[("n", Arg::Num(3.0))]), "3 pliki");
    assert_eq!(fm("pl", pl, &[("n", Arg::Num(5.0))]), "5 plików");
    // Nested: a select containing a plural.
    let msg = "{g, select, other {{n, plural, one {a guest} other {# guests}}}}";
    assert_eq!(
        fm("en", msg, &[("g", Arg::Str("x")), ("n", Arg::Num(2.0))]),
        "2 guests"
    );
}
