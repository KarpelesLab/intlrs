# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
