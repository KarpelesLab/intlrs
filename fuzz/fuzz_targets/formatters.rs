//! Fuzz the locale-aware formatters/parsers: arbitrary locale tags and inputs
//! must never panic (the class of bug behind real production outages).
#![no_main]
use libfuzzer_sys::fuzz_target;

// Interpret the first byte as a value selector, the rest as a UTF-8 locale/input.
fuzz_target!(|data: &[u8]| {
    let (sel, rest) = data.split_first().unwrap_or((&0, &[]));
    let s = core::str::from_utf8(rest).unwrap_or("en");
    let v = (*sel as f64) * 12_345.678 - 1000.0;

    let _ = intl::number::format_decimal(s, v);
    let _ = intl::number::format_currency(s, v, s);
    let _ = intl::number::format_compact(s, v);
    let _ = intl::number::format_scientific(s, v, 6);
    let _ = intl::number::parse_decimal(s, s);
    let _ = intl::spellout::spell_cardinal(s, *sel as i64);
    let _ = intl::locale::Locale::parse(s);
    let _ = intl::datetime::DateTime::parse_iso8601(s);
    let _ = intl::timezone::PosixTz::parse(s);
    let _ = intl::unit::format_duration(s, *sel as i64, intl::unit::UnitWidth::Long);
});
