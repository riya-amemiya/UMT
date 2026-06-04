package validate

import (
	"regexp"
	"strings"
)

// TemplateLiteralPartKind identifies the kind of a single template literal part,
// mirroring the Rust `TemplateLiteralPart` enum (validate/template_literal.rs).
// The TypeScript original (Validate/templateLiteral/core.ts) detects the tag at
// runtime by invoking each primitive validator; the Go port follows the Rust
// design and asks the caller to name the part kind directly.
type TemplateLiteralPartKind int

const (
	// TemplateLiteralLiteral is a string fragment that must appear verbatim.
	TemplateLiteralLiteral TemplateLiteralPartKind = iota
	// TemplateLiteralStringTag contributes a non-greedy any-character fragment.
	TemplateLiteralStringTag
	// TemplateLiteralNumberTag contributes a signed integer or decimal fragment.
	TemplateLiteralNumberTag
	// TemplateLiteralBooleanTag contributes a true/false fragment.
	TemplateLiteralBooleanTag
	// TemplateLiteralBigIntTag contributes a signed integer fragment.
	TemplateLiteralBigIntTag
	// TemplateLiteralUnknownTag contributes a permissive non-empty fragment.
	TemplateLiteralUnknownTag
)

// TemplateLiteralPart is a single part of a template literal definition. For a
// literal part Kind is TemplateLiteralLiteral and Literal holds the verbatim
// fragment; for a tag part Kind names the primitive tag and Literal is ignored.
// This mirrors the Rust `TemplateLiteralPart` enum.
type TemplateLiteralPart struct {
	Kind    TemplateLiteralPartKind
	Literal string
}

// templateLiteralEscapeSet matches the character class escaped by the TypeScript
// `escapeRegex` helper ([$()*+.?[\\\]^{|}]) and the Rust `escape_regex`.
var templateLiteralEscapeSet = map[rune]struct{}{
	'$': {}, '(': {}, ')': {}, '*': {}, '+': {}, '.': {}, '?': {},
	'[': {}, '\\': {}, ']': {}, '^': {}, '{': {}, '|': {}, '}': {},
}

func templateLiteralEscape(input string) string {
	var b strings.Builder
	b.Grow(len(input))
	for _, ch := range input {
		if _, ok := templateLiteralEscapeSet[ch]; ok {
			b.WriteByte('\\')
		}
		b.WriteRune(ch)
	}
	return b.String()
}

func templateLiteralTagPattern(kind TemplateLiteralPartKind) string {
	switch kind {
	case TemplateLiteralStringTag:
		return ".*?"
	case TemplateLiteralNumberTag:
		return `-?(?:\d+\.\d+|\d+(?:\.\d*)?|\.\d+)`
	case TemplateLiteralBooleanTag:
		return "(?:true|false)"
	case TemplateLiteralBigIntTag:
		return `-?\d+`
	case TemplateLiteralUnknownTag:
		return ".+?"
	default:
		return ""
	}
}

// TemplateLiteral creates a validator that checks whether a string matches a
// template literal pattern assembled from the supplied parts. It mirrors the
// TypeScript `templateLiteral(parts, message?)` factory (Validate/templateLiteral/core.ts)
// and the Rust `umt_template_literal` function.
//
// Each part is either a verbatim literal (escaped into the pattern) or a
// primitive tag that contributes a fixed regex fragment. The assembled pattern
// is anchored at both ends. A value matching the pattern yields a successful
// result with an empty message; otherwise the result fails carrying the supplied
// message.
func TemplateLiteral(parts []TemplateLiteralPart, message string) func(value string) ValidateCoreReturnType[string] {
	var pattern strings.Builder
	pattern.WriteByte('^')
	for _, part := range parts {
		if part.Kind == TemplateLiteralLiteral {
			pattern.WriteString(templateLiteralEscape(part.Literal))
		} else {
			pattern.WriteString("(?:")
			pattern.WriteString(templateLiteralTagPattern(part.Kind))
			pattern.WriteByte(')')
		}
	}
	pattern.WriteByte('$')

	// The assembled pattern contains only escaped literals and fixed fragments,
	// so compilation cannot fail; fall back to a never-matching regex anyway,
	// mirroring the Rust port's defensive fallback.
	regex, err := regexp.Compile(pattern.String())
	if err != nil {
		regex = regexp.MustCompile(`$.^`)
	}

	return func(value string) ValidateCoreReturnType[string] {
		if regex.MatchString(value) {
			return ValidateCoreReturnType[string]{
				Validate: true,
				Message:  "",
				Type:     value,
			}
		}
		return ValidateCoreReturnType[string]{
			Validate: false,
			Message:  message,
			Type:     value,
		}
	}
}
