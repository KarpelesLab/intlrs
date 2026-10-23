//! RBNF cardinal spell-out (locale-driven).
#![cfg(feature = "spellout")]
use intl::spellout::spell_cardinal as c;

fn strip(s: Option<String>) -> String {
    s.unwrap().replace('\u{ad}', "").replace('\u{a0}', " ") // drop soft hyphens / nbsp
}

#[test]
fn english() {
    assert_eq!(c("en", 0).as_deref(), Some("zero"));
    assert_eq!(c("en", 7).as_deref(), Some("seven"));
    assert_eq!(c("en", 42).as_deref(), Some("forty-two"));
    assert_eq!(c("en", 100).as_deref(), Some("one hundred"));
    assert_eq!(
        c("en", 1234).as_deref(),
        Some("one thousand two hundred thirty-four")
    );
    assert_eq!(c("en", 2000000).as_deref(), Some("two million"));
    assert_eq!(c("en", -42).as_deref(), Some("minus forty-two"));
}

#[test]
fn other_locales() {
    // French: 80 = "quatre-vingts", 91 = "quatre-vingt-onze".
    assert_eq!(strip(c("fr", 80)), "quatre-vingts");
    assert_eq!(strip(c("fr", 91)), "quatre-vingt-onze");
    assert_eq!(strip(c("fr", 100)), "cent");
    // German: 21 = "einundzwanzig", 100 = "einhundert".
    assert_eq!(strip(c("de", 21)), "einundzwanzig");
    assert_eq!(strip(c("de", 100)), "einhundert");
    // Spanish.
    assert_eq!(strip(c("es", 21)), "veintiuno");
    // Unsupported locale.
    assert_eq!(c("xx", 1), None);
}

#[test]
fn russian() {
    assert_eq!(c("ru", 0).as_deref(), Some("ноль"));
    assert_eq!(c("ru", 22).as_deref(), Some("двадцать два"));
    assert_eq!(c("ru", 200).as_deref(), Some("двести"));
    // Thousands are feminine and inflect by plural category.
    assert_eq!(c("ru", 1000).as_deref(), Some("одна тысяча"));
    assert_eq!(c("ru", 2000).as_deref(), Some("две тысячи"));
    assert_eq!(c("ru", 5000).as_deref(), Some("пять тысяч"));
    assert_eq!(c("ru", 21000).as_deref(), Some("двадцать одна тысяча"));
    assert_eq!(c("ru", 2_000_000).as_deref(), Some("два миллиона"));
    assert_eq!(
        c("ru", 1_234_567).as_deref(),
        Some("один миллион двести тридцать четыре тысячи пятьсот шестьдесят семь")
    );
    // `%spellout-numbering` has no `-x` rule; the sign reaches the cardinal's.
    assert_eq!(c("ru", -42).as_deref(), Some("минус сорок два"));
    assert_eq!(c("ru-RU", 3).as_deref(), Some("три"));
}

#[test]
fn ukrainian() {
    assert_eq!(c("uk", 2000).as_deref(), Some("дві тисячі"));
    assert_eq!(c("uk", 5_000_000).as_deref(), Some("пʼять мільйонів"));
    assert_eq!(
        c("uk", 1_234_567).as_deref(),
        Some("один мільйон двісті тридцять чотири тисячі пʼятсот шістдесят сім")
    );
    assert_eq!(c("uk", -7).as_deref(), Some("мінус сім"));
}

#[test]
fn polish() {
    assert_eq!(c("pl", 22).as_deref(), Some("dwadzieścia dwa"));
    assert_eq!(c("pl", 1000).as_deref(), Some("tysiąc"));
    assert_eq!(c("pl", 2000).as_deref(), Some("dwa tysiące"));
    assert_eq!(c("pl", 5000).as_deref(), Some("pięć tysięcy"));
    assert_eq!(c("pl", 12_000).as_deref(), Some("dwanaście tysięcy"));
    assert_eq!(c("pl", 22_000).as_deref(), Some("dwadzieścia dwa tysiące"));
    assert_eq!(
        c("pl", 1_234_567).as_deref(),
        Some("milion dwieście trzydzieści cztery tysiące pięćset sześćdziesiąt siedem")
    );
    assert_eq!(c("pl", -3).as_deref(), Some("minus trzy"));
}

#[test]
fn japanese() {
    assert_eq!(c("ja", 0).as_deref(), Some("〇"));
    assert_eq!(c("ja", 100).as_deref(), Some("百"));
    assert_eq!(c("ja", 1234).as_deref(), Some("千二百三十四"));
    assert_eq!(c("ja", 12_345).as_deref(), Some("一万二千三百四十五"));
    assert_eq!(
        c("ja", 123_456_789).as_deref(),
        Some("一億二千三百四十五万六千七百八十九")
    );
    assert_eq!(c("ja", -5).as_deref(), Some("マイナス五"));
}

#[test]
fn named_rule_sets() {
    use intl::spellout::spell_rule_set as r;
    assert_eq!(
        r("ru", "spellout-cardinal-feminine", 21).as_deref(),
        Some("двадцать одна")
    );
    assert_eq!(
        r("ru", "%spellout-cardinal-neuter", 1).as_deref(),
        Some("одно")
    );
    assert_eq!(
        r("uk", "spellout-cardinal-neuter", 1).as_deref(),
        Some("одне")
    );
    assert_eq!(
        r("pl", "spellout-cardinal-feminine", 2).as_deref(),
        Some("dwie")
    );
    assert_eq!(
        r("pl", "spellout-cardinal-masculine-personal", 5).as_deref(),
        Some("pięciu")
    );
    assert_eq!(
        r("ja", "spellout-cardinal-financial", 1234).as_deref(),
        Some("壱千弐百参拾四")
    );
    assert_eq!(
        strip(r("es", "spellout-cardinal-feminine", 21)),
        "veintiuna"
    );
    assert_eq!(
        strip(r("fr", "spellout-cardinal-feminine", 21)),
        "vingt-et-une"
    );
    // The default rule set is reachable by name too.
    assert_eq!(r("en", "spellout-cardinal", 7).as_deref(), Some("seven"));
    // Private (`%%`) rule sets, rule sets not vendored, unknown locales.
    assert_eq!(r("ru", "%%yj", 1), None);
    assert_eq!(r("ru", "spellout-cardinal-masculine-dative", 1), None);
    assert_eq!(r("ru", "spellout-numbering-year", 1), None);
    assert_eq!(r("xx", "spellout-cardinal", 1), None);
}

#[test]
fn engine_fixes() {
    // A leading apostrophe only protects whitespace; it is not output.
    assert_eq!(strip(c("fr", 201)), "deux cent un");
    assert_eq!(strip(c("pt", 1001)), "mil e um");
    // `=#,##0=` past the spelled range, with the locale's group separator.
    assert_eq!(
        c("en", 1_000_000_000_000_000_000).as_deref(),
        Some("1,000,000,000,000,000,000")
    );
    assert_eq!(
        c("en", i64::MIN).as_deref(),
        Some("minus 9,223,372,036,854,775,807")
    );
    assert_eq!(
        c("ru", 1_000_000_000_000_000_000).as_deref(),
        Some("1\u{a0}000\u{a0}000\u{a0}000\u{a0}000\u{a0}000\u{a0}000")
    );
    // Negatives through a rule set without its own `-x` rule.
    assert_eq!(strip(c("fr", -30)), "moins trente");
}
