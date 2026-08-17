use chrono::{TimeZone, Utc};
use umt_rust::date::*;

#[test]
fn test_module_exports() {
    // Test that all exports are accessible
    assert!(umt_is_leap_year(2020));

    let _ = umt_now(0);
    let _ = umt_now_jst();

    let _ = umt_new_date_int(2021, 1, 1, None, None, None, None);
    let _ = umt_new_date_string("2021-01-01", None, None, None, None, None);

    let _ = umt_birthday(2000, 1, 1, None);

    let start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 3, 0, 0, 0).unwrap();
    let _ = umt_date_range(start, end);
    let _ = umt_date_range_with_step(start, end, 1);

    let _ = umt_day_of_week(Some(2025), Some(1), Some(1), None);
    let _ = umt_today_day_of_week(None);

    let _ = umt_get_day(0, DayLanguage::En);
    let _ = umt_get_day_ja(0);
    let _ = umt_get_day_en(0);

    let _ = umt_get_timezone_offset_string(540);
    let _ = umt_get_timezone_offset_string_compact(540);
    let _ = umt_get_local_timezone_offset_string();

    let date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let _ = umt_format(&date, "YYYY-MM-DD", 0);
    let _ = umt_format_iso(&date, 0);

    let _ = umt_is_between(&date, &date, &date, None, DateInclusivity::Exclusive);
    let _ = umt_add_business_days(&date, 1, &[]);
    let _ = umt_sub_business_days(&date, 1, &[]);
    let _ = umt_get_quarter(&date);
    let _ = umt_week_of_year(&date);
    let unix = umt_from_unix(0.0, UnixTimeUnit::Second);
    let _ = umt_to_unix(&unix, UnixTimeUnit::Second);
}
