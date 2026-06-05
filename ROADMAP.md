# `intl` roadmap

Goal: a **pure-Rust, `#![no_std]` analog of ICU** — the Unicode/CLDR algorithms
and formatters, with the same conformance guarantees, no C dependency, and
feature-selectable footprint (everything on by default; opt out for size).

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

- ✅🔬 **Line breaking (UAX #14)** — `line_breaks(&str)` yielding break
  opportunities (mandatory vs allowed). **19338/19338 LineBreakTest lines pass**
  (full conformance), including the LB15b/LB19a East-Asian-width quotation rules.
- ✅🔬 **Bidirectional algorithm (UAX #9)** — `bidi::process` resolves embedding
  levels + visual order (X/W/N/I/L rules, isolates, paired brackets).
  **100% on `BidiCharacterTest` (91707/91707)**, including the override+isolate+
  embedding sos/eos boundary cases (eos skips deeper sibling embeddings).
- ✅ **Identifiers (UAX #31)** — `is_xid_start`/`is_xid_continue`, identifier
  validation, default identifier syntax. Data: XID_Start/Continue (already in
  `DerivedCoreProperties.txt`). Small.
- ✅ **Confusables / spoof detection (UTS #39)** — `confusable_skeleton`,
  mixed-script & restriction-level checks. Data: `confusables.txt`,
  `IdentifierStatus.txt`, `IdentifierType.txt`. Conformance: examples in the
  spec; cross-check vs ICU `uspoof`.
- ✅🔬 **IDNA / UTS #46** — domain-name `to_ascii`/`to_unicode` (Punycode +
  mapping + validation). Data: `IdnaMappingTable.txt`. Conformance:
  `IdnaTestV2.txt`. Depends on normalization (have) + Punycode (RFC 3492, small).
- ✅ **Case completeness** — ✅ titlecasing (`titlecase`), ✅ Greek Final_Sigma
  (`lowercase_str`), ✅ Turkic dotted/dotless-i (`lowercase_str_lang` /
  `uppercase_str_lang` for tr/az), ✅ Lithuanian retained-dot
  (`lowercase_str_lang` for lt), ✅ `Changes_When_*` predicates. Case completeness: done.
- ✅ **More properties**: ✅ `Age` (`DerivedAge.txt`),
  ✅ `Block` (`Blocks.txt`), ✅ `Joining_Type` (`DerivedJoiningType.txt`),
  ✅ `Indic_Syllabic_Category`, ✅ `Indic_Positional_Category`, ✅ `Joining_Group`, ✅ Bidi_Class accessor (`bidi_class`),
  ✅ `Default_Ignorable_Code_Point` / `Math` / `Dash` / `Diacritic` /
  `Hex_Digit` / `White_Space` / `Quotation_Mark` / `Join_Control`. Still:
  ✅ the tabulated `Name`
  database behind the `names` feature (`unicode::name`, ~1.3 MB blob). ✅ Algorithmic names (`char_name` / `hangul_syllable_name`): Hangul
  syllables + CJK/Tangut/Khitan/Nüshu ideograph ranges.

---

## Phase 2 — engineering hardening (fuzz, bench, profile, optimize)

Stand this up early; it gates the table-representation optimization and protects
all the conformance work as the surface grows.

- ✅ **Fuzzing** — in-process invariant fuzzing is live (`tests/robustness.rs`,
  20k deterministic random strings: no-panic + idempotence / round-trip /
  ordering invariants across normalization, segmentation, case, collation; runs
  in CI). ✅ differential
  testing vs `std` (`tests/differential.rs`: case mapping + predicates over the
  stable ranges). ✅ a `cargo-fuzz`
  harness (`fuzz/`, targets `unicode` + `formatters`; built in CI, run via
  `cargo +nightly fuzz run`). (`codegen` runs only on trusted, committed UCD/CLDR
  data at packaging time — not a fuzz target.)
- ✅ **Benchmarks (`criterion`)** — `benches/throughput.rs` over ASCII/Latin/CJK/
  mixed corpora (general_category, nfc, nfd, graphemes, words, sort_key); the
  throughput baseline. `cargo bench --features alloc`.
- ✅ **Profiling** — the criterion benchmarks give per-corpus throughput for the
  hot loops (general_category, nfc/nfd, graphemes/words, sort_key), and CI tracks
  compiled size per tier, so a regression is visible without a dedicated
  flamegraph. Since the `match` table representation is settled (below), a
  flamegraph is purely opportunistic — to be run only if a benchmark regression
  surfaces (none has). The measurement infrastructure is in place; nothing
  outstanding gates it.
- ✅ **Table-representation: keep the paged `match` (trie migration rejected).**
  Decision: do *not* migrate the property tables to a runtime two-level trie.
  The paged `match` ("switch/case") is what `rustc`/LLVM lower to dense jump
  tables and range comparisons — the lookup stays in the instruction stream with
  no data-dependent memory loads. A runtime trie (`block_index[cp>>8]` →
  `block_data[...]`) replaces that with two dependent array loads, i.e. *slower*
  lookups and more cache pressure on the hot path, to save generated-source size
  and compile time. LLVM optimizes the generated `match` very well, so the
  representation stays as-is. (The CLDR side already uses `include_bytes!` blobs
  where the data is genuinely table-shaped; that's a separate, data-driven
  choice, not a property-lookup hot path.) Large generated source / compile time
  is accepted as the cost of the fastest lookups and `#[cfg]`-tier gating.
- ✅ **`#![no_std]`/`no_alloc` CI matrix hardening** — CI builds on a bare-metal
  `thumbv7em-none-eabi` target (with and without `alloc`) to prove no `std`
  leakage. ✅ MSRV check
  (builds the default + all tiers on Rust 1.86). ✅ `cargo-public-api`
  surface guard (CI diffs against committed public-api.txt).
- ✅ **Binary-size tracking** — CI reports the embedded-data footprint
  (generated tables + CLDR blobs) and the compiled `.text`/`.data` per feature
  tier via a `sizeprobe` example built in release and measured with `size` (the
  whole point of tiers is footprint, so tier-over-tier growth is visible in the
  log).

---

## Phase 3 — CLDR foundation (the strategic dependency)

The formatting/locale half of ICU needs **CLDR**, a much larger and differently
shaped data source than UCD. This phase is the gate for Phase 4.

- ✅ **CLDR ingestion pipeline** — `codegen` consumes a pinned CLDR (v48) JSON
  set, vendored and committed under `data/cldr/48/`, and transforms it into
  committed binary blobs under `src/cldr/` (`include_bytes!`'d by the `no_std`
  `crate::cldr` module). ~15 transforms (numbers, currency, calendars, units,
  lists, relative, display, likely-subtags, RBNF, compact, numbering systems,
  timezone formats). Deterministic; the CI drift guard regenerates from the
  committed data with no network.
- ✅ **Locale identifiers (BCP 47 / UTS #35)** — parse/canonicalize `Locale`,
  language/script/region/variant/extensions (incl. `-u-`/`-t-`/`-x-`),
  likely-subtags (maximize/minimize), negotiation/matching.
- ✅🔬 **Plural rules (CLDR)** — cardinal `PluralCategory` selection via
  `intl::plural` (rules compiled to a match; 224 locales, validated against the
  CLDR sample data). Cardinal + ordinal, including the compact `c`/`e` operand
  (`PluralOperands::parse("1.2c6")`). Phase 3 complete.

---

## Phase 4 — CLDR-dependent formatters & collators

Each needs Phase 3. These are where "ICU parity" mostly lives.

- ✅ **Number formatting** — `intl::number`: decimal, percent, currency,
  scientific, compact, parsing (`parse_decimal`), and native digit systems
  (`to_numbering_system`). CLDR symbols + grouping/fraction patterns, curated
  locale set.
- ✅🧱 **Rule-based number formatting (RBNF)** — `intl::spellout::spell_cardinal`
  + `spell_ordinal` are a locale-driven CLDR RBNF engine: cardinal and ordinal
  spell-out for the curated locale set (en/de/fr/nl/es/it/pt/sv). ✅ ordinal
  *formatting* (`number::format_ordinal`, "21st"). Still: fractional/year forms.
- ✅ **Calendars** — `intl::calendar`: Gregorian, civil Islamic, Persian (Solar
  Hijri), Hebrew, **Chinese** (lunisolar, 1900–2099 via an embedded lunar table),
  Japanese (era/year), and ISO week dates / day-of-week, all via a Julian-Day-
  Number pivot. Islamic + Persian localized date *rendering*
  (`datetime::format_islamic_date` / `format_persian_date`).
- ✅ **Time zones** — `intl::timezone`: POSIX TZ rules (no_std), plus the full
  IANA tz database behind the `iana-tz` feature (via the embedded `timezone-data`
  crate): historical transitions, DST, abbreviations. Still: zone display names.
- ✅ **Date/Time formatting** (`intl::datetime`, Gregorian) — date/time/datetime
  styles, skeleton/pattern based (UTS #35), ISO-8601 I/O, date arithmetic,
  localized GMT offsets, Islamic/Persian rendering.
- ✅ **Relative date/time** (`intl::relative`, "3 days ago"), ✅ **duration**
  (`intl::unit::format_duration`), ✅ unit/measurement formatting (`intl::unit`),
  ✅ list formatting (`intl::list`), ✅ display names (`intl::display`).
- 🧱 ✅ MessageFormat (`intl::message`, subset) — ICU MessageFormat (and/or MessageFormat 2.0):
  select/plural/gender, nested args.
- 🟡🧱 **Collation tailoring** — ✅ strength levels (`Collator::with_strength`),
  ✅ numeric ordering (`with_numeric`, natural sort), ✅ a locale-tailoring rule
  engine (`Tailoring::parse` for `<`/`<<`/`<<<`/`=` + expansions + multi-char
  digraph targets) with ✅ ~30 bundled locales via `Tailoring::for_locale`.
  **Remaining boundary:** exhaustive CLDR per-locale tailoring (all ~700 locales)
  is *not* "just data" — many CLDR rules use the full ICU syntax (`[import]`,
  `[before N]`, `/extension`, logical reset positions) and need real ICU-style
  **weight allocation** (this crate uses a conformance-verified gap-insertion
  approximation that handles the common reorderings but cannot place arbitrarily
  many letters). Shipping all locales through the approximate engine would yield
  *incorrect* sort orders, so it is deliberately not done; the engine parses any
  rule string a caller supplies. ✅ collation-based **string search**
  (`collate::find`/`contains`) and ✅ a locale-tailored **alphabetic index**
  (`collate::index_labels`/`index_bucket`).

---

## Phase 5 — large, mostly UCD/CLDR-hybrid

- 🟡 **Transliteration** — ✅ `latin_ascii` (Latin→ASCII fold), ✅ Cyrillic→Latin
  (ISO 9), ✅ Greek→Latin (ELOT/ISO 843), ✅ `remove_diacritics`, ✅ `any_ascii`,
  and ✅ a general **rule-based transform engine** (`translit::Transform`:
  longest-match `x > y` rewrites with before/after **context**, character-set
  sources, set **quantifiers** `+`/`*`/`?`, and a `$0` match-reference). Still
  welcome: more script romanizations (Arabic, Devanagari, Han→Latin, …) and the
  remaining ICU rule grammar (named `$1` capture groups, `[import]`).

---

## ICU component parity matrix

| ICU component | `intl` status |
|---------------|---------------|
| Character properties (`uprops`) | ✅ (incl. tabulated `Name` DB) |
| Normalizer (`unorm2`) | ✅ 100% NormalizationTest |
| Case (`ucase`) | ✅ (full + Greek/Turkic/Lithuanian/`Changes_When_*`) |
| BreakIterator grapheme/word/sentence (`ubrk`) | ✅ |
| BreakIterator line (`ubrk`) | ✅ 100% LineBreakTest (19338/19338) |
| Bidi (`ubidi`) | ✅ 100% BidiCharacterTest (91707/91707) |
| IDNA (`uidna`) | ✅ |
| Spoof/confusables (`uspoof`) | ✅ |
| Collator root/DUCET (`ucol`) | ✅ 100% CollationTest (both modes) |
| Collator tailored + search | 🟡 tailoring engine + ~30 locales + search (`find`) + alphabetic index |
| Locale / likely subtags (`uloc`) | ✅ |
| Plural rules | ✅ (cardinal + ordinal) |
| Number/decimal/RBNF format | ✅ (RBNF cardinal; ordinal spell-out partial) |
| Date/time + calendars + tz | ✅ |
| Units / list / relative / display names | ✅ |
| MessageFormat | ✅ (subset) |
| Transliterator (`utrans`) | ✅ (5 transforms + rule engine; more scripts welcome) |

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
