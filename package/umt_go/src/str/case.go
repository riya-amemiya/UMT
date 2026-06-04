package str

import (
	"regexp"
	"strings"
	"unicode"
)

// CamelCase converts a string to camelCase.
// It handles kebab-case, snake_case, space-separated words, PascalCase, and mixed separators.
func CamelCase(s string) string {
	// Replace non-alphanumeric characters followed by a character with the uppercase version
	reNonAlphaFollowed := regexp.MustCompile(`[^a-zA-Z0-9]+(.?)`)
	result := reNonAlphaFollowed.ReplaceAllStringFunc(s, func(match string) string {
		// Find the last character of the match (the captured group)
		sub := reNonAlphaFollowed.FindStringSubmatch(match)
		if len(sub) > 1 && sub[1] != "" {
			return strings.ToUpper(sub[1])
		}
		return ""
	})

	// Remove trailing non-alphanumeric characters
	reTrailing := regexp.MustCompile(`[^a-zA-Z0-9]+$`)
	result = reTrailing.ReplaceAllString(result, "")

	// Lowercase the first character
	if len(result) > 0 {
		runes := []rune(result)
		runes[0] = unicode.ToLower(runes[0])
		result = string(runes)
	}

	return result
}

// KebabCase converts a string to kebab-case.
// It handles camelCase, PascalCase, snake_case, space-separated words, and mixed separators.
func KebabCase(s string) string {
	// Insert dash between lowercase and uppercase
	re1 := regexp.MustCompile(`([a-z])([A-Z])`)
	result := re1.ReplaceAllString(s, "${1}-${2}")

	// Insert dash between sequences of uppercase letters and following lowercase
	re2 := regexp.MustCompile(`([A-Z])([A-Z][a-z])`)
	result = re2.ReplaceAllString(result, "${1}-${2}")

	// Replace spaces and underscores with dashes
	re3 := regexp.MustCompile(`[\s_]+`)
	result = re3.ReplaceAllString(result, "-")

	// Remove special characters except alphanumeric and dashes
	re4 := regexp.MustCompile(`[^a-zA-Z0-9-]`)
	result = re4.ReplaceAllString(result, "-")

	// Remove multiple consecutive dashes
	re5 := regexp.MustCompile(`-+`)
	result = re5.ReplaceAllString(result, "-")

	// Remove leading and trailing dashes
	re6 := regexp.MustCompile(`^-|-$`)
	result = re6.ReplaceAllString(result, "")

	return strings.ToLower(result)
}

// Slugify converts a string to a URL-friendly slug.
// It normalizes unicode characters, removes diacritics, and converts to lowercase
// with hyphen separators.
func Slugify(s string) string {
	// Normalize NFD and remove combining diacritical marks
	// We manually handle common diacritics by decomposing
	result := normalizeNFD(s)

	// Remove combining diacritical marks (U+0300 to U+036F)
	reDiacritics := regexp.MustCompile("[\u0300-\u036F]")
	result = reDiacritics.ReplaceAllString(result, "")

	// Lowercase
	result = strings.ToLower(result)

	// Replace non-word characters (not [a-zA-Z0-9_\s-]) with dash
	reNonWord := regexp.MustCompile(`[^\w\s-]`)
	result = reNonWord.ReplaceAllString(result, "-")

	// Replace whitespace with dashes
	reSpaces := regexp.MustCompile(`\s+`)
	result = reSpaces.ReplaceAllString(result, "-")

	// Replace underscores with dashes
	reUnderscores := regexp.MustCompile(`_+`)
	result = reUnderscores.ReplaceAllString(result, "-")

	// Remove multiple consecutive dashes
	reMultiDash := regexp.MustCompile(`-+`)
	result = reMultiDash.ReplaceAllString(result, "-")

	// Remove leading and trailing dashes
	reEdgeDash := regexp.MustCompile(`^-+|-+$`)
	result = reEdgeDash.ReplaceAllString(result, "")

	return result
}

// wordsBoundary1 inserts a boundary between a lowercase/number and an uppercase letter.
var wordsBoundary1 = regexp.MustCompile(`([\p{Ll}\p{N}])(\p{Lu})`)

// wordsBoundary2 inserts a boundary inside an acronym followed by a capitalized word.
var wordsBoundary2 = regexp.MustCompile(`(\p{Lu})(\p{Lu}\p{Ll})`)

// wordsSeparator splits on runs of non-letter, non-number characters.
var wordsSeparator = regexp.MustCompile(`[^\p{L}\p{N}]+`)

// Words splits a string into words on case boundaries and non-alphanumeric separators.
// It uses Unicode property classes so CJK and other letters are preserved.
//
// Example:
//
//	Words("helloWorld foo-bar") // []string{"hello", "World", "foo", "bar"}
//	Words("XMLHttpRequest")     // []string{"XML", "Http", "Request"}
func Words(s string) []string {
	withBoundaries := wordsBoundary1.ReplaceAllString(s, "$1 $2")
	withBoundaries = wordsBoundary2.ReplaceAllString(withBoundaries, "$1 $2")
	parts := wordsSeparator.Split(withBoundaries, -1)
	result := make([]string, 0, len(parts))
	for _, part := range parts {
		if part != "" {
			result = append(result, part)
		}
	}
	return result
}

// WordsWithPattern splits a string into words using a custom pattern.
// It returns every match of the pattern, mirroring JavaScript's String.match
// with a global regular expression.
//
// Example:
//
//	WordsWithPattern("a1 b2 c3", regexp.MustCompile(`[a-z]\d`)) // []string{"a1", "b2", "c3"}
func WordsWithPattern(s string, pattern *regexp.Regexp) []string {
	matches := pattern.FindAllString(s, -1)
	if matches == nil {
		return []string{}
	}
	return matches
}

// WordCount counts words in a string using the same boundaries as Words.
//
// Example:
//
//	WordCount("hello world")     // 2
//	WordCount("camelCase split") // 3
func WordCount(s string) int {
	return len(Words(s))
}

// Capitalize uppercases the first rune of a string. It is surrogate-pair safe
// and does not lowercase the rest of the string.
//
// Example:
//
//	Capitalize("hello")  // "Hello"
//	Capitalize("éclair") // "Éclair"
//	Capitalize("")       // ""
func Capitalize(s string) string {
	for _, first := range s {
		return string(unicode.ToUpper(first)) + s[len(string(first)):]
	}
	return s
}

// Uncapitalize lowercases the first rune of a string. It is surrogate-pair safe
// and does not change the rest of the string.
//
// Example:
//
//	Uncapitalize("Hello")  // "hello"
//	Uncapitalize("ÉCLAIR") // "éCLAIR"
//	Uncapitalize("")       // ""
func Uncapitalize(s string) string {
	for _, first := range s {
		return string(unicode.ToLower(first)) + s[len(string(first)):]
	}
	return s
}

// CapitalizeWord uppercases the first rune and lowercases the rest of a word.
//
// Example:
//
//	CapitalizeWord("hELLO") // "Hello"
//	CapitalizeWord("a")     // "A"
func CapitalizeWord(word string) string {
	runes := []rune(word)
	if len(runes) == 0 {
		return word
	}
	return string(unicode.ToUpper(runes[0])) + strings.ToLower(string(runes[1:]))
}

// PascalCase converts a string to PascalCase.
//
// Example:
//
//	PascalCase("hello-world") // "HelloWorld"
//	PascalCase("hello_world") // "HelloWorld"
func PascalCase(s string) string {
	w := Words(s)
	var b strings.Builder
	for _, word := range w {
		b.WriteString(CapitalizeWord(word))
	}
	return b.String()
}

// SnakeCase converts a string to snake_case.
//
// Example:
//
//	SnakeCase("helloWorld") // "hello_world"
//	SnakeCase("Hello World") // "hello_world"
func SnakeCase(s string) string {
	w := Words(s)
	for i, word := range w {
		w[i] = strings.ToLower(word)
	}
	return strings.Join(w, "_")
}

// ConstantCase converts a string to CONSTANT_CASE.
//
// Example:
//
//	ConstantCase("helloWorld")  // "HELLO_WORLD"
//	ConstantCase("Hello World") // "HELLO_WORLD"
func ConstantCase(s string) string {
	w := Words(s)
	for i, word := range w {
		w[i] = strings.ToUpper(word)
	}
	return strings.Join(w, "_")
}

// TitleCase converts a string to Title Case, capitalizing each word and
// separating words with single spaces.
//
// Example:
//
//	TitleCase("hello world")        // "Hello World"
//	TitleCase("a-quick brown_fox")  // "A Quick Brown Fox"
func TitleCase(s string) string {
	w := Words(s)
	for i, word := range w {
		w[i] = CapitalizeWord(word)
	}
	return strings.Join(w, " ")
}

// SwapCase swaps the case of each letter, turning uppercase letters into
// lowercase and vice versa. Characters without case are left unchanged.
//
// Example:
//
//	SwapCase("Hello World") // "hELLO wORLD"
func SwapCase(s string) string {
	var b strings.Builder
	b.Grow(len(s))
	for _, r := range s {
		if !unicode.IsLetter(r) {
			b.WriteRune(r)
			continue
		}
		if unicode.IsLower(r) {
			b.WriteRune(unicode.ToUpper(r))
		} else {
			b.WriteRune(unicode.ToLower(r))
		}
	}
	return b.String()
}

// normalizeNFD performs NFD normalization for common accented characters.
// This is a simplified approach that handles the most common Latin diacritics.
func normalizeNFD(s string) string {
	// Map of precomposed characters to their base + combining mark
	var b strings.Builder
	for _, r := range s {
		decomposed := decomposeRune(r)
		b.WriteString(decomposed)
	}
	return b.String()
}

// decomposeRune decomposes a single rune to its NFD form for common Latin characters.
func decomposeRune(r rune) string {
	// Common Latin accented characters decomposition
	decompositions := map[rune]string{
		'\u00C0': "A\u0300", // A with grave
		'\u00C1': "A\u0301", // A with acute
		'\u00C2': "A\u0302", // A with circumflex
		'\u00C3': "A\u0303", // A with tilde
		'\u00C4': "A\u0308", // A with diaeresis
		'\u00C5': "A\u030A", // A with ring above
		'\u00C7': "C\u0327", // C with cedilla
		'\u00C8': "E\u0300", // E with grave
		'\u00C9': "E\u0301", // E with acute
		'\u00CA': "E\u0302", // E with circumflex
		'\u00CB': "E\u0308", // E with diaeresis
		'\u00CC': "I\u0300", // I with grave
		'\u00CD': "I\u0301", // I with acute
		'\u00CE': "I\u0302", // I with circumflex
		'\u00CF': "I\u0308", // I with diaeresis
		'\u00D1': "N\u0303", // N with tilde
		'\u00D2': "O\u0300", // O with grave
		'\u00D3': "O\u0301", // O with acute
		'\u00D4': "O\u0302", // O with circumflex
		'\u00D5': "O\u0303", // O with tilde
		'\u00D6': "O\u0308", // O with diaeresis
		'\u00D9': "U\u0300", // U with grave
		'\u00DA': "U\u0301", // U with acute
		'\u00DB': "U\u0302", // U with circumflex
		'\u00DC': "U\u0308", // U with diaeresis
		'\u00DD': "Y\u0301", // Y with acute
		'\u00E0': "a\u0300", // a with grave
		'\u00E1': "a\u0301", // a with acute
		'\u00E2': "a\u0302", // a with circumflex
		'\u00E3': "a\u0303", // a with tilde
		'\u00E4': "a\u0308", // a with diaeresis
		'\u00E5': "a\u030A", // a with ring above
		'\u00E7': "c\u0327", // c with cedilla
		'\u00E8': "e\u0300", // e with grave
		'\u00E9': "e\u0301", // e with acute
		'\u00EA': "e\u0302", // e with circumflex
		'\u00EB': "e\u0308", // e with diaeresis
		'\u00EC': "i\u0300", // i with grave
		'\u00ED': "i\u0301", // i with acute
		'\u00EE': "i\u0302", // i with circumflex
		'\u00EF': "i\u0308", // i with diaeresis
		'\u00F1': "n\u0303", // n with tilde
		'\u00F2': "o\u0300", // o with grave
		'\u00F3': "o\u0301", // o with acute
		'\u00F4': "o\u0302", // o with circumflex
		'\u00F5': "o\u0303", // o with tilde
		'\u00F6': "o\u0308", // o with diaeresis
		'\u00F9': "u\u0300", // u with grave
		'\u00FA': "u\u0301", // u with acute
		'\u00FB': "u\u0302", // u with circumflex
		'\u00FC': "u\u0308", // u with diaeresis
		'\u00FD': "y\u0301", // y with acute
		'\u00FF': "y\u0308", // y with diaeresis
	}

	if d, ok := decompositions[r]; ok {
		return d
	}
	return string(r)
}
