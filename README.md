# intl

[![crates.io](https://img.shields.io/crates/v/intl.svg)](https://crates.io/crates/intl)
[![docs.rs](https://img.shields.io/docsrs/intl)](https://docs.rs/intl)
[![CI](https://github.com/KarpelesLab/intlrs/actions/workflows/ci.yml/badge.svg)](https://github.com/KarpelesLab/intlrs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Pure-Rust, `#![no_std]` internationalization primitives — a long-term, pure-Rust
analog of ICU (collation, number formatting, normalization, transliteration, …).
See [ROADMAP.md](ROADMAP.md) for the plan toward ICU feature parity.

The foundational layer, available today, is the **`unicode`** module: Unicode
rune analysis driven by the official Unicode Character Database (UCD), with
character properties compiled directly into Rust `match` dispatch by an offline
code generator — so every lookup is a `const fn`, allocates nothing, and needs
no runtime initialization.

- **`no_std`, no `alloc`** — usable in embedded, kernel, and WASM contexts.
- **Tables as code** — the UCD is converted into a two-level paged `match`
  ("switch/case") index, not parsed at runtime.
- **Feature-selectable ranges** — compile only the slice of the codepoint space
  you need. Anything outside the compiled range resolves to the neutral default
  (`Unassigned` / `false`), so every lookup is total.
- Targets **Unicode 17.0.0**.

## Usage

```toml
[dependencies]
intl = "0.1"
```

```rust
use intl::unicode::{general_category, GeneralCategory, CharExt};

assert_eq!(general_category('A'), GeneralCategory::UppercaseLetter);
assert_eq!(general_category('中'), GeneralCategory::OtherLetter);

assert!('A'.is_uppercase());
assert!('٣'.is_numeric());          // Arabic-Indic digit three
assert!(' '.is_whitespace());
assert!(!'\u{0378}'.is_assigned()); // a reserved codepoint
```

Every predicate exists both as a free `const fn` taking a `char`
(`intl::unicode::is_uppercase('A')`) and as a method via the `CharExt` trait
(`'A'.is_uppercase()`).

Normalization and collation (the latter behind the `alloc` feature):

```rust
use intl::unicode::{nfc, nfd};
assert_eq!(nfc("e\u{0301}".chars()).collect::<String>(), "é");
assert_eq!(nfd("é".chars()).collect::<String>(), "e\u{0301}");

// With the `alloc` feature:
use intl::unicode::collate::compare;
use std::cmp::Ordering;
assert_eq!(compare("café", "cafz"), Ordering::Less); // é (≈ e) sorts before z
```

Beyond the `unicode` module:

- `intl::locale` (alloc) parses and canonicalizes BCP-47 language tags
  (`Locale::parse("zh-hant-hk")` → `"zh-Hant-HK"`).
- `intl::plural` (`no_std`, no alloc) selects the CLDR `PluralCategory` for a
  number in a language — `plural_category` (cardinal) and `ordinal_category`
  ("1st"/"2nd"/"3rd"), rules compiled from CLDR into a `match`.
  `plural_category("pl", &PluralOperands::from_int(5))` → `Many`. Validated
  against the CLDR sample data (cardinal + ordinal).

- `intl::number` (alloc) formats numbers in a locale's conventions —
  `format_decimal("de", 1234.5)` → `"1.234,5"`, `format_decimal("hi", 1234567.0)`
  → `"12,34,567"` (Indian grouping), `format_percent("en", 0.5)` → `"50%"`,
  `format_currency("en", 1234.5, "USD")` → `"$1,234.50"`.

- `intl::list` (alloc) joins items with locale connectors —
  `format_list("en", &["a","b","c"], ListStyle::And)` → `"a, b, and c"`.
- `intl::relative` (alloc) formats relative times —
  `format_relative("en", -2, RelativeUnit::Hour)` → `"2 hours ago"`,
  `format_relative("en", -1, RelativeUnit::Day)` → `"yesterday"` (plural- and
  number-aware).
- `intl::display` (`no_std`, no alloc) gives locale display names —
  `language_name("fr", "de")` → `Some("allemand")`, `region_name("en", "JP")`
  → `Some("Japan")`.
- `intl::unit` (alloc) formats measurement units —
  `format_unit("en", 5.0, Unit::Kilometer, UnitWidth::Long)` → `"5 kilometers"`
  (plural- and number-aware, long/short widths).

These build out the CLDR/locale layer toward full ICU-style formatting. The
locale data is compiled by the offline codegen into flat binary blobs committed
under `src/cldr/` and embedded with `include_bytes!`, so the table layer is
`no_std` (no `alloc` dependency); only the formatting functions need `alloc`.

## Features

`default = ["bmp"]`. Range tiers are `ascii ⊂ latin1 ⊂ bmp ⊂ full` (below). The
**`alloc`** feature (still `no_std`) enables the allocating APIs
(`unicode::collate`, `unicode::spoof`, `unicode::idna`, `intl::locale`, …); it
implies `full`.

## Range tiers

Cargo features select how much of the codepoint space is compiled in, trading
coverage for binary size. The tiers are nested (each implies the smaller ones):

| feature  | codepoints compiled         |
|----------|-----------------------------|
| `ascii`  | `U+0000..=U+007F`           |
| `latin1` | `U+0000..=U+00FF`           |
| `bmp`    | `U+0000..=U+FFFF` (default)  |
| `full`   | `U+0000..=U+10FFFF`         |

```toml
# Latin-1 only, no default BMP tables:
intl = { version = "0.1", default-features = false, features = ["latin1"] }
# Everything, including supplementary planes:
intl = { version = "0.1", default-features = false, features = ["full"] }
```

A codepoint outside the compiled tier reports `GeneralCategory::Unassigned`
(and `false` for every boolean predicate) — exactly as a genuinely unassigned
codepoint would.

## What the `unicode` module covers

- `General_Category` (the 29 UAX #44 categories) and their major `Group`s,
  via `general_category` / `general_category_u32`.
- Boolean predicates: `is_alphabetic`, `is_uppercase`, `is_lowercase`,
  `is_whitespace` (from the derived Unicode properties), plus the
  category-derived `is_letter`, `is_mark`, `is_numeric`, `is_decimal_digit`,
  `is_punctuation`, `is_symbol`, `is_separator`, `is_control`, `is_format`,
  and `is_assigned`; plus the property predicates `is_math`, `is_dash`,
  `is_diacritic`, `is_hex_digit`, `is_quotation_mark`, `is_join_control`, and
  `is_default_ignorable`.
- **Segmentation** (UAX #29) — extended grapheme cluster, word, and sentence
  boundary iteration via `graphemes(&str)`, `words(&str)`, and `sentences(&str)`
  (each yielding `&str`, allocation-free). Grapheme breaking handles combining
  marks, Hangul, Indic conjuncts, regional-indicator flags, and emoji ZWJ
  sequences; word and sentence breaking implement the full WB / SB rule sets.
  All three validated against the official `GraphemeBreakTest` / `WordBreakTest`
  / `SentenceBreakTest` suites.
- **Line breaking** (UAX #14) — `line_breaks(&str)` yielding break opportunities
  (mandatory vs allowed). ~99.98% conformant against `LineBreakTest` (a few CJK
  quotation/East-Asian-Width edge cases remain).
- **Collation** (UTS #10) — DUCET root collation via `collate::compare` /
  `collate::Collator` (and `sort_key`), with non-ignorable or shifted variable
  handling. Validated against the full official `CollationTest` suite (both
  modes). Requires the `alloc` feature.
- **Normalization** (UAX #15) — `nfd`, `nfc`, `nfkd`, `nfkc` as streaming,
  allocation-free iterator adaptors over `Iterator<Item = char>`; quick-check
  helpers `is_nfc`/`is_nfd`/`is_nfkc`/`is_nfkd` (and tri-state
  `quick_check_*` → `IsNormalized`); plus `canonical_combining_class`.
  Validated against the full official `NormalizationTest.txt` conformance suite.
- Full, unconditional **case mapping** — per-`char` `to_uppercase`,
  `to_lowercase`, `to_titlecase`, `case_fold` (each a `CaseMapIter`, 1–3 chars,
  e.g. `ß` → `SS`), plus whole-stream adaptors `uppercase` / `lowercase` /
  `fold` over `Iterator<Item = char>` (e.g. `uppercase("Weiß".chars())`; no
  allocation). `fold` gives caseless comparison.
- `Script` and `Script_Extensions` (UAX #24) via `script` / `script_u32` and
  `script_extensions` / `script_extensions_u32` (`Script` enum with
  `.long_name()`; `ScriptExtensions` with `.contains()` / `.iter()`).
- `East_Asian_Width` (UAX #11) via `east_asian_width` / `east_asian_width_u32`
  (`EastAsianWidth` enum, with `.is_wide()`).
- **Bidirectional text** (UAX #9) — `bidi_class` (the `BidiClass` enum),
  `base_direction(&str)` (rules P2–P3), and (with `alloc`) the full reordering
  algorithm `bidi::process(&str, …) -> BidiInfo` (embedding levels + visual
  order). ~99.996% conformant against `BidiCharacterTest`.
- **Identifiers** (UAX #31) — `is_xid_start`, `is_xid_continue`, and
  `is_identifier(&str)` for default identifier validation.
- **Confusables / spoof detection** (UTS #39) — `spoof::skeleton`,
  `spoof::confusable`, and `spoof::is_single_script` (mixed-script detection).
  Requires `alloc`.
- **IDNA / Punycode** (UTS #46 / RFC 3492) — `idna::to_ascii` / `idna::to_unicode`
  for domain names (mapping + NFC + Punycode). The mapping/Punycode core passes
  every clean-success line of IdnaTestV2; the contextual validity rules
  (CheckBidi/CheckJoiners) are not yet enforced. Requires `alloc`.
- `Numeric_Type` and exact `Numeric_Value` via `numeric_type` and
  `numeric_value` / `numeric_value_u32` (`NumericValue` is a rational
  `numerator / denominator`, with `.to_i64()` / `.as_f64()`).
- `UNICODE_VERSION` of the embedded tables.

## Regenerating the tables

The committed files under `src/unicode/generated/` are produced from the
vendored UCD text files in `data/ucd/<version>/` by the `codegen` tool. It is a
**packaging-time** tool run only when updating the data or the Unicode version —
the published crate never builds or invokes it, and `codegen/` is a standalone
package (not a workspace member and not part of `intl`).

```sh
cargo run --manifest-path codegen/Cargo.toml
```

Output is deterministic and rustfmt-clean, so regeneration with the same data
yields no diff. To update the Unicode version, drop the new UCD files into
`data/ucd/<version>/`, bump the `version` in `codegen`, and re-run.

## License

MIT — see [LICENSE](LICENSE).
