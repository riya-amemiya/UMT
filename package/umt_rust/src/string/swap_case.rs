use regex::{Captures, Regex};
use std::sync::LazyLock;

static LETTER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\p{L}").unwrap());

/// Swaps the case of each letter in a string.
///
/// Uppercase letters become lowercase and vice versa. Characters outside the
/// Unicode general category `L` (digits, punctuation, letter-like numerals,
/// etc.) are left unchanged.
///
/// # Arguments
/// * `s` - The string to transform
///
/// # Returns
/// The string with swapped case
///
/// # Examples
/// ```
/// use umt_rust::string::umt_swap_case;
/// assert_eq!(umt_swap_case("Hello World"), "hELLO wORLD");
/// assert_eq!(umt_swap_case("abc123-日本"), "ABC123-日本");
/// ```
pub fn umt_swap_case(s: &str) -> String {
    LETTER_RE
        .replace_all(s, |caps: &Captures| {
            let letter = &caps[0];
            let lower = letter.to_lowercase();
            if lower == letter {
                letter.to_uppercase()
            } else {
                lower
            }
        })
        .into_owned()
}
