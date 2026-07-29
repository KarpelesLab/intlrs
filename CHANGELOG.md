# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- *(unit)* the full ECMA-402 sanctioned unit set and general compound units.
  `Unit` grew from 28 to 47 variants, completing the 45 sanctioned identifiers —
  `acre`, `bit`, `degree`, `fluid-ounce`, `gallon`, `gigabit`, `hectare`,
  `kilobit`, `megabit`, `microsecond`, `mile-scandinavian`, `millisecond`,
  `nanosecond`, `percent`, `petabyte`, `stone`, `terabit`, `terabyte`, `yard` —
  alongside the two `speed-…` compounds CLDR ships pre-composed. Any
  `<unit>-per-<unit>` ratio now formats via `format_unit_id` /
  `format_compound_unit`, and through `NumberFormat`'s `unit` option:
  `format_unit_id("en", 5.0, "meter-per-second", UnitWidth::Long)` →
  `"5 meters per second"`, `("gallon-per-mile", Short)` → `"5 gal/mi"`,
  `("gallon-per-mile", Long)` in German → `"5 Gallonen pro Meile"`. Both UTS #35
  assembly paths are implemented: the denominator's `perUnitPattern` when it has
  one (18 of the 45 units do) and the locale's `per` `compoundUnitPattern`
  otherwise. `Unit::from_ecma_id` / `Unit::ecma_id` expose the identifier
  mapping. Reported in [#17](https://github.com/KarpelesLab/intlrs/issues/17).
- *(unit)* `UnitWidth::Narrow` — ECMA-402 `unitDisplay: "narrow"` is now a real
  third width instead of an alias for short: `format_unit("en", 5.0,
  Unit::Kilometer, UnitWidth::Narrow)` → `"5km"` (short gives `"5 km"`), `"5°"`
  for degrees, `"3h"` for hours. `number::UnitDisplay::Narrow` resolves to it.
  The narrow patterns are a third of the unit table, so they sit behind the new
  `units-narrow` cargo feature (in `default`); without it the width falls back to
  short, which is UTS #35's own narrow → short fallback.
- *(number)* `en-IN` and `zh-Hant` number data, vendored from CLDR 48. `en-IN`
  carries Indian digit grouping (`format_decimal("en-IN", 12345678.0)` →
  `"1,23,45,678"`, was `"12,345,678"`) and crore/lakh compact forms; `zh-Hant`
  carries the Traditional Chinese compact forms (`format_compact("zh-Hant",
  123456789.0)` → `"1.2億"`, was `"1.2亿"`). Upstream ships no region files under
  `zh-Hant`, so codegen derives `zh-TW`, `zh-HK` and `zh-MO` from it through
  CLDR's own likelySubtags — the runtime lookup truncates a tag at each `-` and
  does no script inference, so those would otherwise reach Simplified `zh`.
  `zh-CN`/`zh-SG` still resolve to `zh`, as they should. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).
- *(datetime)* `DateTimeFormatError::UnsupportedOptions`, returned when the
  requested options resolve to a pattern with no fields left in it. Previously
  that case produced `Ok("")`, which a caller cannot tell apart from a real
  result.
- *(datetime)* localized time-zone names (UTS #35 §4.8). `time_zone_name` now
  resolves real names instead of always rendering a GMT offset:
  `America/Los_Angeles` in July is `"Pacific Daylight Time"` / `"PDT"` /
  `"Pacific Time"` / `"PT"` for `long` / `short` / `longGeneric` /
  `shortGeneric` in `en`, and `"heure d’été du Pacifique nord-américain"` /
  `"heure du Pacifique nord-américain"` in `fr`; `UTC` is `"Coordinated
  Universal Time"`. All 101 CLDR locales, 447 zones and 191 metazones, with the
  metazone ranges applied historically (`Europe/London` is `British` before
  1971-10-31 and `GMT` after) and tzdb links canonicalized (`US/Pacific`,
  `Asia/Calcutta`). The full fallback chain is implemented — the zone's own
  name, then its metazone's, then the *generic location format* (the locale's
  `regionFormat` over the country name for a single-zone country, else the
  exemplar city: `"heure : Los Angeles"`), then the localized GMT offset. Values
  were checked against V8/ICU. Chooses the standard or daylight name from the
  zone's DST state at the instant, which needs `iana-tz`; without it the zone is
  reported on standard time. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).
- *(datetime)* one cargo feature per tzdb area — `tz-names-africa`,
  `tz-names-america`, `tz-names-antarctica`, `tz-names-arctic`, `tz-names-asia`,
  `tz-names-atlantic`, `tz-names-australia`, `tz-names-etc`, `tz-names-europe`,
  `tz-names-indian`, `tz-names-pacific` — plus the `tz-names` umbrella, which is
  in `default`. The names are large (~3.6 MB of generated source, +4.4 MB
  compiled for all eleven areas; America +1.29 MB, Asia +1.28 MB, Europe
  +397 KB, Etc +22 KB), so a build can carry only the areas it uses; an area
  that is not compiled in falls back to the localized GMT offset, which is UTS
  #35's own last resort. The umbrella is in `default` on the same reasoning as
  `units-narrow`: answering a spec-mandated `timeZoneName: "long"` with an
  offset is a silently degraded result, which is worse than the size.
- *(number)* `format_range` and `format_range_to_parts` — ECMA-402 v3
  `Intl.NumberFormat.prototype.formatRange`/`formatRangeToParts`, driven by the
  CLDR `miscPatterns` the tables previously did not carry.
  `format_range("en", 2.9, 3.1, …)` with `style: currency` gives
  `"$2.90–$3.10"`; `zh` separates with an ASCII hyphen, `ja` with U+FF1E. Per
  `PartitionNumberRangePattern` step 5, ends that format *identically* collapse
  to the `approximately` form rather than a degenerate range: `"~3"` in `en`,
  `"≈3"` in `de` and `fr`, `"約 3"` in `ja` — and that is the `miscPatterns`
  `approximately` string, which is independent of the `approximatelySign` symbol
  (`fr` has `≈{0}` against `≃`). `format_range_to_parts` returns
  `NumberRangePart`s carrying the spec's `source` field
  (`NumberRangeSource::{StartRange, EndRange, Shared}`) and tags the collapse
  marker `NumberPartType::ApproximatelySign`. Behind the new `number-range`
  cargo feature (in `default`; ~16 KB of code + data). Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).
- *(number)* `-u-nu-<system>` support and `native_numbering_system` /
  `default_numbering_system`. Every `number` entry point now reads the tag's
  UTS #35 `nu` keyword — `format_decimal("hi-u-nu-deva", …)`,
  `format_decimal("ar-u-nu-native", …)` — and `NumberFormatOptions::
  numbering_system` accepts the same values, including the `"native"` alias,
  outranking the tag as ECMA-402 `ResolveLocale` requires. CLDR's
  `otherNumberingSystems.native` is now stored, which is what makes `"native"`
  resolvable: it is *not* the same as `defaultNumberingSystem` (`ar` defaults to
  `latn` but is natively `arab`, `hi` to `latn` but natively `deva`, `zh` to
  `latn` but natively `hanidec`). `number` does not depend on the `locale`
  feature, so the `-u-` scan is a small self-contained one in `number` rather
  than a new `Locale` accessor. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).
- *(number)* `NumberSpec::nan` and `NumberSpec::infinity`, the CLDR
  `symbols/nan` and `symbols/infinity` strings.

### Changed

- *(unit)* the CLDR unit table moved from the `src/cldr/units.bin` blob to
  generated Rust (`src/cldr/generated/units.rs`, `const fn` + `match`), matching
  how the Unicode property tables are shipped. `#[cfg]` sits on individual width
  dispatch arms, so `units` without `units-narrow` never compiles the narrow
  patterns. The table also gained the `perUnitPattern` and `compoundUnitPattern`
  strings and 19 more units, so the compiled footprint is ~905 KB for long+short
  and ~315 KB more for narrow, against 175 KB for the old (long+short only,
  28-unit) blob.
- *(number)* `format_to_parts` with `style: "unit"` now tags the whole unit
  phrase as one `unit` part, as ICU does, instead of splitting it on interior
  whitespace. `"5 kilometers per hour"` yields `integer("5") literal(" ")
  unit("kilometers per hour")`, not six parts. Single-word units are unchanged.
- *(datetime)* the localized GMT offset formats moved off the hand-curated
  26-locale `data/cldr/48/timezone.json` (and its `src/cldr/timezone.bin` blob,
  both now deleted) onto CLDR's own `timeZoneNames` for all 101 locales, in
  `src/cldr/generated/tz_names.rs`. All 26 curated locales matched CLDR 48
  byte-for-byte, so no locale changed; the other 75 stop falling back to
  English. The same table carries `regionFormat` and `fallbackFormat`, which the
  new name resolution needs.
- *(datetime)* `TimeZoneNameStyle::Short` no longer returns the tz database's
  abbreviation. It resolves CLDR's short metazone name, so `America/New_York`
  still gives `"EST"`/`"EDT"`, but `Asia/Tokyo` — for which CLDR has no short
  name — gives `"GMT+9"` rather than `"JST"`, and a non-English locale gets its
  own name or offset rather than the English abbreviation. This is what
  ECMA-402 specifies and what V8/ICU produce.

### Fixed

- *(datetime)* `TimeZoneNameStyle::ShortOffset` panicked for every locale whose
  `hourFormat` uses U+2212 MINUS SIGN (`et`, `fa`, `fr`, `ht`, `lt`, `sv`).
  Trimming the hour's
  leading zero split the string one byte past the sign, inside the multi-byte
  character. The short offset is now built from the `hourFormat` subpattern
  itself, which also fixes its shape: `GMT-7`, not `GMT-7:00` — UTS #35's short
  localized GMT drops the minute field when it is zero, as ICU does.
- *(datetime)* `format_gmt_offset` rendered the letter `H` literally for the two
  locales whose `hourFormat` uses a single `H` rather than `HH`:
  `format_gmt_offset("cs", 330)` gave `"GMT+H:30"` and now gives `"GMT+5:30"`
  (likewise `fi`). Only `HH` was ever substituted.
- *(number)* the CLDR number table moved from the `src/cldr/numbers.bin` and
  `src/cldr/numsys_default.bin` blobs to generated Rust
  (`src/cldr/generated/numbers.rs`, `match` lookups), following `units`. It is
  now keyed by *(locale, numbering system)* rather than by locale alone, which is
  what a blob could not gate: the non-`latn` blocks sit on `#[cfg]`-ed match arms
  behind the new `number-numsys` feature (in `default`; ~4 KB). Every record the
  blobs held was checked field-for-field against the generated table before the
  blobs were dropped — all 121 `numbers.bin` keys (103 locales plus the derived
  `lang-REGION` aliases) and all 103 `numsys_default.bin` keys are byte-identical.
  The base `number` footprint grows ~10 KB, the price of `&'static str` pointer
  pairs over a packed blob.
- *(number)* the `numbering_system` option, and a `-u-nu-` tag, now select the
  numbering system's **separators and patterns** too, not just its digits. CLDR
  keys `symbols-numberSystem-<ns>` and `decimalFormats-numberSystem-<ns>` per
  system and they genuinely differ: `format("ar", …, numbering_system: "arab")`
  gives `"١٬٢٣٤٫٥"` (U+066C group, U+066B decimal), not `"١,٢٣٤.٥"`; `te` groups
  Indian-style in `latn` but not in `telu`. Requests for a system the locale
  ships no block for keep its `latn` symbols, which is ICU's `NumberElements`
  fallback — `format("en", …, numbering_system: "arab")` is unchanged.
- *(number)* non-finite values use the locale's CLDR strings.
  `format("ar", f64::NAN, …)` is `"ليس رقمًا"` and `format("fa", …)` `"ناعدد"`,
  where all four call sites previously hardcoded `"NaN"`/`"∞"`. (`∞` happens to
  be right for every vendored locale, but was not being *read*.)

### Deprecated

- *(number)* `format_decimal_native` is renamed to
  `format_decimal_default_numbering`; the old name is kept as a deprecated
  forwarder. It never read `otherNumberingSystems.native` — it reads
  `defaultNumberingSystem`, and that is the ECMA-402 behaviour worth having
  (`Intl.NumberFormat('ar')` also formats `1.5` with Latin digits, because `ar`
  defaults to `latn` in CLDR 48). The native system is now reachable through
  `format_decimal("<lang>-u-nu-native", …)` or `native_numbering_system`.

### Fixed

- *(number)* `format_decimal_default_numbering` (ex `format_decimal_native`) uses
  the default numbering system's *separators*, not just its digits. Persian gave
  `"۱,۲۳۴.۵"` — `arabext` digits glued to `latn` separators, a combination no
  locale uses; `Intl.NumberFormat('fa')` gives `"۱٬۲۳۴٫۵"`, which is now the
  output.

- *(datetime)* a lone time field no longer formats as the empty string.
  `day_period`, `minute`, `second` and `fractional_second_digits`, each on their
  own, resolved through CLDR `availableFormats` — which tabulates only the
  combinations it expects to be asked for, and has no `m`, `s`, `B` or `S` entry
  in any locale. The lookup fell through to the locale's medium *date* pattern,
  whose fields the keep-pass then stripped in full, leaving `""`. Pure-time
  skeletons with no tabulated entry are now synthesized from their own fields
  (joined with the locale's time separator), the way ICU's
  `DateTimePatternGenerator` does. `minute` alone now gives `"30"`, `day_period`
  alone `"in the morning"` (`morgens`, `du matin`, `朝`), and
  `fractional_second_digits: 3` alone `"123"`. Date skeletons keep falling back
  to the locale's medium date pattern, whose field *order* a naive synthesis
  would get wrong. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).

- *(datetime)* `y` renders the era-relative year, not the astronomical one. UTS
  #35 counts the BCE side back from 1, so year `0` now formats as `1 BC` and
  `-1` as `2 BC`; previously they rendered `0 BC` and `-1 BC`, and a negative
  number beside a `BC` era is never right. This applies whether or not an `era`
  is requested, since that is what the `y` field means. The astronomical year is
  reachable through the `u` field, which is now rendered rather than dropped.
  Non-Gregorian calendars get the same treatment (the Islamic/Persian BH/BP
  side); the Japanese year-within-era and the Chinese cyclic year were already
  era-relative and are unchanged. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).

- *(number)* `format_scientific` no longer hangs on an infinite input. The
  mantissa normalization looped on `m /= 10.0` until `m < 10.0`, but `inf / 10.0`
  is `inf`, so the loop never terminated: a release build spun forever and a
  debug build panicked with "attempt to add with overflow" on the exponent
  counter. Non-finite input now returns early. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).
- *(number)* non-finite values format as ECMA-402's `∞` / `NaN` on every path,
  not Rust's `inf`. `format_decimal`, `format_percent`, `format_compact`,
  `format_currency` and `unit::format_unit` all reach `format_with`, which spelled
  the value with `{:.*}` and produced `"inf"`, `"inf%"`, `"$inf"`,
  `"inf meters"` — disagreeing with `format`/`format_to_parts`, which already
  emitted `"∞"`. The pattern's affixes are kept (`"∞%"`, `"$∞"`), the locale minus
  sign is used for negative infinity, and `NaN` is unsigned per ECMA-402.

- *(datetime)* `format_range` composes a date+time interval instead of repeating
  the date. CLDR keys `intervalFormats` by date-only or time-only skeletons and
  never by a mixed one, so a combined request (`{year, month, day, hour,
  minute}`) always missed the lookup and fell back to formatting the whole
  pattern twice — `"6/15/2024, 9:00 AM – 6/15/2024, 5:00 PM"` for two times on
  one day. When the greatest differing field is a time field, UTS #35 §2.6.2
  composition now applies: the date is formatted once and the time *range* is
  glued into it with the locale's `dateTimeFormat`, giving `"6/15/2024, 9:00 AM
  – 5:00 PM"` (`"2024/6/15 9時00分～17時00分"` in `ja`). A date-field difference
  still carries the whole pattern on both ends, as ICU does. Reported in
  [#17](https://github.com/KarpelesLab/intlrs/issues/17).

- *(datetime)* `format_range` no longer drops the end of a seconds-only
  interval. The greatest-difference scan stopped at the minute, so
  `{hour, minute, second}` over `9:00:10`–`9:00:45` reported no difference at all
  and formatted a single time, `"9:00:10 AM"`, silently losing the end point. It
  now gives `"9:00:10 AM – 9:00:45 AM"`. Sub-second differences count too when
  `fractional_second_digits` puts them on screen (ECMA-402 groups `s`/`S`/`A` as
  one range field); a millisecond difference the pattern does not show still
  collapses to a single value.

- *(datetime)* `format_range_to_parts` attributes the literals inside a fallback
  half to that half. When the two ends are formatted separately and joined by
  `intervalFormatFallback`, every literal was tagged `Shared`, including the
  `", "` and `":"` *within* each date. ECMA-402 substitutes each side into the
  `{0}`/`{1}` slot wholesale, so those are `StartRange`/`EndRange` like the
  fields they punctuate; only the fallback's own separator is `Shared`.
  Reported in [#17](https://github.com/KarpelesLab/intlrs/issues/17).

## [0.5.2](https://github.com/KarpelesLab/intlrs/compare/v0.5.1...v0.5.2) - 2026-07-27

### Added

- *(collation)* re-vendor locale tailorings from CLDR 48

### Changed

- *(collation)* re-vendor locale tailorings from CLDR 48. The rule table is now
  generated by `codegen` from `data/cldr/48/collation/*.xml` instead of a
  pre-filtered JSON blob, so `Tailoring::for_locale` carries CLDR's own
  `standard` rule for 78 locales (was 41). The first vendoring had filtered out
  every rule using `[before]`/`[import]`/expansions; the parser gained that
  syntax later, but the data was never regenerated, stranding Swedish, Finnish,
  Norwegian, Icelandic, Estonian, Turkish, Azerbaijani, Kazakh, Albanian,
  Ukrainian and Greenlandic on lossy hand-written approximations. Swedish now
  folds `ü` under `y` and `æ`/`ø` under `ä`/`ö` and orders `þ`/`ð`/`đ`, none of
  which the old `&z < å < ä < ö` expressed.

### Fixed

- *(collation)* `Tailoring::for_locale("de")` no longer returns the CLDR
  *phonebook* tailoring (`ä = ae`, `ß = ss`). German has no `standard` collation
  in CLDR — phonebook is `de-u-co-phonebk` — so plain `de` sorts in root order,
  matching `Intl.Collator("de")`.
- *(collation)* `Tailoring::for_locale("ga")` no longer invents a primary
  distinction for the long-vowel accents. CLDR states the root order is valid for
  Irish, where `á` is a secondary variant of `a`.
- *(collation)* drop the Balochi (`bal`) tailoring, which inverted its own rule:
  it sorted `ا` *before* `آ` where the rule (and root order) put it after. The
  consistency gate had not caught it because its tokenizer skipped
  space-separated rules entirely.
- *(collation)* the `tests/collation_data_consistency` gate now lexes the same
  grammar the runtime parser does (`[before]` resets, bracket options, `<*` star
  ranges, `'…'` quoting, `\uXXXX` escapes, `/` expansions) rather than scanning
  for relation characters, so it can validate the full unfiltered CLDR rule set
  instead of silently skipping the parts it could not tokenize.

### Added

- *(collation)* `nb`/`nn` inherit Norwegian (`no`) and `tl` inherits Filipino
  (`fil`) collation, as CLDR's locale inheritance specifies.

## [0.5.1](https://github.com/KarpelesLab/intlrs/compare/v0.5.0...v0.5.1) - 2026-07-19

### Added

- *(collation)* zh unihan variant and ko hanja-by-reading
- *(collation)* Unihan radical-stroke fallback, hu gemination, zh stroke/zhuyin + ko variants
- *(datetime)* dayPeriod minute-precision, islamic/persian era widths, Japanese pre-Meiji nengo
- *(locale)* structural validation of extension subtag lengths in canonicalize
- *(collation)* script [reorder] engine (Cyrillic-first for ru/bg/sr)
- *(segment)* NFKC pre-normalization on the CJK segmentation path
- *(locale)* canonicalize -u-/-t- extension keywords in getCanonicalLocales
- *(collation)* zh pinyin collation via generated Han weight table
- *(segment)* Khmer and Burmese dictionary word segmentation
- *(collation)* parser [before]/[import] support; add ja/ko and wide locale coverage
- *(segment)* CJK (and SEA) dictionary word segmentation
- *(segment)* dictionary-based word segmentation for Thai (and related scripts)
- *(collation)* expand per-locale tailoring coverage
- *(datetime)* localized Japanese-era date formatting
- *(calendar)* widen Chinese table and add Korean dangi calendar
- *(datetime)* Chinese calendar formatting with cyclic year names and related year
- *(datetime)* add formatRange/formatRangeToParts interval formatting
- *(calendar)* match ICU for Persian calendar (astronomical leap years)
- *(locale)* resolve CLDR aliases in canonicalize/getCanonicalLocales
- *(calendar)* add Umm al-Qura (islamic-umalqura) calendar

### Fixed

- *(ci)* format codegen crate and drop private intra-doc link
- *(clippy)* replace match with ? in rbnf_lookup (stable clippy -D warnings)

### Other

- update public-api snapshot for format_japanese_date
- update public-api snapshot for dangi calendar
- update public-api snapshot for format_chinese_date
- update public-api snapshot for interval formatting
- update public-api snapshot for umalqura + locale canonicalize

## [0.5.0](https://github.com/KarpelesLab/intlrs/compare/v0.4.1...v0.5.0) - 2026-06-19

### Added

- per-formatter Cargo features (gate module + CLDR data)

### Fixed

- *(ci)* collation needs the _cldr marker (reads CLDR tailorings)
- *(ci)* codegen clippy — avoid explicit loop counter in day_period_table
- *(ci)* collapse nested ifs into let-chains (clippy at MSRV 1.88)

### Other

- make DateTimeFormatOptions and NumberFormatOptions non_exhaustive
- raw-source Islamic/Persian calendars to ~101 locales
- raw-source DisplayNames/units/lists/relative/numsys to ~101 locales
- remove obsolete curated currency.json (raw-sourced now)
- raw-source currency for ~101 locales + currencyDisplay Code/Name
- complete raw-source numbers/compact (codegen + blobs + reader)
- raw-source numbers/compact for ~101 locales + compactDisplay Long
- broaden date coverage to CLDR-modern base locales (~101)
- named timeZoneName via the timezone-data crate
- flexible day periods (B field + dayPeriod option)
- implement style: Unit (ECMA-402)
- raise minimum supported Rust to 1.88
- migrate the crate to Rust edition 2024

## [0.4.1](https://github.com/KarpelesLab/intlrs/compare/v0.4.0...v0.4.1) - 2026-06-11

### Other

- implement h11/h12/h23/h24 hour cycles (0-vs-1 origin)
- derive default hour cycle from locale data; keep weekday in date combos

## [0.4.0](https://github.com/KarpelesLab/intlrs/compare/v0.3.1...v0.4.0) - 2026-06-11

### Other

- rustfmt the ECMA-402 number/datetime code + codegen emit_dates
- ECMA-402 DateTimeFormat options + formatToParts
- ECMA-402 NumberFormat options + formatToParts
- add sub-second DateTime.millisecond field
- raw-source Gregorian date data from official CLDR v48
- linear-time connector folding (quadratic)
- truncate normalization buffer on char boundary (wrong-fallback)
- validate POSIX TZ rule field ranges (reject malformed M-rules)
- document that to_unicode output is unvalidated (not for security decisions)
- bounds-check names.bin parser (panic on malformed blob)
- bound push_pend against MAX_DECOMP (latent OOB)
- wrapping_add for regional-indicator parity counters (debug overflow)
- guard forward conversions against i64 overflow (panic on extreme inputs)
- bound find/contains unaligned-start scan (quadratic DoS)

## [0.3.1](https://github.com/KarpelesLab/intlrs/compare/v0.3.0...v0.3.1) - 2026-06-05

### Other

- use unsigned_abs in format_gmt_offset (i32::MIN panic)
- guard empty separator in parse_decimal (latent hang) and saturating compact width
- cap U-label code points before Punycode encode (encoder DoS)
- memoize SB8 sentence lookahead (quadratic DoS)
- bound window_decision scan (quadratic find/contains DoS)

## [0.3.0](https://github.com/KarpelesLab/intlrs/compare/v0.2.3...v0.3.0) - 2026-06-05

### Other

- mark Error #[non_exhaustive]
- declare bidi feature dependency (fixes component-subsets CI)
- enforce VerifyDnsLength (reject empty/trailing-root labels)
- apply UTS #39 augmented (CJK) script sets in is_single_script
- satisfy CI gates for security fixes (fmt, clippy, public-api snapshot)
- resolve is_single_script via Script_Extensions intersection (UTS #39)
- IDNA tests: adapt to fallible to_unicode; raise conformance rejection bar to 480
- enforce full IDNA2008 validity (CheckBidi/ContextJ/Hyphens/V1/V5/V6) and re-canonicalize xn--
- do not drop combining marks past MAX_COMBINING in NFC recomposition
- O(1) matched-PDI lookup (latent quadratic)
- linear find/contains (quadratic-scan DoS)
- avoid O(n) Vec::remove in discontiguous matching
- strip Default_Ignorable code points in UTS #39 skeleton (confusable-detection miss)
- cap MessageFormat parse recursion depth (stack-overflow DoS)
- use checked arithmetic in POSIX TZ offset parsing (i32 overflow panic/wrap)
- cap punycode decode output (decompression-bomb DoS) and enforce 253-octet domain limit

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
