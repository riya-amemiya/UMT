/// Removes the minimum common leading whitespace from each line of the input.
///
/// Blank lines (lines that are empty after trimming) are ignored when computing
/// the minimum indentation. Only spaces and tabs are considered indentation. If
/// no line has any indentation the input is returned unchanged.
///
/// The TypeScript original also works as a tagged template literal; Rust has no
/// equivalent, so this port covers the plain-string behavior.
///
/// # Arguments
/// * `s` - The string to dedent
///
/// # Returns
/// The dedented string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_dedent;
/// assert_eq!(
///     umt_dedent("    line1\n      line2\n    line3"),
///     "line1\n  line2\nline3"
/// );
/// assert_eq!(umt_dedent("no indent"), "no indent");
/// ```
pub fn umt_dedent(s: &str) -> String {
    let lines: Vec<&str> = s.split('\n').collect();

    let mut min_indent: Option<usize> = None;
    for line in &lines {
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.chars().take_while(|&c| c == ' ' || c == '\t').count();
        min_indent = Some(match min_indent {
            Some(current) if current <= indent => current,
            _ => indent,
        });
    }

    let Some(min_indent) = min_indent else {
        return s.to_string();
    };

    lines
        .iter()
        .map(|line| line.chars().skip(min_indent).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
