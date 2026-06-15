//! Locale-aware date and time formatting (CLDR / UTS #35, Gregorian).
//! Requires the `alloc` feature.
//!
//! A [`DateTime`] holds the broken-down fields (no calendar arithmetic or time
//! zones); [`format_date`] / [`format_time`] / [`format_datetime`] render it with
//! the locale's CLDR patterns, month/weekday names, and am/pm markers. The
//! weekday is derived from the proleptic Gregorian date.
//!
//! ```
//! use intl::datetime::{DateTime, format_date, format_time, DateStyle};
//! let dt = DateTime { year: 2026, month: 6, day: 4, hour: 14, minute: 30, second: 5, millisecond: 0 };
//! assert_eq!(format_date("en", &dt, DateStyle::Long), "June 4, 2026");
//! assert_eq!(format_date("en", &dt, DateStyle::Medium), "Jun 4, 2026");
//! assert_eq!(format_time("en", &dt, DateStyle::Short), "2:30\u{202f}PM");
//! ```

use crate::cldr::{CalendarSpec, calendar_spec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A broken-down Gregorian date and time. Fields are not validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    /// Proleptic Gregorian year (e.g. 2026).
    pub year: i32,
    /// Month, 1–12.
    pub month: u8,
    /// Day of month, 1–31.
    pub day: u8,
    /// Hour, 0–23.
    pub hour: u8,
    /// Minute, 0–59.
    pub minute: u8,
    /// Second, 0–59.
    pub second: u8,
    /// Millisecond, 0–999 (sub-second precision; 0 when unused).
    pub millisecond: u16,
}

impl Default for DateTime {
    /// The Unix epoch, `1970-01-01T00:00:00.000`.
    fn default() -> Self {
        DateTime {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }
    }
}

impl DateTime {
    /// Render as a machine-readable ISO-8601 timestamp
    /// (`YYYY-MM-DDTHH:MM:SS`), independent of locale. A non-zero
    /// [`millisecond`](Self::millisecond) is appended as `.SSS`; it is omitted
    /// entirely when zero.
    #[must_use]
    pub fn to_iso8601(&self) -> String {
        let base = alloc::format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second
        );
        if self.millisecond == 0 {
            base
        } else {
            alloc::format!("{base}.{:03}", self.millisecond)
        }
    }

    /// The ISO-8601 weekday: 1 = Monday … 7 = Sunday.
    #[must_use]
    pub fn weekday(&self) -> u8 {
        crate::calendar::day_of_week(self.year as i64, self.month as i64, self.day as i64)
    }

    /// This date-time advanced by `delta` seconds (negative to go back), with
    /// day/month/year carry handled through the proleptic Gregorian calendar.
    ///
    /// ```
    /// use intl::datetime::DateTime;
    /// let dt = DateTime { year: 2026, month: 12, day: 31, hour: 23, minute: 59, second: 30, millisecond: 0 };
    /// let next = dt.add_seconds(90); // crosses into the new year
    /// assert_eq!(next, DateTime { year: 2027, month: 1, day: 1, hour: 0, minute: 1, second: 0, millisecond: 0 });
    /// ```
    #[must_use]
    pub fn add_seconds(&self, delta: i64) -> DateTime {
        let jdn =
            crate::calendar::gregorian_to_jdn(self.year as i64, self.month as i64, self.day as i64);
        // Saturating arithmetic: an extreme year combined with an extreme delta
        // would otherwise overflow i64 and panic in debug builds.
        let total = jdn
            .saturating_mul(86_400)
            .saturating_add(self.hour as i64 * 3600)
            .saturating_add(self.minute as i64 * 60)
            .saturating_add(self.second as i64)
            .saturating_add(delta);
        let (new_jdn, sod) = (total.div_euclid(86_400), total.rem_euclid(86_400));
        let (y, m, d) = crate::calendar::jdn_to_gregorian(new_jdn);
        DateTime {
            year: y as i32,
            month: m as u8,
            day: d as u8,
            hour: (sod / 3600) as u8,
            minute: (sod % 3600 / 60) as u8,
            second: (sod % 60) as u8,
            millisecond: self.millisecond,
        }
    }

    /// This date advanced by `delta` whole days (keeping the time of day).
    #[must_use]
    pub fn add_days(&self, delta: i64) -> DateTime {
        self.add_seconds(delta * 86_400)
    }

    /// Parse an ISO-8601 timestamp such as `"2026-06-04T14:30:05"` or
    /// `"2026-06-04T14:30:05.250"`. Accepts a space instead of `T`, an omitted
    /// time or seconds, optional fractional seconds (`.S`–`.SSS…`, truncated to
    /// millisecond precision), and a trailing `Z`. Returns `None` if malformed.
    /// (A time-zone offset, if present, is ignored.)
    #[must_use]
    pub fn parse_iso8601(s: &str) -> Option<DateTime> {
        let s = s.trim().trim_end_matches('Z');
        let (date, time) = match s.split_once(['T', ' ']) {
            Some((d, t)) => (d, Some(t)),
            None => (s, None),
        };
        let mut dp = date.split('-');
        let year: i32 = dp.next()?.parse().ok()?;
        let month: u8 = dp.next()?.parse().ok()?;
        let day: u8 = dp.next()?.parse().ok()?;
        if dp.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return None;
        }
        let (hour, minute, second, millisecond) = match time {
            None => (0, 0, 0, 0),
            Some(t) => {
                // Drop any zone offset on the time component.
                let t = t.split(['+', '-']).next().unwrap_or(t);
                let mut tp = t.split(':');
                let h: u8 = tp.next()?.parse().ok()?;
                let mi: u8 = tp.next()?.parse().ok()?;
                let (se, ms) = match tp.next() {
                    Some(x) => {
                        let (whole, frac) = match x.split_once(['.', ',']) {
                            Some((w, f)) => (w, Some(f)),
                            None => (x, None),
                        };
                        let se: u8 = whole.parse().ok()?;
                        let ms = match frac {
                            None | Some("") => 0u16,
                            Some(f) => {
                                if !f.bytes().all(|b| b.is_ascii_digit()) {
                                    return None;
                                }
                                // First up-to-3 fractional digits, scaled to ms.
                                let take = &f[..f.len().min(3)];
                                let v: u16 = take.parse().ok()?;
                                v * 10u16.pow(3 - take.len() as u32)
                            }
                        };
                        (se, ms)
                    }
                    None => (0, 0),
                };
                (h, mi, se, ms)
            }
        };
        Some(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
        })
    }
}

/// The kind of a [`DateTimePart`] produced by [`format_to_parts`], matching the
/// ECMA-402 `Intl.DateTimeFormat.prototype.formatToParts` part `type` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimePartType {
    /// Weekday name.
    Weekday,
    /// Era name.
    Era,
    /// Year.
    Year,
    /// Month (number or name).
    Month,
    /// Day of month.
    Day,
    /// Hour.
    Hour,
    /// Minute.
    Minute,
    /// Second.
    Second,
    /// Fractional second digits.
    FractionalSecond,
    /// Day period (AM/PM).
    DayPeriod,
    /// Time-zone name or offset.
    TimeZoneName,
    /// Literal text (separators, glue).
    Literal,
}

impl DateTimePartType {
    /// The ECMA-402 part `type` string for this kind.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            DateTimePartType::Weekday => "weekday",
            DateTimePartType::Era => "era",
            DateTimePartType::Year => "year",
            DateTimePartType::Month => "month",
            DateTimePartType::Day => "day",
            DateTimePartType::Hour => "hour",
            DateTimePartType::Minute => "minute",
            DateTimePartType::Second => "second",
            DateTimePartType::FractionalSecond => "fractionalSecond",
            DateTimePartType::DayPeriod => "dayPeriod",
            DateTimePartType::TimeZoneName => "timeZoneName",
            DateTimePartType::Literal => "literal",
        }
    }
}

/// One tagged segment of a formatted date-time (see [`format_to_parts`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTimePart {
    /// What this segment represents.
    pub kind: DateTimePartType,
    /// The literal text of this segment.
    pub value: String,
}

/// One of the four CLDR length styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateStyle {
    /// Full ("Thursday, June 4, 2026").
    Full,
    /// Long ("June 4, 2026").
    Long,
    /// Medium ("Jun 4, 2026").
    Medium,
    /// Short ("6/4/26").
    Short,
}

impl DateStyle {
    fn idx(self) -> usize {
        match self {
            DateStyle::Full => 0,
            DateStyle::Long => 1,
            DateStyle::Medium => 2,
            DateStyle::Short => 3,
        }
    }
}

/// Day of week for a proleptic Gregorian date: 0 = Sunday … 6 = Saturday
/// (Sakamoto's algorithm).
fn weekday(dt: &DateTime) -> usize {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    // Compute in i64: i32 year arithmetic here overflows for extreme years (and
    // `year - 1` underflows at i32::MIN) on unvalidated DateTime input.
    let m = (dt.month as i64).clamp(1, 12);
    let d = dt.day as i64;
    let y = if m < 3 {
        dt.year as i64 - 1
    } else {
        dt.year as i64
    };
    (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d).rem_euclid(7) as usize
}

fn two(n: i64) -> String {
    alloc::format!("{n:02}")
}

/// Render one date-field run (`field` repeated `n` times) of a CLDR pattern.
fn field(field: char, n: usize, dt: &DateTime, s: &CalendarSpec) -> String {
    let m = dt.month as usize;
    // Index for the 12-element month-name arrays, clamped so an out-of-range
    // (unvalidated) month never indexes out of bounds.
    let mi = m.clamp(1, 12) - 1;
    match field {
        'y' | 'Y' => {
            if n == 2 {
                two((dt.year.rem_euclid(100)) as i64)
            } else {
                dt.year.to_string()
            }
        }
        'M' | 'L' => match n {
            1 => m.to_string(),
            2 => two(m as i64),
            3 => s.months_abbr[mi].to_string(),
            5 => s.months_narrow[mi].to_string(),
            _ => s.months_wide[mi].to_string(),
        },
        'd' => {
            if n >= 2 {
                two(dt.day as i64)
            } else {
                dt.day.to_string()
            }
        }
        'E' | 'e' | 'c' => {
            let w = weekday(dt);
            if n == 5 {
                s.days_narrow[w].to_string()
            } else if n >= 4 {
                s.days_wide[w].to_string()
            } else {
                s.days_abbr[w].to_string()
            }
        }
        'G' => {
            // Gregorian era: index 0 = BCE (year ≤ 0), 1 = CE.
            let idx = usize::from(dt.year > 0);
            if n >= 5 {
                s.eras_narrow[idx]
            } else if n == 4 {
                s.eras_wide[idx]
            } else {
                s.eras_abbr[idx]
            }
            .to_string()
        }
        'h' => {
            // h12: 1–12 (12-hour clock). Widen first: `dt.hour + 11` would
            // overflow u8 for hour > 244.
            let h = ((dt.hour as u16 + 11) % 12) + 1;
            if n >= 2 { two(h as i64) } else { h.to_string() }
        }
        'K' => {
            // h11: 0–11 (12-hour clock, midnight/noon = 0).
            let h = (dt.hour as u16) % 12;
            if n >= 2 { two(h as i64) } else { h.to_string() }
        }
        'H' => {
            // h23: 0–23.
            if n >= 2 {
                two(dt.hour as i64)
            } else {
                dt.hour.to_string()
            }
        }
        'k' => {
            // h24: 1–24 (midnight = 24).
            let h = if dt.hour == 0 { 24 } else { dt.hour as u16 };
            if n >= 2 { two(h as i64) } else { h.to_string() }
        }
        'm' => {
            if n >= 2 {
                two(dt.minute as i64)
            } else {
                dt.minute.to_string()
            }
        }
        's' => {
            if n >= 2 {
                two(dt.second as i64)
            } else {
                dt.second.to_string()
            }
        }
        'a' | 'b' | 'B' => if dt.hour < 12 { s.am } else { s.pm }.to_string(),
        'S' => {
            // Fractional second: `n` digits from the millisecond field.
            let ms = dt.millisecond.min(999);
            let base = alloc::format!("{ms:03}");
            if n <= 3 {
                base[..n].to_string()
            } else {
                let mut out = base;
                for _ in 0..(n - 3) {
                    out.push('0');
                }
                out
            }
        }
        _ => String::new(), // unsupported field (e.g. time zone) -> nothing
    }
}

/// Map a CLDR field letter to its ECMA-402 `formatToParts` part type.
fn part_type(ch: char) -> DateTimePartType {
    use DateTimePartType::*;
    match ch {
        'y' | 'Y' | 'u' | 'U' | 'r' => Year,
        'M' | 'L' => Month,
        'd' | 'D' | 'F' | 'g' => Day,
        'E' | 'e' | 'c' => Weekday,
        'h' | 'H' | 'k' | 'K' => Hour,
        'm' => Minute,
        's' => Second,
        'S' | 'A' => FractionalSecond,
        'a' | 'b' | 'B' => DayPeriod,
        'G' => Era,
        'z' | 'Z' | 'O' | 'v' | 'V' | 'X' | 'x' => TimeZoneName,
        _ => Literal,
    }
}

/// Parts-producing core of [`render`]: interprets a CLDR pattern into tagged
/// [`DateTimePart`]s. Adjacent literals are coalesced and empty field outputs
/// (unsupported letters) are dropped, so joining the part values reproduces
/// [`render`]'s string exactly.
fn render_parts(pattern: &str, dt: &DateTime, s: &CalendarSpec) -> Vec<DateTimePart> {
    fn push_lit(parts: &mut Vec<DateTimePart>, text: &str) {
        if text.is_empty() {
            return;
        }
        if let Some(last) = parts.last_mut() {
            if last.kind == DateTimePartType::Literal {
                last.value.push_str(text);
                return;
            }
        }
        parts.push(DateTimePart {
            kind: DateTimePartType::Literal,
            value: String::from(text),
        });
    }

    let c: Vec<char> = pattern.chars().collect();
    let mut parts: Vec<DateTimePart> = Vec::new();
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch == '\'' {
            i += 1;
            if i < c.len() && c[i] == '\'' {
                push_lit(&mut parts, "'");
                i += 1;
                continue;
            }
            let mut lit = String::new();
            while i < c.len() && c[i] != '\'' {
                lit.push(c[i]);
                i += 1;
            }
            i += 1; // closing quote
            push_lit(&mut parts, &lit);
        } else if ch.is_ascii_alphabetic() {
            let start = i;
            while i < c.len() && c[i] == ch {
                i += 1;
            }
            let val = field(ch, i - start, dt, s);
            if !val.is_empty() {
                parts.push(DateTimePart {
                    kind: part_type(ch),
                    value: val,
                });
            }
        } else {
            let mut buf = [0u8; 4];
            push_lit(&mut parts, ch.encode_utf8(&mut buf));
            i += 1;
        }
    }
    parts
}

/// Interpret a CLDR date/time pattern, handling quoted literals.
fn render(pattern: &str, dt: &DateTime, s: &CalendarSpec) -> String {
    let mut out = String::new();
    for part in render_parts(pattern, dt, s) {
        out.push_str(&part.value);
    }
    out
}

fn spec(lang: &str) -> CalendarSpec {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    loop {
        if let Some(s) = calendar_spec(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return calendar_spec("en").expect("root calendar present"),
        }
    }
}

/// Format the date part of `dt` in `lang` at the given `style`.
#[must_use]
pub fn format_date(lang: &str, dt: &DateTime, style: DateStyle) -> String {
    let s = spec(lang);
    render(s.date[style.idx()], dt, &s)
}

/// Format the time part of `dt` in `lang` at the given `style`.
#[must_use]
pub fn format_time(lang: &str, dt: &DateTime, style: DateStyle) -> String {
    let s = spec(lang);
    render(s.time[style.idx()], dt, &s)
}

/// Format `dt` with a CLDR *skeleton* (e.g. `"yMMMd"`, `"Hm"`, `"MMMEd"`) — the
/// modern, flexible date API: the skeleton names the fields you want and the
/// locale supplies their order and punctuation. Falls back through the locale
/// chain (and to English), then to the medium date pattern for an unknown
/// skeleton.
///
/// ```
/// use intl::datetime::{DateTime, format_skeleton};
/// let dt = DateTime { year: 2026, month: 6, day: 4, hour: 14, minute: 30, second: 5, millisecond: 0 };
/// assert_eq!(format_skeleton("en", &dt, "yMMMd"), "Jun 4, 2026");
/// assert_eq!(format_skeleton("de", &dt, "yMMMd"), "4. Juni 2026");
/// assert_eq!(format_skeleton("en", &dt, "Hm"), "14:30");
/// ```
#[must_use]
pub fn format_skeleton(lang: &str, dt: &DateTime, skeleton: &str) -> String {
    let s = spec(lang);
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let pattern = loop {
        if let Some(p) = crate::cldr::skeleton_pattern(&norm[..end], skeleton) {
            break p;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break crate::cldr::skeleton_pattern("en", skeleton).unwrap_or(s.date[2]),
        }
    };
    render(pattern, dt, &s)
}

/// Render a non-Gregorian date with a calendar's month names/era; the weekday
/// name (if any) uses the Gregorian day names at the date's `jdn`.
fn render_alt(
    cal: &crate::cldr::AltCalSpec,
    style: DateStyle,
    year: i64,
    month: i64,
    day: i64,
    jdn: i64,
    greg: &CalendarSpec,
) -> String {
    let wd = ((jdn.rem_euclid(7) + 1) % 7) as usize; // 0 = Sunday
    let c: Vec<char> = cal.date[style.idx()].chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch == '\'' {
            i += 1;
            if i < c.len() && c[i] == '\'' {
                out.push('\'');
                i += 1;
                continue;
            }
            while i < c.len() && c[i] != '\'' {
                out.push(c[i]);
                i += 1;
            }
            i += 1;
        } else if ch.is_ascii_alphabetic() {
            let start = i;
            while i < c.len() && c[i] == ch {
                i += 1;
            }
            let (n, m) = (i - start, month as usize);
            let mi = m.clamp(1, 12) - 1; // unvalidated month must not index OOB
            match ch {
                'y' | 'Y' => out.push_str(&year.to_string()),
                'M' | 'L' => match n {
                    1 => out.push_str(&m.to_string()),
                    2 => out.push_str(&two(m as i64)),
                    3 => out.push_str(cal.months_abbr[mi]),
                    _ => out.push_str(cal.months_wide[mi]),
                },
                'd' => out.push_str(&day.to_string()),
                'E' | 'e' | 'c' => out.push_str(if n >= 4 {
                    greg.days_wide[wd]
                } else {
                    greg.days_abbr[wd]
                }),
                'G' => out.push_str(cal.era),
                _ => {}
            }
        } else {
            out.push(ch);
            i += 1;
        }
    }
    out
}

fn alt_spec(lang: &str, f: fn(&str) -> Option<crate::cldr::AltCalSpec>) -> crate::cldr::AltCalSpec {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    loop {
        if let Some(s) = f(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return f("en").expect("root calendar present"),
        }
    }
}

/// Format an Islamic (Hijri) date in `lang`, e.g.
/// `format_islamic_date("en", 1445, 9, 1, DateStyle::Long)` →
/// `"Ramadan 1, 1445 AH"` (localized month names + era).
#[must_use]
pub fn format_islamic_date(
    lang: &str,
    year: i64,
    month: i64,
    day: i64,
    style: DateStyle,
) -> String {
    let cal = alt_spec(lang, crate::cldr::islamic_spec);
    let jdn = crate::calendar::islamic_to_jdn(year, month, day);
    render_alt(&cal, style, year, month, day, jdn, &spec(lang))
}

/// Format a Persian (Solar Hijri) date in `lang`, e.g.
/// `format_persian_date("en", 1404, 1, 1, DateStyle::Long)` →
/// `"Farvardin 1, 1404 AP"` (localized month names + era).
#[must_use]
pub fn format_persian_date(
    lang: &str,
    year: i64,
    month: i64,
    day: i64,
    style: DateStyle,
) -> String {
    let cal = alt_spec(lang, crate::cldr::persian_spec);
    let jdn = crate::calendar::persian_to_jdn(year, month, day);
    render_alt(&cal, style, year, month, day, jdn, &spec(lang))
}

/// Format a fixed UTC offset (in minutes) in the localized GMT form, e.g.
/// `"GMT+5:30"`-style output: `format_gmt_offset("en", 330)` → `"GMT+05:30"`,
/// `format_gmt_offset("fr", -480)` → `"UTC−08:00"`, `0` → `"GMT"` / `"UTC"`.
/// This is the data-light part of time-zone support (a concrete offset, not the
/// IANA zone database).
#[must_use]
pub fn format_gmt_offset(lang: &str, offset_minutes: i32) -> String {
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let tz = loop {
        if let Some(t) = crate::cldr::tz_spec(&norm[..end]) {
            break t;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break crate::cldr::tz_spec("en").expect("root tz present"),
        }
    };
    if offset_minutes == 0 {
        return String::from(tz.zero);
    }
    let (pos, neg) = tz.hour.split_once(';').unwrap_or((tz.hour, tz.hour));
    let sub = if offset_minutes >= 0 { pos } else { neg };
    let (h, m) = (
        offset_minutes.unsigned_abs() / 60,
        offset_minutes.unsigned_abs() % 60,
    );
    let body = sub
        .replace("HH", &alloc::format!("{h:02}"))
        .replace("mm", &alloc::format!("{m:02}"));
    tz.gmt.replace("{0}", &body)
}

/// Format both date and time, combined with the locale's date+time pattern.
#[must_use]
pub fn format_datetime(
    lang: &str,
    dt: &DateTime,
    date_style: DateStyle,
    time_style: DateStyle,
) -> String {
    let s = spec(lang);
    let date = render(s.date[date_style.idx()], dt, &s);
    let time = render(s.time[time_style.idx()], dt, &s);
    s.datetime[date_style.idx()]
        .replace("{1}", &date)
        .replace("{0}", &time)
}

// ---------------------------------------------------------------------------
// ECMA-402-style component options + formatToParts
// ---------------------------------------------------------------------------

/// Width for a numeric field (ECMA-402 `"numeric"` / `"2-digit"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Numeric2Digit {
    /// `"numeric"` — no padding.
    Numeric,
    /// `"2-digit"` — zero-padded to two digits.
    TwoDigit,
}

/// Month presentation (ECMA-402 `month`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthStyle {
    /// Numeric (`6`).
    Numeric,
    /// Two-digit (`06`).
    TwoDigit,
    /// Wide name (`June`).
    Long,
    /// Abbreviated name (`Jun`).
    Short,
    /// Narrow name (`J`).
    Narrow,
}

/// Name width for weekday / era / day period (ECMA-402 `"long"`/`"short"`/`"narrow"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameStyle {
    /// Wide.
    Long,
    /// Abbreviated.
    Short,
    /// Narrow.
    Narrow,
}

/// Time-zone-name presentation (ECMA-402 `timeZoneName`). Only the offset forms
/// are rendered today (from a caller-supplied offset); named/generic forms fall
/// back to the offset (no metazone data).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneNameStyle {
    /// Long specific name; falls back to the long offset.
    Long,
    /// Short specific name; falls back to the short offset.
    Short,
    /// Short localized GMT offset.
    ShortOffset,
    /// Long localized GMT offset.
    LongOffset,
    /// Short generic name; falls back to the short offset.
    ShortGeneric,
    /// Long generic name; falls back to the long offset.
    LongGeneric,
}

/// Hour cycle (ECMA-402 `hourCycle`). `H11`/`H12` use the 12-hour clock,
/// `H23`/`H24` the 24-hour clock (the 0-vs-1 origin is approximated).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HourCycle {
    /// 0–11 with day period.
    H11,
    /// 1–12 with day period.
    H12,
    /// 0–23.
    H23,
    /// 1–24.
    H24,
}

/// Per-component options for [`format_options`] / [`format_to_parts`], modeled on
/// `Intl.DateTimeFormat`. [`Default`] is all-`None` (which formats a numeric
/// year/month/day). `date_style`/`time_style` are a shortcut that is mutually
/// exclusive with the component fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DateTimeFormatOptions {
    /// Weekday name width.
    pub weekday: Option<NameStyle>,
    /// Era name width.
    pub era: Option<NameStyle>,
    /// Year width.
    pub year: Option<Numeric2Digit>,
    /// Month presentation.
    pub month: Option<MonthStyle>,
    /// Day width.
    pub day: Option<Numeric2Digit>,
    /// Hour width.
    pub hour: Option<Numeric2Digit>,
    /// Minute width.
    pub minute: Option<Numeric2Digit>,
    /// Second width.
    pub second: Option<Numeric2Digit>,
    /// Fractional-second digits (1–3).
    pub fractional_second_digits: Option<u8>,
    /// Day-period width (only affects 12-hour formatting).
    pub day_period: Option<NameStyle>,
    /// Time-zone-name presentation (requires `tz_offset_minutes`).
    pub time_zone_name: Option<TimeZoneNameStyle>,
    /// Hour cycle (overrides `hour12`).
    pub hour_cycle: Option<HourCycle>,
    /// 12-hour clock toggle.
    pub hour12: Option<bool>,
    /// Date-style shortcut (mutually exclusive with components).
    pub date_style: Option<DateStyle>,
    /// Time-style shortcut (mutually exclusive with components).
    pub time_style: Option<DateStyle>,
    /// Caller-supplied UTC offset in minutes, used for `time_zone_name`.
    pub tz_offset_minutes: Option<i32>,
}

/// Error returned by [`format_options`] / [`format_to_parts`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateTimeFormatError {
    /// `date_style`/`time_style` were combined with component options.
    ConflictingOptions,
}

fn normalize_lang(lang: &str) -> String {
    lang.chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

/// The locale's default clock letter (`h` = 12-hour, `H` = 24-hour), read from
/// its CLDR time patterns: the first hour field letter found outside quotes.
/// `K`/`k` collapse to `h`/`H`. Defaults to `H` if none is found.
fn locale_hour_letter(s: &CalendarSpec) -> char {
    for pat in s.time {
        let chars: Vec<char> = pat.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '\'' => {
                    i += 1;
                    while i < chars.len() && chars[i] != '\'' {
                        i += 1;
                    }
                    i += 1;
                }
                'h' | 'K' => return 'h',
                'H' | 'k' => return 'H',
                _ => i += 1,
            }
        }
    }
    'H'
}

/// The 12- vs 24-hour clock **family** letter (`h` or `H`) implied by the cycle
/// / `hour12` options, falling back to the locale default (`loc`). Used for
/// skeleton lookup (availableFormats keys only use `h`/`H`) and the day-period
/// decision.
fn hour_letter(o: &DateTimeFormatOptions, loc: char) -> char {
    match (o.hour_cycle, o.hour12) {
        (Some(HourCycle::H11 | HourCycle::H12), _) => 'h',
        (Some(HourCycle::H23 | HourCycle::H24), _) => 'H',
        (None, Some(true)) => 'h',
        (None, Some(false)) => 'H',
        (None, None) => loc,
    }
}

/// The exact CLDR hour field letter for the requested cycle: `h` = h12 (1–12),
/// `K` = h11 (0–11), `H` = h23 (0–23), `k` = h24 (1–24). With no explicit cycle,
/// uses the locale default family (`loc`, always `h`/`H`).
fn hour_exact_letter(o: &DateTimeFormatOptions, loc: char) -> char {
    match (o.hour_cycle, o.hour12) {
        (Some(HourCycle::H11), _) => 'K',
        (Some(HourCycle::H12), _) => 'h',
        (Some(HourCycle::H23), _) => 'H',
        (Some(HourCycle::H24), _) => 'k',
        (None, Some(true)) => 'h',
        (None, Some(false)) => 'H',
        (None, None) => loc,
    }
}

/// Build the canonical date-field skeleton (`G y M… E d`) and whether any date
/// component was requested.
fn build_date_skeleton(o: &DateTimeFormatOptions) -> (String, bool) {
    let mut sk = String::new();
    if o.era.is_some() {
        sk.push('G');
    }
    if o.year.is_some() {
        sk.push('y');
    }
    if let Some(m) = o.month {
        // Use a representative width for *lookup* only — numeric (`M`) vs name
        // (`MMM`); the requested width is applied later by `patch_widths`. CLDR
        // availableFormats key combos use `M` or `MMM`, so collapsing the name
        // widths to `MMM` maximizes exact-match hits (e.g. a wide-month +
        // weekday request matches `MMMEd` instead of falling back and losing the
        // weekday).
        let c = match m {
            MonthStyle::Numeric | MonthStyle::TwoDigit => 1,
            MonthStyle::Short | MonthStyle::Long | MonthStyle::Narrow => 3,
        };
        for _ in 0..c {
            sk.push('M');
        }
    }
    if o.weekday.is_some() {
        sk.push('E');
    }
    if o.day.is_some() {
        sk.push('d');
    }
    let any = o.era.is_some()
        || o.year.is_some()
        || o.month.is_some()
        || o.weekday.is_some()
        || o.day.is_some();
    (sk, any)
}

/// Build the canonical time-field skeleton (`h/H m s`) and whether any time
/// component was requested. `loc` is the locale default hour letter.
fn build_time_skeleton(o: &DateTimeFormatOptions, loc: char) -> (String, bool) {
    let mut sk = String::new();
    if o.hour.is_some() {
        sk.push(hour_letter(o, loc));
    }
    if o.minute.is_some() {
        sk.push('m');
    }
    if o.second.is_some() {
        sk.push('s');
    }
    let any = o.hour.is_some() || o.minute.is_some() || o.second.is_some();
    (sk, any)
}

/// Resolve a skeleton to a locale pattern through the fallback chain.
fn resolve_one(lang: &str, s: &CalendarSpec, skeleton: &str) -> String {
    let norm = normalize_lang(lang);
    let mut end = norm.len();
    loop {
        if let Some(p) = crate::cldr::skeleton_pattern(&norm[..end], skeleton) {
            return String::from(p);
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => {
                return crate::cldr::skeleton_pattern("en", skeleton)
                    .map_or_else(|| String::from(s.date[2]), String::from);
            }
        }
    }
}

/// Replace each run of a field letter (in `from`) with `to` repeated `count`,
/// skipping quoted literals.
fn set_field(pattern: &str, from: &[char], to: char, count: usize) -> String {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\'' {
            out.push(ch);
            i += 1;
            if i < chars.len() && chars[i] == '\'' {
                out.push('\'');
                i += 1;
                continue;
            }
            while i < chars.len() && chars[i] != '\'' {
                out.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                out.push('\'');
                i += 1;
            }
        } else if from.contains(&ch) {
            while i < chars.len() && chars[i] == ch {
                i += 1;
            }
            for _ in 0..count {
                out.push(to);
            }
        } else {
            out.push(ch);
            i += 1;
        }
    }
    out
}

/// Apply the requested component widths to a resolved pattern.
fn patch_widths(pattern: &str, o: &DateTimeFormatOptions, loc: char) -> String {
    let mut p = String::from(pattern);
    if let Some(y) = o.year {
        p = set_field(
            &p,
            &['y'],
            'y',
            if y == Numeric2Digit::TwoDigit { 2 } else { 1 },
        );
    }
    if let Some(m) = o.month {
        let c = match m {
            MonthStyle::Numeric => 1,
            MonthStyle::TwoDigit => 2,
            MonthStyle::Short => 3,
            MonthStyle::Long => 4,
            MonthStyle::Narrow => 5,
        };
        p = set_field(&p, &['M', 'L'], 'M', c);
    }
    if let Some(d) = o.day {
        p = set_field(
            &p,
            &['d'],
            'd',
            if d == Numeric2Digit::TwoDigit { 2 } else { 1 },
        );
    }
    if let Some(w) = o.weekday {
        let c = match w {
            NameStyle::Long => 4,
            NameStyle::Short => 3,
            NameStyle::Narrow => 5,
        };
        p = set_field(&p, &['E', 'e', 'c'], 'E', c);
    }
    if let Some(e) = o.era {
        let c = match e {
            NameStyle::Long => 4,
            NameStyle::Short => 1,
            NameStyle::Narrow => 5,
        };
        p = set_field(&p, &['G'], 'G', c);
    }
    if let Some(h) = o.hour {
        let letter = hour_exact_letter(o, loc);
        let c = if h == Numeric2Digit::TwoDigit { 2 } else { 1 };
        p = set_field(&p, &['h', 'H', 'k', 'K'], letter, c);
    }
    if let Some(mi) = o.minute {
        p = set_field(
            &p,
            &['m'],
            'm',
            if mi == Numeric2Digit::TwoDigit { 2 } else { 1 },
        );
    }
    if let Some(se) = o.second {
        p = set_field(
            &p,
            &['s'],
            's',
            if se == Numeric2Digit::TwoDigit { 2 } else { 1 },
        );
    }
    p
}

/// Inject a locale decimal separator + `n` `S` digits right after the seconds
/// run (skeletons carry no `S`), for `fractionalSecondDigits`.
fn inject_fractional(pattern: &str, lang: &str, n: u8) -> String {
    let dec = crate::cldr::number_spec(lang).map_or(".", |s| s.decimal);
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut done = false;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\'' {
            out.push(ch);
            i += 1;
            while i < chars.len() && chars[i] != '\'' {
                out.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                out.push('\'');
                i += 1;
            }
        } else if ch == 's' && !done {
            while i < chars.len() && chars[i] == 's' {
                out.push('s');
                i += 1;
            }
            out.push_str(dec);
            for _ in 0..n {
                out.push('S');
            }
            done = true;
        } else {
            out.push(ch);
            i += 1;
        }
    }
    out
}

/// Remove field runs whose letter is not in `keep`, then tidy separators: drop
/// leading/trailing literal tokens and merge adjacent literals. Quoted literals
/// are unwrapped into plain text. Keeps the common CLDR patterns clean when a
/// best-fit skeleton carried extra fields.
fn strip_fields(pattern: &str, keep: &[char]) -> String {
    enum Tok {
        Field(String),
        Lit(String),
    }
    let chars: Vec<char> = pattern.chars().collect();
    let mut toks: Vec<Tok> = Vec::new();
    let mut i = 0;
    let mut lit = String::new();
    let flush = |lit: &mut String, toks: &mut Vec<Tok>| {
        if !lit.is_empty() {
            toks.push(Tok::Lit(core::mem::take(lit)));
        }
    };
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\'' {
            i += 1;
            if i < chars.len() && chars[i] == '\'' {
                lit.push('\'');
                i += 1;
                continue;
            }
            while i < chars.len() && chars[i] != '\'' {
                lit.push(chars[i]);
                i += 1;
            }
            i += 1;
        } else if ch.is_ascii_alphabetic() {
            flush(&mut lit, &mut toks);
            let start = i;
            while i < chars.len() && chars[i] == ch {
                i += 1;
            }
            let run: String = chars[start..i].iter().collect();
            if keep.contains(&ch) {
                toks.push(Tok::Field(run));
            }
            // else: dropped
        } else {
            lit.push(ch);
            i += 1;
        }
    }
    flush(&mut lit, &mut toks);

    // Drop leading/trailing literals.
    while matches!(toks.first(), Some(Tok::Lit(_))) {
        toks.remove(0);
    }
    while matches!(toks.last(), Some(Tok::Lit(_))) {
        toks.pop();
    }
    // Merge adjacent literals and reassemble.
    let mut out = String::new();
    let mut prev_lit = false;
    for t in toks {
        match t {
            Tok::Field(f) => {
                out.push_str(&f);
                prev_lit = false;
            }
            Tok::Lit(l) => {
                if prev_lit {
                    // collapse a doubled separator left by a removed middle field
                    continue;
                }
                out.push_str(&l);
                prev_lit = true;
            }
        }
    }
    out
}

/// Resolve a full pattern string for the requested options.
fn resolve_pattern(
    lang: &str,
    s: &CalendarSpec,
    o: &DateTimeFormatOptions,
) -> Result<String, DateTimeFormatError> {
    let has_components = o.weekday.is_some()
        || o.era.is_some()
        || o.year.is_some()
        || o.month.is_some()
        || o.day.is_some()
        || o.hour.is_some()
        || o.minute.is_some()
        || o.second.is_some()
        || o.fractional_second_digits.is_some()
        || o.day_period.is_some();

    if o.date_style.is_some() || o.time_style.is_some() {
        if has_components {
            return Err(DateTimeFormatError::ConflictingOptions);
        }
        let pat = match (o.date_style, o.time_style) {
            (Some(d), Some(t)) => s.datetime[d.idx()]
                .replace("{1}", s.date[d.idx()])
                .replace("{0}", s.time[t.idx()]),
            (Some(d), None) => String::from(s.date[d.idx()]),
            (None, Some(t)) => String::from(s.time[t.idx()]),
            (None, None) => unreachable!(),
        };
        return Ok(pat);
    }

    let loc_hour = locale_hour_letter(s);
    let (date_sk, mut want_date) = build_date_skeleton(o);
    let (time_sk, want_time) = build_time_skeleton(o, loc_hour);
    // ECMA-402 default when nothing is requested: numeric year/month/day.
    let date_sk = if !want_date && !want_time {
        want_date = true;
        String::from("yMd")
    } else {
        date_sk
    };

    let date_pat = if want_date {
        Some(resolve_one(lang, s, &date_sk))
    } else {
        None
    };
    let time_pat = if want_time {
        Some(resolve_one(lang, s, &time_sk))
    } else {
        None
    };

    let mut combined = match (date_pat, time_pat) {
        (Some(d), Some(t)) => s.datetime[2].replace("{1}", &d).replace("{0}", &t),
        (Some(d), None) => d,
        (None, Some(t)) => t,
        (None, None) => String::from(s.date[2]),
    };

    // When explicit components were given, drop any extra fields a best-fit
    // pattern carried (the defaulted "yMd" path resolves exactly, so skip it).
    if has_components {
        let mut keep: Vec<char> = Vec::new();
        if o.year.is_some() {
            keep.extend(['y', 'Y', 'u', 'U', 'r']);
        }
        if o.month.is_some() {
            keep.extend(['M', 'L']);
        }
        if o.day.is_some() {
            keep.extend(['d', 'D', 'F', 'g']);
        }
        if o.weekday.is_some() {
            keep.extend(['E', 'e', 'c']);
        }
        if o.era.is_some() {
            keep.push('G');
        }
        if o.hour.is_some() {
            keep.extend(['h', 'H', 'k', 'K']);
        }
        if o.minute.is_some() {
            keep.push('m');
        }
        if o.second.is_some() {
            keep.push('s');
        }
        // Keep the day period when explicitly asked or implied by a 12-hour clock.
        if o.day_period.is_some() || (o.hour.is_some() && hour_letter(o, loc_hour) == 'h') {
            keep.extend(['a', 'b', 'B']);
        }
        combined = strip_fields(&combined, &keep);
    }

    combined = patch_widths(&combined, o, loc_hour);
    if let Some(n) = o.fractional_second_digits {
        combined = inject_fractional(&combined, lang, n);
    }
    Ok(combined)
}

/// The time-zone-name string for an offset and presentation style.
fn zone_string(lang: &str, style: TimeZoneNameStyle, offset: i32) -> String {
    // Long offset = zero-padded GMT form; short offset trims leading zero in the
    // hour. Named/generic styles fall back to the offset (no metazone data).
    let long = format_gmt_offset(lang, offset);
    match style {
        TimeZoneNameStyle::LongOffset
        | TimeZoneNameStyle::Long
        | TimeZoneNameStyle::LongGeneric => long,
        TimeZoneNameStyle::ShortOffset
        | TimeZoneNameStyle::Short
        | TimeZoneNameStyle::ShortGeneric => {
            // "GMT-08:00" -> "GMT-8:00": drop a single leading zero after the sign.
            if let Some(pos) = long.find(['+', '-', '\u{2212}']) {
                let (head, tail) = long.split_at(pos + 1);
                let trimmed = tail.strip_prefix('0').unwrap_or(tail);
                alloc::format!("{head}{trimmed}")
            } else {
                long
            }
        }
    }
}

/// Format `dt` in `lang` with ECMA-402-style component options, returning the
/// tagged parts (`Intl.DateTimeFormat.prototype.formatToParts`).
///
/// ```
/// use intl::datetime::{DateTime, DateTimeFormatOptions, MonthStyle, Numeric2Digit, format_to_parts};
/// let dt = DateTime { year: 2026, month: 6, day: 4, hour: 14, minute: 30, second: 5, millisecond: 0 };
/// let opts = DateTimeFormatOptions {
///     year: Some(Numeric2Digit::Numeric),
///     month: Some(MonthStyle::Short),
///     day: Some(Numeric2Digit::Numeric),
///     ..Default::default()
/// };
/// let parts = format_to_parts("en", &dt, &opts).unwrap();
/// let joined: String = parts.iter().map(|p| p.value.as_str()).collect();
/// assert_eq!(joined, "Jun 4, 2026");
/// ```
///
/// # Errors
/// Returns [`DateTimeFormatError::ConflictingOptions`] if `date_style`/`time_style`
/// are combined with component fields.
pub fn format_to_parts(
    lang: &str,
    dt: &DateTime,
    opts: &DateTimeFormatOptions,
) -> Result<Vec<DateTimePart>, DateTimeFormatError> {
    let s = spec(lang);
    let pattern = resolve_pattern(lang, &s, opts)?;
    let mut parts = render_parts(&pattern, dt, &s);
    if let (Some(style), Some(off)) = (opts.time_zone_name, opts.tz_offset_minutes) {
        parts.push(DateTimePart {
            kind: DateTimePartType::Literal,
            value: String::from(" "),
        });
        parts.push(DateTimePart {
            kind: DateTimePartType::TimeZoneName,
            value: zone_string(lang, style, off),
        });
    }
    Ok(parts)
}

/// Format `dt` in `lang` with ECMA-402-style component options.
///
/// ```
/// use intl::datetime::{DateTime, DateTimeFormatOptions, DateStyle, format_options};
/// let dt = DateTime { year: 2026, month: 6, day: 4, hour: 14, minute: 30, second: 5, millisecond: 0 };
/// let opts = DateTimeFormatOptions { date_style: Some(DateStyle::Long), ..Default::default() };
/// assert_eq!(format_options("en", &dt, &opts).unwrap(), "June 4, 2026");
/// ```
///
/// # Errors
/// Returns [`DateTimeFormatError::ConflictingOptions`] if `date_style`/`time_style`
/// are combined with component fields.
pub fn format_options(
    lang: &str,
    dt: &DateTime,
    opts: &DateTimeFormatOptions,
) -> Result<String, DateTimeFormatError> {
    let parts = format_to_parts(lang, dt, opts)?;
    let mut out = String::new();
    for p in parts {
        out.push_str(&p.value);
    }
    Ok(out)
}

/// Format `dt` with a CLDR skeleton, returning tagged parts (see
/// [`format_skeleton`] and [`format_to_parts`]).
#[must_use]
pub fn format_skeleton_to_parts(lang: &str, dt: &DateTime, skeleton: &str) -> Vec<DateTimePart> {
    let s = spec(lang);
    let norm = normalize_lang(lang);
    let mut end = norm.len();
    let pattern = loop {
        if let Some(p) = crate::cldr::skeleton_pattern(&norm[..end], skeleton) {
            break String::from(p);
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => {
                break crate::cldr::skeleton_pattern("en", skeleton)
                    .map_or_else(|| String::from(s.date[2]), String::from);
            }
        }
    };
    render_parts(&pattern, dt, &s)
}
