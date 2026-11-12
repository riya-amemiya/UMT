/// Wraps text to lines of at most `width` characters at word boundaries
///
/// Existing line breaks are kept and each line is wrapped on its own. Words are
/// separated by spaces, tabs, carriage returns, form feeds and vertical tabs, and
/// a word longer than `width` is placed on its own line without being broken.
///
/// # Arguments
/// * `s` - The text to wrap
/// * `width` - The maximum line width
///
/// # Returns
/// The wrapped text joined with `"\n"`
///
/// # Example
/// ```
/// use umt_rust::string::umt_wrap_text;
/// assert_eq!(umt_wrap_text("The quick brown fox", 10), "The quick\nbrown fox");
/// assert_eq!(umt_wrap_text("a extraordinary b", 5), "a\nextraordinary\nb");
/// ```
pub fn umt_wrap_text(s: &str, width: usize) -> String {
    s.split('\n')
        .map(|paragraph| {
            let mut lines: Vec<(String, usize)> = Vec::new();
            for word in paragraph
                .split([' ', '\t', '\r', '\x0C', '\x0B'])
                .filter(|word| !word.is_empty())
            {
                let word_length = word.chars().count();
                match lines.last_mut() {
                    Some((line, line_length)) if *line_length + 1 + word_length <= width => {
                        line.push(' ');
                        line.push_str(word);
                        *line_length += 1 + word_length;
                    }
                    _ => lines.push((word.to_string(), word_length)),
                }
            }
            lines
                .into_iter()
                .map(|(line, _)| line)
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
