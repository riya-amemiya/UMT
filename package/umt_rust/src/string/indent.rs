/// Prefixes every non-empty line with `count` repetitions of `unit`
///
/// Empty lines are left untouched, which makes this the inverse of `umt_dedent`.
///
/// # Arguments
/// * `s` - The string to indent
/// * `count` - The number of times `unit` is repeated per line
/// * `unit` - The indentation unit (default: " ")
///
/// # Returns
/// The indented string
///
/// # Example
/// ```
/// use umt_rust::string::umt_indent;
/// assert_eq!(umt_indent("line1\nline2", 2, " "), "  line1\n  line2");
/// assert_eq!(umt_indent("a\n\nb", 1, "\t"), "\ta\n\n\tb");
/// ```
pub fn umt_indent(s: &str, count: usize, unit: &str) -> String {
    let prefix = unit.repeat(count);
    s.split('\n')
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{prefix}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Prefixes every non-empty line with `count` spaces
pub fn umt_indent_default(s: &str, count: usize) -> String {
    umt_indent(s, count, " ")
}
