use chrono::{TimeZone, Utc};
use umt_rust::date::{
    DayLanguage, umt_birthday, umt_date_range, umt_day_of_week, umt_format, umt_get_day,
    umt_is_leap_year, umt_new_date_int,
};

#[test]
fn test_date_range_and_format() {
    let start_date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2025, 1, 5, 0, 0, 0).unwrap();
    let dates = umt_date_range(start_date, end_date);

    let formatted_dates: Vec<String> = dates
        .iter()
        .map(|date| umt_format(date, "YYYY-MM-DD", 0))
        .collect();

    assert_eq!(
        formatted_dates,
        vec![
            "2025-01-01",
            "2025-01-02",
            "2025-01-03",
            "2025-01-04",
            "2025-01-05",
        ]
    );
}

#[test]
fn test_age_and_birthday_format() {
    let birth_year = 2000;
    let birth_month = 3;
    let birth_day = 15;

    let age = umt_birthday(birth_year, birth_month, birth_day, None);
    // Note: age calculation depends on current date, so just check it's reasonable
    assert!(age >= 23);

    let day_num = umt_day_of_week(Some(birth_year), Some(birth_month), Some(birth_day), None);
    let day_name = umt_get_day(day_num.unwrap(), DayLanguage::En);

    let birth_date =
        umt_new_date_int(birth_year, birth_month, birth_day, None, None, None, None).unwrap();
    let formatted_birth = umt_format(&birth_date, "MM/DD/YYYY", 0);

    assert_eq!(day_name, "Wed");
    assert_eq!(formatted_birth, "03/15/2000");
}

#[test]
fn test_leap_year_dates() {
    let leap_years = vec![2020, 2024, 2028];
    let non_leap_years = vec![2021, 2022, 2023];

    for year in leap_years {
        assert!(umt_is_leap_year(year));
        let feb1 = umt_new_date_int(year, 2, 1, None, None, None, None).unwrap();
        let feb29 = umt_new_date_int(year, 2, 29, None, None, None, None).unwrap();
        let dates = umt_date_range(feb1, feb29);
        assert_eq!(dates.len(), 29);
        assert_eq!(
            umt_format(&dates[dates.len() - 1], "YYYY-MM-DD", 0),
            format!("{}-02-29", year)
        );
    }

    for year in non_leap_years {
        assert!(!umt_is_leap_year(year));
    }
}
