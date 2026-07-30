Raw Unicode CLDR v48 measurement-unit data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-units-full/main/<locale>/units.json

Committed unmodified. `codegen` (emit_units) reads them to produce the generated
Rust table at src/cldr/generated/units.rs. Do not hand-edit; to refresh,
re-download from the same upstream path at the pinned CLDR version.

The base set is the CLDR "modern" coverage base languages (language subtag only),
matching the sibling `*-raw` directories. Three Traditional Chinese bundles are
vendored on top because their unit wording differs from Simplified `zh` and no
fallback can derive it:

  zh-Hant     "每小時 {0} 公里", short "{0} 公里/小時"
  zh-Hant-HK  short "{0} 公里每小時", narrow "{0}kph"
  zh-Hant-MO  as HK

`emit_units` derives `zh-TW`/`zh-HK`/`zh-MO` from these via CLDR's likelySubtags,
preferring the most specific vendored bundle — `zh-HK` resolves to `zh-Hant-HK`,
not `zh-Hant`, because Hong Kong's short form genuinely differs from Taiwan's.
