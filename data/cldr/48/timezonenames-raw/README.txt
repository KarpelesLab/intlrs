Raw Unicode CLDR v48 time-zone name data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-dates-full/main/<locale>/timeZoneNames.json

These are the official Unicode CLDR `cldr-dates-full` time-zone-name files,
committed unmodified, for the same CLDR "modern" coverage base languages as the
sibling `dates/` directory (101 locales). `codegen` (emit_tz_names) reads them,
together with ../metaZones.json, to generate the localized zone-name tables.
Do not hand-edit; to refresh, re-download from the same upstream path at the
pinned CLDR version.

Each file carries the localized GMT/region/fallback patterns, per-zone exemplar
cities, and the long/short generic/standard/daylight names for each metazone
(e.g. `America_Pacific` -> "Pacific Standard Time" / "PST").
