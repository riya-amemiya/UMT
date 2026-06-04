package tool

import "strings"

// regExpSpecialChars is the set of characters escaped by EscapeRegExp. It mirrors
// the TypeScript character class /[$()*+.?[\\\]^{|}]/ exactly.
const regExpSpecialChars = "$()*+.?[\\]^{|}"

// EscapeRegExp escapes characters in a string so the result can be used literally
// inside a regular expression. Each regex metacharacter is prefixed with a
// backslash. It mirrors the TypeScript escapeRegExp utility, which escapes the
// character class $ ( ) * + . ? [ \ ] ^ { | }.
//
// Example:
//
//	EscapeRegExp("a.b")   // "a\\.b"
//	EscapeRegExp("a.b+c") // "a\\.b\\+c"
//	EscapeRegExp("")      // ""
func EscapeRegExp(value string) string {
	var builder strings.Builder
	builder.Grow(len(value))
	for _, r := range value {
		if r <= 0x7F && strings.IndexByte(regExpSpecialChars, byte(r)) != -1 {
			builder.WriteByte('\\')
		}
		builder.WriteRune(r)
	}
	return builder.String()
}
