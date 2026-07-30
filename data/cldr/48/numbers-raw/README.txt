Raw Unicode CLDR v48 number data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-numbers-full/main/<locale>/numbers.json
        https://github.com/unicode-org/cldr (tag release-48)
        common/main/root.xml                              [root.xml only]

These are the official Unicode CLDR files, committed unmodified. `codegen`
(emit_numbers, emit_numsys, emit_currency) reads them to produce
src/cldr/numbers.bin, src/cldr/compact.bin and src/cldr/numsys_default.bin.
Do not hand-edit; to refresh, re-download from the same upstream paths at the
pinned CLDR version.

The base set is the CLDR "modern" coverage base languages (language subtag only),
matching the other `*-raw` directories. Two non-base locales are vendored on top
because their number formatting differs from their base language and no fallback
can derive it:

  en-IN    Indian digit grouping (`#,##,##0.###`) and crore/lakh compact forms.
  zh-Hant  Traditional Chinese compact forms (億 rather than 亿).

`zh-Hant` has no region files upstream; `emit_numbers` derives `zh-TW`, `zh-HK`
and `zh-MO` from it using CLDR's own likelySubtags, since the runtime lookup does
no script inference.

A further 22 `lang-REGION` files are vendored for one field each: they are the
only region locales in all of CLDR 48 whose `defaultNumberingSystem` differs from
their base language's, and no fallback can derive that either (UTS #35 §3.4 —
`ar-EG` is `arab` where plain `ar` is `latn`).

  ar-BH ar-DJ ar-EG ar-ER ar-IL ar-IQ ar-JO ar-KM ar-KW ar-LB ar-MR
  ar-OM ar-PS ar-QA ar-SA ar-SD ar-SO ar-SS ar-SY ar-TD ar-YE        arab
  ur-IN                                                              arabext

Everything else in those files is identical to the base language's, so
`emit_numbers` deduplicates the records and emits only the differing numbering
system pair. Region locales whose *symbols* differ (`ar-MA`, `ar-DZ`, `ar-TN`, …)
are deliberately NOT vendored: they need a full record, not one field, and the
base set is the coverage line this crate draws.

root.xml is the LDML source rather than cldr-json because cldr-json's `und`
(`cldr-numbers-full/main/und/numbers.json`) carries only the `latn` block. Root's
`arab` and `arabext` symbols are what a locale with no block of its own inherits
— `Intl.NumberFormat('en-u-nu-arab')` groups with U+066C, not with `en`'s comma —
and they exist only in the XML. Every other root numbering system aliases to
`latn` with `source="locale"`, i.e. to the *requesting* locale's `latn` block,
which is why `emit_numbers` emits root arms for those two systems only.
