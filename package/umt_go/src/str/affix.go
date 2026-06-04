package str

import "strings"

// EnsurePrefix ensures a string starts with the given prefix, prepending it
// only when it is not already present.
//
// Example:
//
//	EnsurePrefix("example.com", "https://")         // "https://example.com"
//	EnsurePrefix("https://example.com", "https://") // "https://example.com"
func EnsurePrefix(s string, prefix string) string {
	if strings.HasPrefix(s, prefix) {
		return s
	}
	return prefix + s
}

// EnsureSuffix ensures a string ends with the given suffix, appending it only
// when it is not already present.
//
// Example:
//
//	EnsureSuffix("file", ".txt")     // "file.txt"
//	EnsureSuffix("file.txt", ".txt") // "file.txt"
func EnsureSuffix(s string, suffix string) string {
	if strings.HasSuffix(s, suffix) {
		return s
	}
	return s + suffix
}

// RemovePrefix removes the given prefix from the start of a string once, if
// present. An empty prefix leaves the string unchanged.
//
// Example:
//
//	RemovePrefix("https://example.com", "https://") // "example.com"
//	RemovePrefix("example.com", "https://")         // "example.com"
func RemovePrefix(s string, prefix string) string {
	if prefix != "" && strings.HasPrefix(s, prefix) {
		return s[len(prefix):]
	}
	return s
}

// RemoveSuffix removes the given suffix from the end of a string once, if
// present. An empty suffix leaves the string unchanged.
//
// Example:
//
//	RemoveSuffix("file.txt", ".txt") // "file"
//	RemoveSuffix("file", ".txt")     // "file"
func RemoveSuffix(s string, suffix string) string {
	if suffix != "" && strings.HasSuffix(s, suffix) {
		return s[:len(s)-len(suffix)]
	}
	return s
}
