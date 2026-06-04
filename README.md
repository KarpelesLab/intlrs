# intl

[![crates.io](https://img.shields.io/crates/v/intl.svg)](https://crates.io/crates/intl)
[![docs.rs](https://img.shields.io/docsrs/intl)](https://docs.rs/intl)
[![CI](https://github.com/KarpelesLab/intlrs/actions/workflows/ci.yml/badge.svg)](https://github.com/KarpelesLab/intlrs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Pure-Rust, `#![no_std]` internationalization primitives — a long-term, pure-Rust
analog of ICU (collation, number formatting, normalization, transliteration, …).

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

## Features

`default = ["bmp"]`. Range tiers are `ascii ⊂ latin1 ⊂ bmp ⊂ full` (below). The
**`alloc`** feature (still `no_std`) enables the `unicode::collate` module; it
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
  and `is_assigned`.
- **Segmentation** (UAX #29) — extended grapheme cluster and word boundary
  iteration via `graphemes(&str)` and `words(&str)` (yielding `&str`,
  allocation-free). Grapheme breaking is correct for combining marks, Hangul,
  Indic conjuncts, regional-indicator flags, and emoji ZWJ sequences; word
  breaking implements the full WB rule set. Both validated against the official
  `GraphemeBreakTest` / `WordBreakTest` suites.
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
