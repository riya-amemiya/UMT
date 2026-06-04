package str

import (
	"regexp"
	"strings"
	"unicode"
)

// DeleteSpaces removes all whitespace characters from a string.
// This handles all Unicode whitespace characters including em space, full-width space, etc.
func DeleteSpaces(s string) string {
	var b strings.Builder
	b.Grow(len(s))
	for _, r := range s {
		if !unicode.IsSpace(r) {
			b.WriteRune(r)
		}
	}
	return b.String()
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

// whitespaceRun matches one or more whitespace characters.
var whitespaceRun = regexp.MustCompile(`\s+`)

// NormalizeWhitespace collapses consecutive whitespace characters into a single
// space and trims the result. Unlike DeleteSpaces, which removes all whitespace,
// this keeps words separated by single spaces.
//
// Example:
//
//	NormalizeWhitespace("  hello   world \t\n foo ") // "hello world foo"
func NormalizeWhitespace(s string) string {
	return strings.TrimSpace(whitespaceRun.ReplaceAllString(s, " "))
}

// nonPrintable matches characters outside the printable ASCII range while
// keeping TAB, LF, CR, and the space-to-tilde range.
var nonPrintable = regexp.MustCompile("[^\t\n\r -~]")

// SanitizeString removes non-printable characters from a string. It keeps
// printable ASCII characters (space to tilde), newlines, carriage returns, and
// tabs, which is useful for cleaning up output from external commands.
//
// Example:
//
//	SanitizeString("a\x00b") // "ab"
func SanitizeString(s string) string {
	return nonPrintable.ReplaceAllString(s, "")
}

// ansiPattern matches ANSI escape sequences, covering CSI sequences (colors,
// cursor movement) and OSC sequences. It mirrors the JavaScript regex used by
// the TypeScript reference, expressed with explicit hex escapes for the ESC,
// CSI, and BEL control characters.
var ansiPattern = regexp.MustCompile(`(?:\x1b\[|\x{009b})[0-?]*[ -/]*[@-~]|\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)`)

// StripAnsi removes ANSI escape sequences (e.g. terminal color codes) from a
// string, keeping the visible text. Unlike SanitizeString, which strips all
// non-printable characters, this targets only ANSI sequences.
//
// Example:
//
//	StripAnsi("\x1b[31mred\x1b[0m") // "red"
func StripAnsi(s string) string {
	return ansiPattern.ReplaceAllString(s, "")
}

// htmlTag matches a single HTML/XML tag.
var htmlTag = regexp.MustCompile(`<[^<>]*>`)

// StripTags removes HTML/XML tags from a string, keeping the text content.
// The removal repeats until the string is stable so that nested constructs
// (e.g. "<sc<script>ript>") cannot survive a single pass.
//
// Example:
//
//	StripTags("<p>Hello <b>World</b></p>") // "Hello World"
func StripTags(s string) string {
	result := s
	for {
		previous := result
		result = htmlTag.ReplaceAllString(result, "")
		if result == previous {
			break
		}
	}
	return result
}

// combiningMarks matches combining diacritical marks (U+0300 to U+036F).
var combiningMarks = regexp.MustCompile(`[\x{0300}-\x{036F}]`)

// Deburr removes diacritical marks from a string by decomposing characters and
// stripping the combining marks (e.g. "é" becomes "e").
//
// Example:
//
//	Deburr("déjà vu")      // "deja vu"
//	Deburr("Crème brûlée") // "Creme brulee"
func Deburr(s string) string {
	return combiningMarks.ReplaceAllString(normalizeNFD(s), "")
}

// CountOccurrences counts the number of non-overlapping occurrences of a
// substring within a string. It returns 0 when the search string is empty.
//
// Example:
//
//	CountOccurrences("ababab", "ab") // 3
//	CountOccurrences("aaaa", "aa")   // 2
func CountOccurrences(s string, search string) int {
	if search == "" {
		return 0
	}
	return strings.Count(s, search)
}

// SplitByLength splits a string into chunks of the specified length. It is
// rune-aware, so multibyte characters are not split.
//
// Example:
//
//	SplitByLength("abcdefg", 3) // []string{"abc", "def", "g"}
func SplitByLength(s string, length int) []string {
	chars := []rune(s)
	result := []string{}
	for index := 0; index < len(chars); index += length {
		end := index + length
		if end > len(chars) {
			end = len(chars)
		}
		result = append(result, string(chars[index:end]))
	}
	return result
}

// MaskOptions configures Mask. Zero values fall back to the defaults of
// Start=1, End=1, and Char="*".
type MaskOptions struct {
	// Start is the number of leading characters to keep visible. Default 1.
	Start int
	// End is the number of trailing characters to keep visible. Default 1.
	End int
	// Char is the mask character. Default "*".
	Char string
}

// Mask masks the middle portion of a string with a fill character, preserving
// the leading and trailing visible counts. It is rune-aware, so multibyte
// characters count as single graphemes. Calling Mask with the zero MaskOptions
// uses the defaults Start=1, End=1, Char="*".
//
// Example:
//
//	Mask("secret", MaskOptions{})                     // "s****t"
//	Mask("1234567890", MaskOptions{Start: 2, End: 4}) // "12****7890"
func Mask(s string, options MaskOptions) string {
	start := options.Start
	if start == 0 {
		start = 1
	}
	end := options.End
	if end == 0 {
		end = 1
	}
	char := options.Char
	if char == "" {
		char = "*"
	}

	graphemes := []rune(s)
	length := len(graphemes)
	if start+end >= length {
		return s
	}
	middleLength := length - start - end
	return string(graphemes[:start]) + strings.Repeat(char, middleLength) + string(graphemes[length-end:])
}

// Dedent removes the minimum common leading whitespace from each line of the
// input.
//
// Example:
//
//	Dedent("    line1\n      line2\n    line3") // "line1\n  line2\nline3"
func Dedent(s string) string {
	lines := strings.Split(s, "\n")
	minIndent := -1
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}
		indent := 0
		for indent < len(line) && (line[indent] == ' ' || line[indent] == '\t') {
			indent++
		}
		if minIndent == -1 || indent < minIndent {
			minIndent = indent
		}
	}
	if minIndent == -1 {
		return s
	}
	for i, line := range lines {
		if minIndent <= len(line) {
			lines[i] = line[minIndent:]
		} else {
			lines[i] = ""
		}
	}
	return strings.Join(lines, "\n")
}
