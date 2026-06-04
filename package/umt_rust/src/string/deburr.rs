use unicode_normalization::UnicodeNormalization;

/// Removes diacritical marks from a string by decomposing characters and
/// stripping the combining marks (e.g. "é" becomes "e").
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
    s.nfd()
        .filter(|c| !('\u{0300}'..='\u{036F}').contains(c))
        .collect()
}
