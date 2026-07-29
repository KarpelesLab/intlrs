#![cfg(all(feature = "datetime", feature = "iana-tz"))]
use intl::datetime::*;
#[test]
fn tz_in_pattern_position() {
    let d = DateTime {
        year: 2026,
        month: 7,
        day: 15,
        hour: 12,
        minute: 0,
        second: 0,
        millisecond: 0,
    };
    for style in [DateStyle::Full, DateStyle::Long, DateStyle::Medium] {
        let mut o = DateTimeFormatOptions::default();
        o.time_style = Some(style);
        o.time_zone = Some("America/Los_Angeles");
        println!(
            "{style:?} (no tz opt)  = {:?}",
            format_options("en", &d, &o).unwrap()
        );
        o.time_zone_name = Some(TimeZoneNameStyle::Long);
        println!(
            "{style:?} (+tzname)    = {:?}",
            format_options("en", &d, &o).unwrap()
        );
    }
}
