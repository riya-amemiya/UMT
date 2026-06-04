/// Options controlling how [`umt_mask`] hides the middle of a string.
///
/// `None` fields fall back to the defaults: `start = 1`, `end = 1`, `char = "*"`.
#[derive(Default, Clone)]
pub struct MaskOptions {
    /// Number of leading characters to keep visible. Default: 1
    pub start: Option<usize>,
    /// Number of trailing characters to keep visible. Default: 1
    pub end: Option<usize>,
    /// Mask string used to fill the middle. Default: "*"
    pub char: Option<String>,
}

/// Masks the middle portion of a string with a fill character, preserving the
/// leading and trailing visible counts.
///
/// Operates on Unicode scalar values, so characters outside the Basic
/// Multilingual Plane (e.g. emoji) are treated as a single unit. When the sum of
/// the visible counts is greater than or equal to the length, the input is
/// returned unchanged.
///
/// # Arguments
/// * `s` - The string to mask
/// * `options` - Visible-character counts and the mask string
///
/// # Returns
/// The masked string
///
/// # Examples
/// ```
/// use umt_rust::string::{umt_mask, MaskOptions};
/// assert_eq!(umt_mask("secret", &MaskOptions::default()), "s****t");
/// assert_eq!(
///     umt_mask("1234567890", &MaskOptions { start: Some(2), end: Some(4), char: None }),
///     "12****7890"
/// );
/// ```
pub fn umt_mask(s: &str, options: &MaskOptions) -> String {
    let start = options.start.unwrap_or(1);
    let end = options.end.unwrap_or(1);
    let mask_char = options.char.as_deref().unwrap_or("*");

    let graphemes: Vec<char> = s.chars().collect();
    let length = graphemes.len();
    if start + end >= length {
        return s.to_string();
    }

    let middle_length = length - start - end;
    let head: String = graphemes[..start].iter().collect();
    let tail: String = graphemes[length - end..].iter().collect();

    format!("{head}{}{tail}", mask_char.repeat(middle_length))
}
