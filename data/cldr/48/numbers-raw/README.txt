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

On top of that, every `lang-REGION` file of a base language whose `numbers`
block differs from the base language's in anything the codegen consumes is
vendored — 116 files in CLDR 48. Region locales that are byte-for-byte the base
language's in those fields (179 of them: `en-US`, `de-DE`, `fr-FR`, `ja-JP`, …)
are not, since the runtime's truncating fallback (`fr-FR` -> `fr`) reaches the
same data. The fields compared are:

  symbols-numberSystem-*         separators, signs, percent, NaN/infinity
  decimalFormats-numberSystem-*  standard + compact (short/long) patterns
  percentFormats-numberSystem-*  standard pattern
  currencyFormats-numberSystem-* standard pattern (+ unitPattern)
  miscPatterns-numberSystem-*    approximately / range
  minimumGroupingDigits
  defaultNumberingSystem, otherNumberingSystems

Without these, `pt-PT` formatted like `pt` (`987.654.321,5`, `€ 3,00`, `3–5`)
where CLDR gives it `987 654 321,5`, `3,00 €` and `3 - 5`; `de-CH`/`it-CH`
lost their `'` grouping, `es-MX` its `.` decimal point, `en-ZA` its `,` one.
22 of the 116 change nothing but the numbering system pair (UTS #35 §3.4 —
`ar-EG` is `arab` where plain `ar` is `latn`):

  ar-BH ar-DJ ar-EG ar-ER ar-IL ar-IQ ar-JO ar-KM ar-KW ar-LB ar-MR
  ar-OM ar-PS ar-QA ar-SA ar-SD ar-SO ar-SS ar-SY ar-TD ar-YE        arab
  ur-IN                                                              arabext

`emit_numbers` deduplicates: locales whose symbols, patterns and `miscPatterns`
are identical share one table index, and a differing numbering system pair is
emitted as a per-locale override. `emit_currency` reads the region files too,
for their currency *pattern* (`pt-PT` puts the symbol after the amount);
currency *names* stay per base language (`currencies-raw` is base-only), so a
region locale's symbol for a given currency is its base language's — CLDR's
`es-MX` spells `EUR` as "EUR" where `es` uses "€", and that difference is
not carried.

`lang-Script-REGION` files (`zh-Hant-HK`, `sr-Latn-BA`, …) are not vendored;
`zh-HK`/`zh-MO`/`zh-TW` derive from `zh-Hant` as described above.

root.xml is the LDML source rather than cldr-json because cldr-json's `und`
(`cldr-numbers-full/main/und/numbers.json`) carries only the `latn` block. Root's
`arab` and `arabext` symbols are what a locale with no block of its own inherits
— `Intl.NumberFormat('en-u-nu-arab')` groups with U+066C, not with `en`'s comma —
and they exist only in the XML. Every other root numbering system aliases to
`latn` with `source="locale"`, i.e. to the *requesting* locale's `latn` block,
which is why `emit_numbers` emits root arms for those two systems only.
