# unicode

Pure-Rust, `#![no_std]` Unicode rune analysis driven by the official Unicode
Character Database (UCD). Character properties are compiled directly into Rust
`match` dispatch by an offline code generator, so every lookup is a `const fn`,
allocates nothing, and needs no runtime initialization.

- **`no_std`, no `alloc`** — usable in embedded, kernel, and WASM contexts.
- **Tables as code** — the UCD is converted into a two-level paged `match`
  ("switch/case") index at build-of-the-crate time, not parsed at runtime.
- **Feature-selectable ranges** — compile only the slice of the codepoint space
  you need. Anything outside the compiled range resolves to the neutral default
  (`Unassigned` / `false`), so every lookup is total.
- Targets **Unicode 17.0.0**.

## Usage

```toml
[dependencies]
unicode = "0.1"
```

```rust
use unicode::{general_category, GeneralCategory, CharExt};

assert_eq!(general_category('A'), GeneralCategory::UppercaseLetter);
assert_eq!(general_category('中'), GeneralCategory::OtherLetter);

assert!('A'.is_uppercase());
assert!('٣'.is_numeric());          // Arabic-Indic digit three
assert!(' '.is_whitespace());
assert!(!'\u{0378}'.is_assigned()); // a reserved codepoint
```

Every predicate exists both as a free `const fn` taking a `char`
(`unicode::is_uppercase('A')`) and as a method via the `CharExt` trait
(`'A'.is_uppercase()`).

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
unicode = { version = "0.1", default-features = false, features = ["latin1"] }
# Everything, including supplementary planes:
unicode = { version = "0.1", default-features = false, features = ["full"] }
```

A codepoint outside the compiled tier reports `GeneralCategory::Unassigned`
(and `false` for every boolean predicate) — exactly as a genuinely unassigned
codepoint would.

## What's covered

- `General_Category` (the 29 UAX #44 categories) and their major `Group`s,
  via `general_category` / `general_category_u32`.
- Boolean predicates: `is_alphabetic`, `is_uppercase`, `is_lowercase`,
  `is_whitespace` (from the derived Unicode properties), plus the
  category-derived `is_letter`, `is_mark`, `is_numeric`, `is_decimal_digit`,
  `is_punctuation`, `is_symbol`, `is_separator`, `is_control`, `is_format`,
  and `is_assigned`.
- `UNICODE_VERSION` of the embedded tables.

## Regenerating the tables

The committed files under `src/generated/` are produced from the vendored UCD
text files in `data/ucd/<version>/` by the `codegen` tool:

```sh
cargo run -p codegen
```

Output is deterministic and rustfmt-clean, so regeneration with the same data
yields no diff. To update the Unicode version, drop the new UCD files into
`data/ucd/<version>/`, bump the `version` in `codegen`, and re-run.

## License

MIT — see [LICENSE](LICENSE).
