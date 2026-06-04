package urlutil

import "regexp"

// absoluteUrlPattern mirrors the TypeScript regular expression
// /^[a-z][\d+.a-z-]*:/i — a scheme that starts with a letter and continues with
// letters, digits, "+", ".", or "-", immediately followed by a colon.
var absoluteUrlPattern = regexp.MustCompile(`^[a-zA-Z][0-9a-zA-Z+.-]*:`)

// IsAbsoluteUrl reports whether a URL string is absolute (RFC 3986).
//
// An absolute URL starts with a scheme followed by a colon, where the scheme
// begins with a letter and may contain letters, digits, plus, hyphen, or
// period. This matches the TypeScript isAbsoluteUrl exactly, so
// protocol-relative URLs ("//example.com") and schemes that start with a digit
// or symbol return false.
//
// Example:
//
//	IsAbsoluteUrl("https://example.com") // true
//	IsAbsoluteUrl("ftp://files.example") // true
//	IsAbsoluteUrl("mailto:user@host")    // true
//	IsAbsoluteUrl("/path/to/page")       // false
//	IsAbsoluteUrl("relative/path")       // false
func IsAbsoluteUrl(rawURL string) bool {
	return absoluteUrlPattern.MatchString(rawURL)
}
