/// Converts a number to its English ordinal string representation.
///
/// Handles the special cases for 11th, 12th, and 13th
/// (which end in "th" despite their last digit).
///
/// # Arguments
///
/// * `value` - The number to convert to ordinal form
///
/// # Returns
///
/// The ordinal string (e.g., "1st", "2nd", "3rd", "11th")
///
/// # Example
///
/// ```
/// use umt_rust::number::umt_to_ordinal;
/// assert_eq!(umt_to_ordinal(1.0), "1st");
/// assert_eq!(umt_to_ordinal(2.0), "2nd");
/// assert_eq!(umt_to_ordinal(3.0), "3rd");
/// assert_eq!(umt_to_ordinal(11.0), "11th");
/// assert_eq!(umt_to_ordinal(21.0), "21st");
/// assert_eq!(umt_to_ordinal(112.0), "112th");
/// ```
pub fn umt_to_ordinal(value: f64) -> String {
    let module100 = value % 100.0;
    if (11.0..=13.0).contains(&module100) {
        return format!("{}th", value);
    }

    let module10 = value % 10.0;
    if module10 == 1.0 {
        return format!("{}st", value);
    }
    if module10 == 2.0 {
        return format!("{}nd", value);
    }
    if module10 == 3.0 {
        return format!("{}rd", value);
    }

    format!("{}th", value)
}
