# CLDR BCP-47 keyword data (vendored)

Source: Unicode CLDR, release-48, `common/bcp47/*.xml`
Upstream: https://github.com/unicode-org/cldr/tree/release-48/common/bcp47

Files vendored here define the Unicode (`-u-`) and Transform (`-t-`) extension
keys and their type values, including the `deprecated`/`preferred`/`alias`
attributes and canonical casings used to canonicalize extension keywords
(UTS #35 tr35 §3.6.5 / ECMA-402 CanonicalizeUnicodeLocaleId).

- `calendar.xml`  — `ca`, `fw`, `hc` keys (e.g. `islamicc` -> `islamic-civil`)
- `collation.xml` — `co`, `ka`, `ks`, `kn`, ... keys (e.g. `ks/primary` -> `level1`)
- `measure.xml`   — `ms`, `mu` keys (e.g. `ms/imperial` -> `uksystem`)
- `number.xml`    — `nu` numbering-system key
- `timezone.xml`  — `tz` key (deprecated -> preferred short zone ids)
- `transform.xml` — `-t-` `m0` transform-mechanism field (e.g. `m0/names` -> `prprname`)

Consumed by `codegen`'s `emit_bcp47`, which compiles the valid (BCP-47-legal,
short) type-value aliases into `src/cldr/bcp47.bin`. Boolean `yes`/`no`/`true`/
`false` type aliases are intentionally excluded: `true`/`yes` keyword type
values are dropped universally at canonicalization time, and `false`/`no` are
kept verbatim (matching V8 / ECMA-402).
