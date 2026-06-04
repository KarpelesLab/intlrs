# `intl` roadmap

Goal: a **pure-Rust, `#![no_std]` analog of ICU** — the Unicode/CLDR algorithms
and formatters, with the same conformance guarantees, no C dependency, and
feature-selectable footprint.

This document tracks what exists, what's missing for ICU feature parity, and the
cross-cutting engineering work (fuzzing, benchmarks, profiling, and the
table-representation optimization that profiling will drive).

**Legend:** ✅ done & conformance-verified · 🟡 partial · ⬜ not started ·
🧱 needs CLDR data · 🔬 needs a new conformance/test corpus.

---

## Foundation — done

The `unicode` module (UCD-driven, codegen → committed tables, range-tier gated):

| Area | API | Conformance |
|------|-----|-------------|
| General_Category + predicates | `general_category`, `is_*`, `CharExt` | spot + std cross-check |
| Scripts / Script_Extensions | `script`, `script_extensions` | — |
| East Asian Width | `east_asian_width` | — |
| Numeric values | `numeric_value`, `numeric_type` | — |
| Case mapping/folding | `to_uppercase`/…/`case_fold`, `uppercase`/`lowercase`/`fold` | — |
| Normalization (UAX #15) | `nfc`/`nfd`/`nfkc`/`nfkd`, quick-check | ✅ NormalizationTest (20,034) |
| Collation (UTS #10, DUCET) | `collate::{compare,sort_key,Collator}` | ✅ CollationTest NON_IGNORABLE + SHIFTED |
| Segmentation (UAX #29) | `graphemes`/`words`/`sentences` | ✅ Grapheme/Word/SentenceBreakTest |

Infra: offline `codegen` (packaging-time), CI drift guard, deterministic output,
`no_std` + optional `alloc`, release-plz publishing.

---

## Phase 1 — remaining UCD/UAX algorithms (no CLDR)

Self-contained, driven by data we already vendor or can add; each has an official
conformance corpus. Highest value, lowest risk — do these next.

- 🟡🔬 **Line breaking (UAX #14)** — `line_breaks(&str)` yielding break
  opportunities (mandatory vs allowed). ~99.98% conformant against
  `LineBreakTest`; remaining gap is the LB19 CJK-quotation / East_Asian_Width
  sub-rules. *Done apart from those edge cases.*
- ✅🔬 **Bidirectional algorithm (UAX #9)** — `bidi::process` resolves embedding
  levels + visual order (X/W/N/I/L rules, isolates, paired brackets).
  ~99.996% on `BidiCharacterTest`; the residual handful are an override+isolate+
  embedding sos/eos edge case.
- ✅ **Identifiers (UAX #31)** — `is_xid_start`/`is_xid_continue`, identifier
  validation, default identifier syntax. Data: XID_Start/Continue (already in
  `DerivedCoreProperties.txt`). Small.
- ✅ **Confusables / spoof detection (UTS #39)** — `confusable_skeleton`,
  mixed-script & restriction-level checks. Data: `confusables.txt`,
  `IdentifierStatus.txt`, `IdentifierType.txt`. Conformance: examples in the
  spec; cross-check vs ICU `uspoof`.
- 🟡🔬 **IDNA / UTS #46** — domain-name `to_ascii`/`to_unicode` (Punycode +
  mapping + validation). Data: `IdnaMappingTable.txt`. Conformance:
  `IdnaTestV2.txt`. Depends on normalization (have) + Punycode (RFC 3492, small).
- 🟡 **Case completeness** — add: conditional/locale case (Turkic dotless-i,
  Greek final sigma, Lithuanian) via `SpecialCasing.txt` conditions; proper
  **titlecasing** (now unblocked by word segmentation); `Changes_When_*`
  predicates.
- ⬜ **More properties** (incremental, cheap): `Age` (`DerivedAge.txt`), `Block`
  (`Blocks.txt`), Bidi_Class accessor, Joining_Type/Group, Indic positional/
  syllabic, `Default_Ignorable_Code_Point`, full `White_Space`/`Math`/etc.,
  character `Name` ↔ codepoint, Hangul syllable name. Optional `name` lookup is
  large (consider a perfect-hash or trie).

---

## Phase 2 — engineering hardening (fuzz, bench, profile, optimize)

Stand this up early; it gates the table-representation optimization and protects
all the conformance work as the surface grows.

- 🟡 **Fuzzing** — in-process invariant fuzzing is live (`tests/robustness.rs`,
  20k deterministic random strings: no-panic + idempotence / round-trip /
  ordering invariants across normalization, segmentation, case, collation; runs
  in CI). Still to add: a `cargo-fuzz` project and **differential** fuzzing vs
  ICU4X / `unicode-*` / `std`, plus fuzzing the `codegen` parsers.
- ✅ **Benchmarks (`criterion`)** — `benches/throughput.rs` over ASCII/Latin/CJK/
  mixed corpora (general_category, nfc, nfd, graphemes, words, sort_key); the
  baseline for the trie optimization. `cargo bench --features alloc`.
- ⬜ **Profiling** — flamegraph the hot loops (normalization decompose,
  collation CEA generation, property lookups) on the bench corpora; identify
  whether lookups, branches, or allocation dominate.
- ⬜ **Table-representation optimization (profiling-driven).** Current property
  tables are paged `match` ("switch/case") — branchy, and large generated
  source (collation alone is ~42k lines; slow `rustc`). Evaluate and likely
  migrate the codegen emitter to a **two-level deduplicated trie** (the std /
  ICU4X representation): `block_index[cp>>8] → block_data[...]`, packed to the
  property's bit-width (1/2/4/8), identical blocks deduped. Expected wins:
  branchless O(1) lookups, big throughput gain on bulk/mixed text, far smaller
  generated source, faster compiles. Open questions to settle with data:
  - trie vs flat bit-packed blob (`include_bytes!`) vs keep-match, per property
    density;
  - how feature-tier gating maps onto tables (per-tier blobs vs full-table +
    bounds check) — the one real ergonomic regression vs `#[cfg]`-gated `match`;
  - keep `const fn` lookups; keep deterministic, diffable codegen output.
  Conformance suites are the safety net for the rewrite.
- 🟡 **`#![no_std]`/`no_alloc` CI matrix hardening** — CI builds on a bare-metal
  `thumbv7em-none-eabi` target (with and without `alloc`) to prove no `std`
  leakage. Still to add: `cargo-public-api` API-surface guard, MSRV check.
- ⬜ **Binary-size tracking** — measure `.text`/`.rodata` per feature tier; the
  whole point of tiers is size, so regressions should be visible.

---

## Phase 3 — CLDR foundation (the strategic dependency)

The formatting/locale half of ICU needs **CLDR**, a much larger and differently
shaped data source than UCD. This phase is the gate for Phase 4.

- 🧱 **CLDR ingestion pipeline** — extend `codegen` (or a sibling tool) to
  consume CLDR JSON, vendor a pinned CLDR version, generate compact per-locale
  tables. Decide locale-data packaging: baked-in (feature/locale-gated) vs
  loadable blobs. (ICU4X's `databake`/`provider` model is the reference design.)
- 🟡 **Locale identifiers (BCP 47 / UTS #35)** — parse/canonicalize `Locale`,
  language/script/region/variant/extensions, likely-subtags
  (add/remove/maximize), locale fallback & negotiation/matching.
- 🟡🔬 **Plural rules (CLDR)** — cardinal `PluralCategory` selection via
  `intl::plural` (rules compiled to a match; 224 locales, validated against the
  CLDR sample data). Cardinal + ordinal. Still to add: the compact `c`/`e` operand.

---

## Phase 4 — CLDR-dependent formatters & collators

Each needs Phase 3. These are where "ICU parity" mostly lives.

- 🟡 **Number formatting** — `intl::number::format_decimal` / `format_percent`
  (CLDR symbols + grouping/fraction patterns; curated locale set). Still to add:
  currency, scientific, compact, parsing, native digit systems.
- ⬜🧱 **Rule-based number formatting (RBNF)** — spell-out, ordinals, numbering
  systems. Needs the CLDR RBNF rule engine (locale-driven); an English-only
  version was rejected as out of place in an i18n crate.
- 🧱 **Calendars** — Gregorian + non-Gregorian (Islamic, Hebrew, Japanese,
  Persian, Buddhist, Indian, Chinese, …) and date arithmetic.
- 🧱 **Time zones (IANA tz)** — zone data, offsets, DST, zone display names.
- 🧱 🟡 **Date/Time formatting** (`intl::datetime`, Gregorian) — skeleton/pattern based (UTS #35),
  calendar- and zone-aware.
- 🧱 **Relative date/time** ("3 days ago"), **duration**, ✅ unit/measurement formatting (`intl::unit`), ✅ list formatting (`intl::list`), ✅ display names (`intl::display`, language/region).
- 🧱 ✅ MessageFormat (`intl::message`, subset) — ICU MessageFormat (and/or MessageFormat 2.0):
  select/plural/gender, nested args.
- 🧱 **Collation tailoring** — locale-tailored collators from CLDR (beyond DUCET
  root), collation strength/options, **string search** (collation-based) and
  **alphabetic index**.

---

## Phase 5 — large, mostly UCD/CLDR-hybrid

- ⬜🔬 **Transliteration** — script-to-script + rule-based transforms (e.g.
  Latin↔Cyrillic, Any-Latin, NFC/NFD as transforms). Rule engine + CLDR/ICU
  transform rules. Conformance: ICU transform test data.

---

## ICU component parity matrix

| ICU component | `intl` status |
|---------------|---------------|
| Character properties (`uprops`) | ✅ core · 🟡 long tail |
| Normalizer (`unorm2`) | ✅ |
| Case (`ucase`) | 🟡 (unconditional only) |
| BreakIterator grapheme/word/sentence (`ubrk`) | ✅ |
| BreakIterator line (`ubrk`) | ⬜ Phase 1 |
| Bidi (`ubidi`) | ⬜ Phase 1 |
| IDNA (`uidna`) | ⬜ Phase 1 |
| Spoof/confusables (`uspoof`) | ⬜ Phase 1 |
| Collator root/DUCET (`ucol`) | ✅ |
| Collator tailored + search | ⬜ Phase 4 |
| Locale / likely subtags (`uloc`) | ⬜ Phase 3 |
| Plural rules | ⬜ Phase 3 |
| Number/decimal/RBNF format | ⬜ Phase 4 |
| Date/time + calendars + tz | ⬜ Phase 4 |
| Units / list / relative / display names | ⬜ Phase 4 |
| MessageFormat | ⬜ Phase 4 |
| Transliterator (`utrans`) | ⬜ Phase 5 |

**Non-goals** (well-served by the Rust ecosystem): charset conversion
(`encoding_rs`), regex (`regex`), and the C/Java ICU APIs themselves.

---

## Cross-cutting principles

- **Conformance-first:** every algorithm ships with its official Unicode/CLDR
  conformance suite wired into CI (download large corpora on demand, as the
  collation job already does).
- **`#![no_std]`** always; `alloc` opt-in for whole-string/locale work.
- **Codegen → committed tables**, deterministic and diffable; no build.rs, no
  network at build time.
- **Feature-gated footprint** (range tiers today; locale gating later) so users
  pay only for what they use.
