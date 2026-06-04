# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
