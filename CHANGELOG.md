# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3](https://github.com/KarpelesLab/intlrs/compare/v0.2.2...v0.2.3) - 2026-06-05

### Other

- reject malformed labels; make the conformance test honest
- revert the eos-skip hack; add exhaustive BidiTest.txt conformance
- gate out CLDR locales the parser mis-orders (correctness fix)
- Remove ROADMAP.md (every item complete)

## [0.2.2](https://github.com/KarpelesLab/intlrs/compare/v0.2.1...v0.2.2) - 2026-06-05

### Other

- Collation for_locale: data-driven from official CLDR rules
- Collation tailoring: unbounded weight allocation (pair-encoded primaries)
- transliteration component delivered (mark done)
- profiling addressed via benchmarks + size tracking
- add Armenian + Georgian romanizations
- Document collation tailoring capacity (gap-insertion limit)
- add alphabetic index (index_labels / index_bucket)
- Format spellout_ordinal regression test (rustfmt)
- Fix RBNF stack overflow / runaway on adversarial input
- add primary-strength string search (find/contains)
- add ordinal spell-out (spell_ordinal)

## [0.2.1](https://github.com/KarpelesLab/intlrs/compare/v0.2.0...v0.2.1) - 2026-06-05

### Other

- use then_some over then(||...) (clippy)
- set quantifiers ([..]+/*/?) and $0 match-reference
- Bundle more collation locales + fix 3-letter subtag resolution
- full conformance (91707/91707) via isolate-boundary eos fix
- Per-component Unicode features + iana-tz/full/names default (MSRV 1.86)
- Default to everything: full + alloc + names (opt out for size)
- Line breaking: full conformance (19338/19338) via LB15b EAW tailoring
- Bundle more locale collation tailorings (hu, ro, sq, uk, vi)
- add character-set sources ([abc x-z] > t)
- add before/after context (ICU `b { src } a > tgt` syntax)
- Add the full tabulated character Name database (names feature)
- line-break now 19335/19338 after LB21a fix
- Fix line-break LB21a: Hebrew-letter rule is HY-only, not HY|BA
- Add cargo-public-api surface guard (CI)
- multi-char targets (digraphs) + many more locale rules

## [0.2.0](https://github.com/KarpelesLab/intlrs/compare/v0.1.4...v0.2.0) - 2026-06-04

### Other

- Add a rule-based Transform to transliteration (x > y rewrites)
- Add collation expansions to the tailoring engine (ä → "ae")
- document collation strength/numeric/tailoring
- Add Tailoring::for_locale for well-known collations
- handle <<, <<<, = relation levels (not just primary)
- Add locale-tailored collation engine (Tailoring, primary reordering)
- Add numeric collation (Collator::with_numeric, natural sort)
- Add translit::any_ascii convenience (mixed-script -> ASCII)
- Add collation strength levels (Collator::with_strength)
- Add Greek->Latin transliteration (ELOT 743 / ISO 843)
- Add Cyrillic->Latin transliteration (ISO 9)
- Add remove_diacritics to the transliteration module
- Add transliteration module: Latin-ASCII fold (translit::latin_ascii)
- Fix format_compact panic on non-finite values
- Add ordinal number formatting (number::format_ordinal)
- Parse the compact c/e plural operand (PluralOperands)
- Add cargo-fuzz harness (unicode + formatters targets)
- Add bidi mirroring (bidi_mirror + is_bidi_mirrored)
- Add Changes_When_* predicates (UAX #44)
- Fix doc-comment placement on lowercase_str_lang
- Add Lithuanian locale casing (retained dot above)
- Track compiled binary size per tier in CI (sizeprobe example)
- Gate differential test on the bmp tier
- Format differential test (rustfmt)
- Add differential tests against std (case mapping + predicates)
- reject the trie migration; keep the paged match tables
- Add Joining_Group property (UAX #9)
- Add char_name for algorithmically-named characters (Hangul + ideographs)
- Add algorithmic Hangul syllable names
- Add MSRV CI job (Rust 1.70); fix f64::abs not in core on MSRV
- Add Turkic (tr/az) locale-aware casing
- Fix codegen clippy: allow too_many_arguments on emit_value_enum
- Add context-sensitive lowercasing with Greek Final_Sigma
- Add Indic_Positional_Category; factor a value-enum codegen helper
- Add Indic_Syllabic_Category property (UAX #44)
- Add Joining_Type property (Arabic shaping, UAX #9)
- Add the Chinese (lunisolar) calendar
- Add duration formatting; correct stale CLDR/formatter roadmap status
- Add Age and Block character properties (UCD)
- Fix panics on unvalidated input; fuzz every input-facing API
- Add full IANA time-zone support via the timezone-data crate
- Add native digit systems to number formatting
- Add compact number notation (number::format_compact)
- Add scientific number notation (number::format_scientific)
- Add locale-aware number parsing (number::parse_decimal)
- Add locale-driven RBNF cardinal spell-out
- Add POSIX TZ time zones (intl::timezone)
- Update the crate-level docs to describe the full library
- Render localized Persian dates; share the non-Gregorian renderer
- Add date arithmetic to DateTime (add_seconds/add_days/weekday)
- Render localized Islamic (Hijri) dates
- report embedded-data footprint (Phase 2 size tracking)
- Add Hebrew calendar (Dershowitz-Reingold arithmetic)
- Add Persian (Solar Hijri) and Japanese calendars
- Add localized GMT offset formatting (CLDR time zones, data-light)
- Add ISO-8601 timestamp parse/format to DateTime
- Add calendar conversions (no_std): Gregorian, Islamic, ISO week
- Parse BCP-47 extension and private-use subtags in Locale
- Add locale negotiation (best-match)
- Add likely-subtags maximize/minimize to Locale (CLDR / UTS #35)
- Add flexible date skeleton formatting (CLDR availableFormats)
- Remove English-only spellout
- Add English number spell-out (RBNF)
- Add Gregorian date/time formatting (CLDR / UTS #35)
- Add ICU MessageFormat subset (intl::message)
- Add measurement-unit formatting (CLDR / UTS #35)
- Add locale display names (CLDR / UTS #35)
- Embed CLDR formatter tables as no_std binary blobs; add currency

## [0.1.4](https://github.com/KarpelesLab/intlrs/compare/v0.1.3...v0.1.4) - 2026-06-04

### Other

- Add relative-time formatting + fix non-alloc tier builds
- Add locale-aware list formatting (CLDR / UTS #35)
- Add locale-aware number formatting (CLDR / UTS #35)
- Add CLDR ordinal plural rules
- Add CLDR cardinal plural rules (UTS #35)
- Implement the full bidi algorithm (UAX #9)
- Add common boolean properties (Math, Dash, Hex_Digit, …)
- Add BCP-47 locale parsing (Phase 3 foundation)
- build on a bare-metal no_std target
- Add Bidi_Class property + paragraph base direction (UAX #9)
- Add IDNA / Punycode (UTS #46 + RFC 3492)
- Add confusable / spoof detection (UTS #39)
- Phase 2: add robustness fuzzing + criterion benchmarks
- Add Unicode title-casing
- Add Unicode identifiers (UAX #31)
- Add line breaking (UAX #14)
- Add ROADMAP.md (path to ICU feature parity)
- Add sentence boundary segmentation (UAX #29)
- Add word boundary segmentation (UAX #29)
- Add grapheme cluster segmentation (UAX #29)

## [0.1.3](https://github.com/KarpelesLab/intlrs/compare/v0.1.2...v0.1.3) - 2026-06-04

### Other

- add crates.io, docs.rs, CI, and license badges
- Add whole-string case adaptors: uppercase/lowercase/fold
- Format codegen (separate package, not covered by cargo fmt --all)
- Add UTS #10 collation (DUCET root collation)

## [0.1.2](https://github.com/KarpelesLab/intlrs/compare/v0.1.1...v0.1.2) - 2026-06-04

### Other

- Add normalization quick-check (is_nfc/is_nfd/is_nfkc/is_nfkd)
- Make codegen a standalone packaging-time tool; drop the workspace
- Update crate description for the full unicode module surface
- Add Unicode normalization (NFD/NFC/NFKD/NFKC, UAX #15)

## [0.1.1](https://github.com/KarpelesLab/intlrs/compare/intl-v0.1.0...intl-v0.1.1) - 2026-06-04

### Other

- skip doctests in the per-tier test loop
- Add Numeric_Type / Numeric_Value; broaden CI tier testing
- Add full case mapping and folding
- Add Scripts + Script_Extensions (UAX #24)
- Add East Asian Width; generalize codegen to u32 value codes
- Point repository URL at renamed KarpelesLab/intlrs
