use regex::Regex;

/// Removes ANSI escape sequences (e.g. terminal color codes) from a string,
/// keeping the visible text.
///
/// Handles CSI sequences (colors, cursor movement) and OSC sequences. Unlike a
/// general sanitizer that strips all non-printable characters, this targets only
/// ANSI sequences.
///
/// # Arguments
/// * `s` - The string to strip
///
/// # Returns
/// The string without ANSI escape sequences
///
/// # Examples
/// ```
/// use umt_rust::string::umt_strip_ansi;
/// assert_eq!(umt_strip_ansi("\u{001B}[31mred\u{001B}[0m"), "red");
/// ```
#[inline]
pub fn umt_strip_ansi(s: &str) -> String {
    let ansi = Regex::new(
        "(?:\u{001B}\\[|\u{009B})[0-?]*[ -/]*[@-~]|\u{001B}\\][^\u{0007}\u{001B}]*(?:\u{0007}|\u{001B}\\\\)",
    )
    .unwrap();
    ansi.replace_all(s, "").to_string()
}
