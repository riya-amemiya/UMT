use regex::Regex;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

static DIACRITIC_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\p{Diacritic}").unwrap());

/// Removes diacritical marks from a string by decomposing characters and
/// stripping every character with the Unicode `Diacritic` property
/// (e.g. "é" becomes "e").
///
/// # Arguments
/// * `s` - The string to deburr
///
/// # Returns
/// The string without diacritical marks
///
/// # Examples
/// ```
/// use umt_rust::string::umt_deburr;
/// assert_eq!(umt_deburr("déjà vu"), "deja vu");
/// assert_eq!(umt_deburr("Crème brûlée"), "Creme brulee");
/// ```
#[inline]
pub fn umt_deburr(s: &str) -> String {
    let decomposed: String = s.nfd().collect();
    DIACRITIC_RE.replace_all(&decomposed, "").into_owned()
}
