use regex::Regex;

/// Splits a string into words on case boundaries and non-alphanumeric separators.
///
/// Uses Unicode property escapes so CJK and other letters are preserved. When a
/// custom pattern is supplied, the matches of that pattern are returned directly.
///
/// # Arguments
/// * `s` - The string to split
/// * `pattern` - Optional custom matching pattern; defaults to a Unicode word matcher
///
/// # Returns
/// A vector of words
///
/// # Examples
/// ```
/// use umt_rust::string::umt_words;
/// assert_eq!(umt_words("helloWorld", None), vec!["hello", "World"]);
/// assert_eq!(umt_words("XMLHttpRequest", None), vec!["XML", "Http", "Request"]);
/// ```
pub fn umt_words(s: &str, pattern: Option<&Regex>) -> Vec<String> {
    if let Some(re) = pattern {
        return re.find_iter(s).map(|m| m.as_str().to_string()).collect();
    }

    // Insert a space between a lowercase/number and an uppercase letter,
    // then between an uppercase letter and an uppercase-followed-by-lowercase.
    let lower_upper = Regex::new(r"([\p{Ll}\p{N}])(\p{Lu})").unwrap();
    let upper_upper_lower = Regex::new(r"(\p{Lu})(\p{Lu}\p{Ll})").unwrap();
    let separators = Regex::new(r"[^\p{L}\p{N}]+").unwrap();

    let step1 = lower_upper.replace_all(s, "$1 $2");
    let with_boundaries = upper_upper_lower.replace_all(&step1, "$1 $2");

    separators
        .split(&with_boundaries)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}
