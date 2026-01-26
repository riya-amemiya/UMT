//! String validation module for regular expression matching
//!
//! Provides validation functionality for checking if a string matches a regular expression pattern.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a string matches a pattern
///
/// # Arguments
///
/// * `pattern` - A closure that takes a string and returns true if it matches
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for pattern matching
pub fn umt_regex_match<F>(pattern: F, message: Option<&str>) -> ValidateReturnType<String>
where
    F: Fn(&str) -> bool + 'static,
{
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| pattern(value),
        message.map(String::from),
    )
}

/// Creates a validator for checking if a string matches a pattern string
///
/// # Arguments
///
/// * `pattern_str` - A pattern string. Supported patterns:
///   - `^...$` anchored patterns
///   - `[a-z]`, `[A-Z]`, `[0-9]` character classes
///   - `\d` (digit), `\w` (word char), `\s` (whitespace)
///   - `+` (one or more), `*` (zero or more), `?` (optional)
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for pattern matching
/// Returns a validator that always fails if the pattern is not supported
pub fn umt_regex_match_str(pattern_str: &str, message: Option<&str>) -> ValidateReturnType<String> {
    let pattern = pattern_str.to_string();
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| simple_pattern_match(&pattern, value),
        message.map(String::from),
    )
}

/// Simple pattern matching for basic regex-like patterns.
/// Supports: ^, $, [a-z], [A-Z], [0-9], [\da-f], \d, \w, \s, +, *, ?
fn simple_pattern_match(pattern: &str, value: &str) -> bool {
    let pat = pattern.as_bytes();
    let val: Vec<char> = value.chars().collect();

    let (pat, anchored_start) = if pat.first() == Some(&b'^') {
        (&pat[1..], true)
    } else {
        (pat, false)
    };

    let (pat, anchored_end) = if pat.last() == Some(&b'$') {
        (&pat[..pat.len() - 1], true)
    } else {
        (pat, false)
    };

    if anchored_start && anchored_end {
        match_here(pat, &val, 0).map_or(false, |end| end == val.len())
    } else if anchored_start {
        match_here(pat, &val, 0).is_some()
    } else if anchored_end {
        for start in 0..=val.len() {
            if let Some(end) = match_here(pat, &val, start) {
                if end == val.len() {
                    return true;
                }
            }
        }
        false
    } else {
        for start in 0..=val.len() {
            if match_here(pat, &val, start).is_some() {
                return true;
            }
        }
        false
    }
}

/// Try to match pattern starting at val[pos]. Returns end position if matched.
fn match_here(pat: &[u8], val: &[char], pos: usize) -> Option<usize> {
    if pat.is_empty() {
        return Some(pos);
    }

    let (class, consumed) = parse_class(pat)?;
    let rest = &pat[consumed..];

    match rest.first() {
        Some(&b'+') => {
            let rest = &rest[1..];
            let mut p = pos;
            let mut count = 0;
            while p < val.len() && class_matches(&class, val[p]) {
                p += 1;
                count += 1;
            }
            if count == 0 {
                return None;
            }
            while p >= pos + 1 {
                if let Some(end) = match_here(rest, val, p) {
                    return Some(end);
                }
                p -= 1;
            }
            None
        }
        Some(&b'*') => {
            let rest = &rest[1..];
            let mut p = pos;
            while p < val.len() && class_matches(&class, val[p]) {
                p += 1;
            }
            while p >= pos {
                if let Some(end) = match_here(rest, val, p) {
                    return Some(end);
                }
                if p == pos {
                    break;
                }
                p -= 1;
            }
            None
        }
        Some(&b'?') => {
            let rest = &rest[1..];
            if pos < val.len() && class_matches(&class, val[pos]) {
                if let Some(end) = match_here(rest, val, pos + 1) {
                    return Some(end);
                }
            }
            match_here(rest, val, pos)
        }
        _ => {
            if pos < val.len() && class_matches(&class, val[pos]) {
                match_here(rest, val, pos + 1)
            } else {
                None
            }
        }
    }
}

enum CharClass {
    Literal(u8),
    Dot,
    Digit,
    Word,
    Whitespace,
    Range(Vec<(u8, u8)>),
}

fn class_matches(class: &CharClass, c: char) -> bool {
    if !c.is_ascii() {
        return matches!(class, CharClass::Dot);
    }
    let b = c as u8;
    match class {
        CharClass::Literal(l) => b == *l,
        CharClass::Dot => true,
        CharClass::Digit => b.is_ascii_digit(),
        CharClass::Word => b.is_ascii_alphanumeric() || b == b'_',
        CharClass::Whitespace => b.is_ascii_whitespace(),
        CharClass::Range(ranges) => ranges.iter().any(|&(lo, hi)| b >= lo && b <= hi),
    }
}

/// Parse one character class or literal from the pattern. Returns (class, bytes consumed).
fn parse_class(pat: &[u8]) -> Option<(CharClass, usize)> {
    match pat.first()? {
        b'.' => Some((CharClass::Dot, 1)),
        b'\\' => {
            let next = pat.get(1)?;
            let class = match next {
                b'd' => CharClass::Digit,
                b'w' => CharClass::Word,
                b's' => CharClass::Whitespace,
                other => CharClass::Literal(*other),
            };
            Some((class, 2))
        }
        b'[' => {
            let mut ranges = Vec::new();
            let mut i = 1;
            while i < pat.len() && pat[i] != b']' {
                if pat[i] == b'\\' && i + 1 < pat.len() {
                    let c = pat[i + 1];
                    match c {
                        b'd' => {
                            ranges.push((b'0', b'9'));
                            i += 2;
                        }
                        b'w' => {
                            ranges.push((b'a', b'z'));
                            ranges.push((b'A', b'Z'));
                            ranges.push((b'0', b'9'));
                            ranges.push((b'_', b'_'));
                            i += 2;
                        }
                        _ => {
                            ranges.push((c, c));
                            i += 2;
                        }
                    }
                } else if i + 2 < pat.len() && pat[i + 1] == b'-' && pat[i + 2] != b']' {
                    ranges.push((pat[i], pat[i + 2]));
                    i += 3;
                } else {
                    ranges.push((pat[i], pat[i]));
                    i += 1;
                }
            }
            if i < pat.len() && pat[i] == b']' {
                Some((CharClass::Range(ranges), i + 1))
            } else {
                None
            }
        }
        b => Some((CharClass::Literal(*b), 1)),
    }
}
