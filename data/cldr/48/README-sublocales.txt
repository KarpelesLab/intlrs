parentLocales.json and the vendored sublocales — Unicode CLDR v48.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-core/supplemental/parentLocales.json      [verbatim]

Locale inheritance
------------------
The runtime walks a locale's CLDR inheritance chain (UTS #35 §4.1.3), not its
subtags: `crate::cldr::fallback`, driven by src/cldr/generated/parents.rs, which
`codegen` (Chain::emit) derives from parentLocales.json and likely.json.

  * a `lang-REGION` tag first gets the script likelySubtags gives it when that is
    not the language's default: `zh-TW` is walked as `zh-Hant-TW`, `sr-ME` as
    `sr-Latn-ME`;
  * an explicit `parentLocale` entry outranks truncation: `en-GB` -> `en-001`
    -> `en`, `pt-AO` -> `pt-PT` -> `pt`, `es-MX` -> `es-419` -> `es`,
    `zh-Hant-MO` -> `zh-Hant-HK` -> `zh-Hant`;
  * a `lang-Script` whose script is not the language's likely one has root as
    its parent (`_localeRules: nonlikelyScript`): `zh-Hant`, `sr-Latn`;
  * when the walk reaches root without trying the bare language, the bare
    language is tried last, so a section that vendors no `zh-Hant` still
    answers in Chinese rather than English.

Codegen applies the same chain (`Chain` in codegen/src/main.rs) when it decides
what a sublocale record is compared against, so data and runtime agree.

Which sublocales are vendored
-----------------------------
The base set of every section is the CLDR "modern" coverage base languages. On
top of that, each section vendors those of the following sublocales whose
cldr-json file differs from what the chain would otherwise reach:

  the parentLocales hubs   en-001 en-150 en-IN es-419 fr-HT pt-PT zh-Hant-HK
  the non-default scripts  zh-Hant sr-Latn yue-Hans hi-Latn kk-Arab
  plus                     en-GB

Hubs are what most regional locales inherit from, so `en-AU`, `en-IE`, `es-MX`,
`pt-AO`, … now get their hub's data instead of their base language's.

currencies-raw/ and localenames-raw/ additionally vendor every CLDR
modern-coverage sublocale that differs, because those two are stored as deltas
(below) and cost ~35 KB each in total: `en-AU` names AUD "$", `es-AR` USD "US$".

numbers-raw/ keeps its region files as described in its README, minus those the
chain now reaches through a hub (`pt-CV` is `pt-PT`'s, `es-PR` is `es-419`'s).

How sublocales are stored
-------------------------
Tables the runtime reads one key at a time store a sublocale as a delta against
its nearest vendored ancestor, and the lookup walks the chain for the rest:

  currency.bin, display_*.bin   only the codes whose names/symbols differ
  generated/tz_names.rs         only the differing names (`tz_names::parent`)
  generated/units.rs            only the differing patterns (`units::parent`)

Whole-record tables (calendar.bin, skeletons.bin, intervals.bin, cal_*.bin,
compact.bin) drop a sublocale record that is byte-identical to its ancestor's.

Known gaps
----------
Leaf regions other than the ones above use their hub's dates (`en-AU` short
date is `en-001`'s `09/08/2023` where CLDR has `9/8/23`).
