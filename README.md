# intl

[![crates.io](https://img.shields.io/crates/v/intl.svg)](https://crates.io/crates/intl)
[![docs.rs](https://img.shields.io/docsrs/intl)](https://docs.rs/intl)
[![CI](https://github.com/KarpelesLab/intlrs/actions/workflows/ci.yml/badge.svg)](https://github.com/KarpelesLab/intlrs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Pure-Rust, `#![no_std]` internationalization primitives — a pure-Rust analog of
ICU (collation, number formatting, normalization, transliteration, …). The core
Unicode algorithms ship with their official conformance suites passing 100%
(Normalization, Collation, Grapheme/Word/Sentence, Line break, Bidi).

The foundational layer, available today, is the **`unicode`** module: Unicode
rune analysis driven by the official Unicode Character Database (UCD), with
character properties compiled directly into Rust `match` dispatch by an offline
code generator — so every lookup is a `const fn`, allocates nothing, and needs
no runtime initialization.

- **Everything by default** — the full Unicode range, the formatters, and the
  character-name database are on out of the box; opt *out* for size.
- **Always `no_std`** — and fully usable with no allocator (drop the default
  features and pick a range tier) for embedded, kernel, and WASM contexts.
- **Tables as code** — the UCD is converted into a two-level paged `match`
  ("switch/case") index, not parsed at runtime.
- **Feature-selectable ranges** — trim to only the slice of the codepoint space
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
  (`Locale::parse("zh-hant-hk")` → `"zh-Hant-HK"`), and adds/removes likely
  subtags (`Locale::maximize`: `en` → `en-Latn-US`; `Locale::minimize`:
  `zh-Hans-CN` → `zh`), and negotiates a best match between a user's requested
  locales and what's available (`negotiate`).
- `intl::plural` (`no_std`, no alloc) selects the CLDR `PluralCategory` for a
  number in a language — `plural_category` (cardinal) and `ordinal_category`
  ("1st"/"2nd"/"3rd"), rules compiled from CLDR into a `match`.
  `plural_category("pl", &PluralOperands::from_int(5))` → `Many`. Validated
  against the CLDR sample data (cardinal + ordinal).

- `intl::number` (alloc) formats numbers in a locale's conventions —
  `format_decimal("de", 1234.5)` → `"1.234,5"`, `format_decimal("hi", 1234567.0)`
  → `"12,34,567"` (Indian grouping), `format_percent("en", 0.5)` → `"50%"`,
  `format_currency("en", 1234.5, "USD")` → `"$1,234.50"`, `format_scientific`
  (`"1.2345E4"`), `format_compact` (`"1.5K"`, `"2.3M"`), and `parse_decimal`
  back to an `f64` (`parse_decimal("de", "1.234,5")` → `1234.5`), plus native
  digit systems (`to_numbering_system("2024", "arab")` → `"٢٠٢٤"`) and ordinals
  (`format_ordinal("en", 21)` → `"21st"`).

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
  (plural- and number-aware, long/short widths) — and durations:
  `format_duration("en", 3661, UnitWidth::Long)` → `"1 hour 1 minute 1 second"`.
- `intl::message` (alloc) is a subset of ICU MessageFormat — `{arg}`
  substitution, `plural`/`selectordinal` (with `=N` and `#`), and `select`,
  composing the plural rules and number formatting.
- `intl::datetime` (alloc) formats Gregorian dates/times —
  `format_date("en", &dt, DateStyle::Long)` → `"June 4, 2026"`,
  `format_date("de", &dt, DateStyle::Long)` → `"4. Juni 2026"` (CLDR patterns,
  month/weekday names, am/pm; weekday via Sakamoto's algorithm). Also
  `format_skeleton("en", &dt, "yMMMd")` → `"Jun 4, 2026"` (flexible field-set
  formatting), and renders **Islamic (Hijri)** and **Persian** dates with
  localized month names (`format_islamic_date("en", 1445, 9, 1, DateStyle::Long)`
  → `"Ramadan 1, 1445 AH"`; `format_persian_date` likewise).
- `intl::spellout` spells integers out in words via the CLDR RBNF rules
  (locale-driven) — `spell_cardinal("en", 1234)` → `"one thousand two hundred
  thirty-four"`, `spell_cardinal("fr", 80)` → `"quatre-vingts"`, and ordinals via `spell_ordinal("en", 21)` → `"twenty-first"`. *(alloc)*
- `intl::timezone` parses a POSIX `TZ` string (`"PST8PDT,M3.2.0,M11.1.0/2"`)
  and computes the UTC offset / DST state for any date. With the **`iana-tz`**
  feature it also loads the full **IANA tz database** (via the embedded
  `timezone-data` crate): `load_zone("America/New_York")` then `offset_at` /
  `abbrev_at` / `is_dst_at` / `to_local` for any instant, with historical
  transitions (the `iana-tz` feature, on by default).
- `intl::calendar` (`no_std`, no alloc) converts dates between the Gregorian,
  civil (tabular) Islamic, Persian (Solar Hijri), Hebrew, and Chinese (lunisolar,
  1900–2099 via an embedded lunar table) calendars through the Julian Day Number,
  gives the Japanese era/year, plus ISO-8601 week dates and day-of-week — pure
  integer arithmetic. `DateTime` also does
  ISO-8601 timestamp parse/format, date arithmetic (`add_seconds`/`add_days`/
  `weekday`, leap- and carry-aware), and `format_gmt_offset` renders a localized
  UTC offset (`GMT+05:30`, `UTC−08:00`).

- `intl::translit` (alloc) transliterates: `latin_ascii` ("café"→"cafe",
  "Straße"→"Strasse"), `remove_diacritics`, `cyrillic_to_latin` (ISO 9),
  `greek_to_latin` (ELOT/ISO 843), and `any_ascii` for best-effort mixed-script
  ASCII ("Москва café Αθήνα"→"Moskva cafe Athina").

These build out the CLDR/locale layer toward full ICU-style formatting. The
locale data is compiled by the offline codegen into flat binary blobs committed
under `src/cldr/` and embedded with `include_bytes!`, so the table layer is
`no_std` (no `alloc` dependency); only the formatting functions need `alloc`.

## Features

**Everything on by default, opt out for size.** Out of the box you get the
**whole Unicode codepoint space** (`full`), every allocating API (`alloc`), every
Unicode **algorithm component**, the **full character-name database** (`names`),
and the **IANA time-zone database** (`iana-tz`) — all `no_std`. MSRV 1.88.

To shrink the build, set `default-features = false` and enable only the range
tier + the components you want.

### Algorithm components (opt out individually)

Each component gates its module *and* its (sometimes large) generated table, so
disabling one removes it from the build entirely:

| feature         | what it provides                         | gated table |
|-----------------|------------------------------------------|-------------|
| `normalization` | UAX #15 NFC/NFD/NFKC/NFKD                 | 659 KB |
| `segmentation`  | UAX #29 grapheme/word/sentence + UAX #14 line | 429 KB |
| `bidi`          | UAX #9 bidirectional algorithm           | 61 KB |
| `case`          | case mapping/folding (→ normalization, segmentation) | 284 KB |
| `collation`     | UTS #10 collation (→ case, alloc)        | **1.9 MB** |
| `idna`          | UTS #46 IDNA (→ normalization, alloc)    | 464 KB |
| `confusables`   | UTS #39 confusable/skeleton (→ normalization, alloc) | 369 KB |
| `identifiers`   | UAX #31 identifiers                      | — |
| `names`         | full character-name database (→ alloc)   | 1.3 MB |

The foundational property lookups — `General_Category`, predicates, scripts,
East Asian Width, numeric values — are always available and not gated.

```toml
# Just normalization, nothing else:
intl = { version = "0.1", default-features = false, features = ["full", "normalization"] }
# Just collation (pulls in case + normalization + alloc automatically):
intl = { version = "0.1", default-features = false, features = ["collation"] }
# Everything except the 1.9 MB collation table and IANA tz:
intl = { version = "0.1", default-features = false, features = [
    "names", "segmentation", "bidi", "case", "idna", "confusables", "identifiers",
] }
```

## Range tiers (opt out for size)

The range tiers select how much of the codepoint space is compiled in, trading
coverage for binary size. They are nested (each implies the smaller ones):

| feature  | codepoints compiled              |
|----------|----------------------------------|
| `ascii`  | `U+0000..=U+007F`                |
| `latin1` | `U+0000..=U+00FF`                |
| `bmp`    | `U+0000..=U+FFFF`                |
| `full`   | `U+0000..=U+10FFFF` (default)    |

```toml
# Everything, the default:
intl = "0.1"
# Trim to the BMP and drop alloc/names for a smaller no_std build:
intl = { version = "0.1", default-features = false, features = ["bmp"] }
# Minimal: ASCII tables only:
intl = { version = "0.1", default-features = false, features = ["ascii"] }
# iana-tz is already on by default; drop it (and pick what you need) to avoid the dep:
intl = { version = "0.1", default-features = false, features = ["full", "alloc"] }
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
  handling, **strength levels** (`with_strength`: accent-/case-insensitive),
  **numeric ordering** (`with_numeric`: `file2 < file10`), and **locale
  tailoring** (`Tailoring::parse("&z < å < ä < ö")` / `Tailoring::for_locale("sv")`
  for primary reordering). Validated against the full official `CollationTest`
  suite (both modes). Requires the `alloc` feature.
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
