package str

import (
	"regexp"
	"unicode"
)

// DeleteSpaces removes all whitespace characters from a string.
func DeleteSpaces(s string) string {
	re := regexp.MustCompile(`\s`)
	return re.ReplaceAllString(s, "")
}

// ReverseString reverses a string, operating on runes for proper Unicode support.
func ReverseString(s string) string {
	runes := []rune(s)
	n := len(runes)
	for i := 0; i < n/2; i++ {
		runes[i], runes[n-1-i] = runes[n-1-i], runes[i]
	}
	return string(runes)
}

// Truncate truncates a string to a specified length and appends a suffix.
// If length is negative, it panics. If the string is shorter than or equal to
// the length, the original string is returned unchanged.
func Truncate(s string, length int, suffix string) string {
	if length < 0 {
		panic("Length must be non-negative")
	}

	if len(s) <= length {
		return s
	}

	return s[:length] + suffix
}

// HasNoLetters checks if the string contains no Unicode letter characters.
// Returns true if the string has no letters (only digits, symbols, whitespace, emojis, etc.).
func HasNoLetters(s string) bool {
	for _, r := range s {
		if unicode.IsLetter(r) {
			return false
		}
	}
	return true
}
