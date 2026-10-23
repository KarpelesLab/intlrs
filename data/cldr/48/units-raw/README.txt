Raw Unicode CLDR v48 measurement-unit data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-units-full/main/<locale>/units.json

Committed unmodified. `codegen` (emit_units) reads them to produce the generated
Rust table at src/cldr/generated/units.rs. Do not hand-edit; to refresh,
re-download from the same upstream path at the pinned CLDR version.

The base set is the CLDR "modern" coverage base languages (language subtag only),
matching the sibling `*-raw` directories. Three region bundles are vendored on top
because their unit wording differs from the base language and no fallback can
derive it:

  zh-Hant     "每小時 {0} 公里", short "{0} 公里/小時"
  zh-Hant-HK  short "{0} 公里每小時", narrow "{0}kph"
  fr-CA       short concentr-percent "{0}\u00a0%" — a no-break space where `fr`
              has a plain one. That one character is load-bearing: ECMA-402
              formats `style: "percent"` through the `concentr-percent` unit
              pattern under compact notation, so without this bundle every
              `fr-CA` compact percentage carries the wrong space.

`zh-TW`/`zh-HK`/`zh-MO` reach these through the runtime's CLDR locale chain
(`zh-HK` -> `zh-Hant-HK`, `zh-MO` -> `zh-Hant-HK`, `zh-TW` -> `zh-Hant`). The
CLDR hub and script sublocales are vendored too, and a sublocale stores only the
patterns that differ from its ancestor; see ../README-sublocales.txt.
