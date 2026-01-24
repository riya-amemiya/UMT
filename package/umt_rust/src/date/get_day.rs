//! Day name localization utilities.

/// Supported languages for day names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayLanguage {
    /// German (Deutsch)
    De,
    /// Korean (Korean)
    Ko,
    /// English
    En,
    /// Japanese (default)
    Ja,
    /// French (Francais)
    Fr,
}

impl DayLanguage {
    /// Creates a DayLanguage from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - Language code string ("de", "ko", "en", "ja", "fr")
    ///
    /// # Returns
    ///
    /// The corresponding DayLanguage, or Ja (Japanese) as default.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "de" => DayLanguage::De,
            "ko" => DayLanguage::Ko,
            "en" => DayLanguage::En,
            "ja" => DayLanguage::Ja,
            "fr" => DayLanguage::Fr,
            _ => DayLanguage::Ja,
        }
    }
}

const DAYS_DE: [&str; 7] = ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"];
const DAYS_KO: [&str; 7] = ["일", "월", "화", "수", "목", "금", "토"];
const DAYS_EN: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const DAYS_JA: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];
const DAYS_FR: [&str; 7] = ["Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"];

/// Converts a day number to a localized day name.
///
/// # Arguments
///
/// * `day` - Number representing the day (0-6, where 0 is Sunday).
/// * `lang` - Language for the day name.
///
/// # Returns
///
/// The day name string in the specified language.
/// Returns Sunday for invalid day numbers.
///
/// # Examples
///
/// ```
/// use umt_rust::date::{umt_get_day, DayLanguage};
///
/// assert_eq!(umt_get_day(0, DayLanguage::En), "Sun");
/// assert_eq!(umt_get_day(0, DayLanguage::Ja), "日");
/// assert_eq!(umt_get_day(3, DayLanguage::De), "Mi");
/// ```
#[inline]
pub fn umt_get_day(day: i32, lang: DayLanguage) -> &'static str {
    let days = match lang {
        DayLanguage::De => &DAYS_DE,
        DayLanguage::Ko => &DAYS_KO,
        DayLanguage::En => &DAYS_EN,
        DayLanguage::Ja => &DAYS_JA,
        DayLanguage::Fr => &DAYS_FR,
    };

    match day {
        0..=6 => days[day as usize],
        _ => days[0], // Return Sunday for invalid day numbers
    }
}

/// Converts a day number to a localized day name using default language (Japanese).
///
/// # Arguments
///
/// * `day` - Number representing the day (0-6, where 0 is Sunday).
///
/// # Returns
///
/// The day name string in Japanese.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_day_default;
///
/// assert_eq!(umt_get_day_default(0), "日");
/// assert_eq!(umt_get_day_default(1), "月");
/// ```
#[inline]
pub fn umt_get_day_default(day: i32) -> &'static str {
    umt_get_day(day, DayLanguage::Ja)
}

/// Converts a day number to a localized day name using a string language code.
///
/// # Arguments
///
/// * `day` - Number representing the day (0-6, where 0 is Sunday).
/// * `lang` - Language code string ("de", "ko", "en", "ja", "fr").
///
/// # Returns
///
/// The day name string in the specified language.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_day_str;
///
/// assert_eq!(umt_get_day_str(0, "en"), "Sun");
/// assert_eq!(umt_get_day_str(3, "ja"), "水");
/// ```
#[inline]
pub fn umt_get_day_str(day: i32, lang: &str) -> &'static str {
    umt_get_day(day, DayLanguage::from_str(lang))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunday_in_different_languages() {
        assert_eq!(umt_get_day_str(0, "en"), "Sun");
        assert_eq!(umt_get_day_str(0, "ja"), "日");
        assert_eq!(umt_get_day_str(0, "ko"), "일");
        assert_eq!(umt_get_day_str(0, "de"), "So");
        assert_eq!(umt_get_day_str(0, "fr"), "Dim");
    }

    #[test]
    fn test_wednesday_in_different_languages() {
        assert_eq!(umt_get_day_str(3, "en"), "Wed");
        assert_eq!(umt_get_day_str(3, "ja"), "水");
        assert_eq!(umt_get_day_str(3, "ko"), "수");
        assert_eq!(umt_get_day_str(3, "de"), "Mi");
        assert_eq!(umt_get_day_str(3, "fr"), "Mer");
    }

    #[test]
    fn test_japanese_as_default() {
        assert_eq!(umt_get_day_default(0), "日");
        assert_eq!(umt_get_day_default(1), "月");
        assert_eq!(umt_get_day_default(2), "火");
        assert_eq!(umt_get_day_default(3), "水");
        assert_eq!(umt_get_day_default(4), "木");
        assert_eq!(umt_get_day_default(5), "金");
        assert_eq!(umt_get_day_default(6), "土");
    }

    #[test]
    fn test_invalid_day_numbers_return_sunday() {
        assert_eq!(umt_get_day_default(-1), "日");
        assert_eq!(umt_get_day_default(7), "日");
        assert_eq!(umt_get_day_default(100), "日");
    }

    #[test]
    fn test_all_days_in_english() {
        let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        for (index, day) in days.iter().enumerate() {
            assert_eq!(umt_get_day_str(index as i32, "en"), *day);
        }
    }

    #[test]
    fn test_day_language_from_str() {
        assert_eq!(DayLanguage::from_str("de"), DayLanguage::De);
        assert_eq!(DayLanguage::from_str("ko"), DayLanguage::Ko);
        assert_eq!(DayLanguage::from_str("en"), DayLanguage::En);
        assert_eq!(DayLanguage::from_str("ja"), DayLanguage::Ja);
        assert_eq!(DayLanguage::from_str("fr"), DayLanguage::Fr);
        assert_eq!(DayLanguage::from_str("unknown"), DayLanguage::Ja);
        assert_eq!(DayLanguage::from_str("EN"), DayLanguage::En); // Case insensitive
    }
}
