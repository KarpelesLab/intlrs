Raw Unicode CLDR v48 time-zone name data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-dates-full/main/<locale>/timeZoneNames.json

These are the official Unicode CLDR `cldr-dates-full` time-zone-name files,
committed unmodified, for the same CLDR "modern" coverage base languages as the
sibling `dates/` directory. `codegen` (emit_tz_names) reads them,
together with ../metaZones.json, to generate the localized zone-name tables.
Do not hand-edit; to refresh, re-download from the same upstream path at the
pinned CLDR version.

Each file carries the localized GMT/region/fallback patterns, per-zone exemplar
cities, and the long/short generic/standard/daylight names for each metazone
(e.g. `America_Pacific` -> "Pacific Standard Time" / "PST").

The CLDR hub and non-default-script sublocales (`en-001`, `pt-PT`, `zh-Hant`,
…) are vendored on top of the base languages; see ../README-sublocales.txt.
