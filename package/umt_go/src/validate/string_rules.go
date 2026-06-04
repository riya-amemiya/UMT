package validate

import (
	"math"
	"regexp"
	"strconv"
)

// ExactLength creates a string validation rule that passes only when the input
// string's length equals length. It mirrors the TypeScript `exactLength(length,
// message?)` validator (Validate/string/exactLength.ts), whose predicate is
// `value.length === length`, and the Rust `umt_exact_length` validator.
//
// As in the TypeScript/Rust originals the rule carries the supplied failure
// message, which the enclosing string validator surfaces when the predicate
// fails. The returned ValidateReturnType[string] is intended to be passed in the
// option slice of ValidateString / StringValidator.
//
// Length is measured in bytes (Go's len), matching the Rust port's
// `value.len()`; this coincides with the TypeScript `value.length` for ASCII
// input such as the cases covered by the reference tests.
func ExactLength(length int, message string) ValidateReturnType[string] {
	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool { return len(value) == length },
	}
}

// MinLength creates a string validation rule that passes only when the input
// string's length is greater than or equal to minLength. It mirrors the
// TypeScript `minLength(minLength, message?)` validator
// (Validate/string/minLength.ts), whose predicate is `value.length >=
// minLength`, and the Rust `umt_min_length` validator.
//
// The rule carries the supplied failure message and is intended to be passed in
// the option slice of ValidateString / StringValidator. Length is measured in
// bytes (Go's len), matching the Rust port.
func MinLength(minLength int, message string) ValidateReturnType[string] {
	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool { return len(value) >= minLength },
	}
}

// MaxLength creates a string validation rule that passes only when the input
// string's length is less than or equal to maxLength. It mirrors the TypeScript
// `maxLength(maxLength, message?)` validator (Validate/string/maxLength.ts),
// whose predicate is `value.length <= maxLength`, and the Rust `umt_max_length`
// validator.
//
// The rule carries the supplied failure message and is intended to be passed in
// the option slice of ValidateString / StringValidator. Length is measured in
// bytes (Go's len), matching the Rust port.
func MaxLength(maxLength int, message string) ValidateReturnType[string] {
	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool { return len(value) <= maxLength },
	}
}

// NumberString creates a string validation rule that passes only when the input
// string represents a finite number. It mirrors the TypeScript
// `numberString(message?)` validator (Validate/string/numberString.ts), whose
// predicate is `isNumber(value)` (loose mode, i.e. JavaScript `isFinite`), and
// the Rust `umt_number_string` validator.
//
// Following the JavaScript `isFinite` semantics that both the TypeScript and
// Rust originals reproduce, the empty string is considered valid (JavaScript
// coerces "" to 0), and any non-empty string is valid only when it parses as a
// finite floating-point number. Trailing garbage such as "123abc" is rejected,
// matching strconv.ParseFloat and the reference tests.
func NumberString(message string) ValidateReturnType[string] {
	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool {
			if value == "" {
				return true
			}
			num, err := strconv.ParseFloat(value, 64)
			if err != nil {
				return false
			}
			return !math.IsInf(num, 0) && !math.IsNaN(num)
		},
	}
}

// OneOf creates a top-level string validator that reports whether the input
// value is one of the given allowed literal values. It mirrors the TypeScript
// `oneOf(values, message?)` validator (Validate/string/oneOf.ts) and the Rust
// `umt_one_of` validator.
//
// Unlike the length/format helpers in this file, OneOf is a standalone
// validator (not a ValidateReturnType[string] rule): it returns a function from
// string to ValidateCoreReturnType[string], matching the TypeScript top-level
// validator shape. The match is exact and case sensitive. On success the
// returned message is empty; on failure it is the supplied message (or empty
// when none was given). The Type field carries the input value back so
// composing helpers (object/union/intersection) can recover the matched
// literal, mirroring the TypeScript `type` field and the Rust `type_value`
// field. An empty allowed list rejects every value.
func OneOf(values []string, message string) func(value string) ValidateCoreReturnType[string] {
	allowed := make(map[string]struct{}, len(values))
	for _, v := range values {
		allowed[v] = struct{}{}
	}
	return func(value string) ValidateCoreReturnType[string] {
		_, isValid := allowed[value]
		resultMessage := ""
		if !isValid {
			resultMessage = message
		}
		return ValidateCoreReturnType[string]{
			Validate: isValid,
			Message:  resultMessage,
			Type:     value,
		}
	}
}

// RegexMatch creates a string validation rule that passes only when the input
// string matches the supplied compiled regular expression. It mirrors the
// TypeScript `regexMatch(pattern, message?)` validator
// (Validate/string/regexMatch.ts), whose predicate is `pattern.test(value)`,
// and the Rust `umt_regex_match` validator.
//
// The rule carries the supplied failure message and is intended to be passed in
// the option slice of ValidateString / StringValidator. The match follows the
// supplied pattern's anchoring, so an unanchored pattern matches any substring,
// matching JavaScript `RegExp.prototype.test`.
func RegexMatch(pattern *regexp.Regexp, message string) ValidateReturnType[string] {
	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool { return pattern.MatchString(value) },
	}
}

// validUUIDVersions lists the UUID versions whose value may be interpolated into
// a regular expression. It mirrors the TypeScript VALID_UUID_VERSIONS guard,
// which restricts the interpolated version to a known single digit to prevent
// regex injection, and the Rust version filtering.
var validUUIDVersions = map[int]struct{}{
	1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {},
}

// Uuid creates a string validation rule that passes only when the input string
// is a valid UUID of one of the requested versions. It mirrors the TypeScript
// `uuid(versions=[4], message?)` validator (Validate/string/uuid.ts) and the
// Rust `umt_uuid` validator.
//
// Versions outside the safe range 1..=7 are filtered out before being
// interpolated into the per-version pattern, matching the TypeScript regex
// injection guard. When no version remains after filtering (including a
// supplied empty slice), the rule rejects every value. A nil versions slice
// defaults to []int{4}, matching the TypeScript default parameter and the Rust
// `unwrap_or_else(|| vec![4])`.
//
// Each version's pattern is case insensitive and accepts both hyphenated and
// unhyphenated UUID forms, mirroring the shared TypeScript/Rust regex
// `^[\da-f]{8}-?[\da-f]{4}-?<version>[\da-f]{3}-?[89ab][\da-f]{3}-?[\da-f]{12}$`.
// The rule carries the supplied failure message and is intended to be passed in
// the option slice of ValidateString / StringValidator.
func Uuid(versions []int, message string) ValidateReturnType[string] {
	if versions == nil {
		versions = []int{4}
	}
	safeVersions := make([]int, 0, len(versions))
	for _, v := range versions {
		if _, ok := validUUIDVersions[v]; ok {
			safeVersions = append(safeVersions, v)
		}
	}
	if len(safeVersions) == 0 {
		return ValidateReturnType[string]{
			TypeName: "string",
			Message:  message,
			Validate: func(value string) bool { return false },
		}
	}

	patterns := make([]*regexp.Regexp, 0, len(safeVersions))
	for _, version := range safeVersions {
		pattern := `(?i)^[\da-f]{8}-?[\da-f]{4}-?` + strconv.Itoa(version) +
			`[\da-f]{3}-?[89ab][\da-f]{3}-?[\da-f]{12}$`
		patterns = append(patterns, regexp.MustCompile(pattern))
	}

	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool {
			for _, pattern := range patterns {
				if pattern.MatchString(value) {
					return true
				}
			}
			return false
		},
	}
}

// ValidateEmailRule creates a string validation rule that passes only when the
// input string is a valid email address at the given RFC compliance level. It
// mirrors the TypeScript `validateEmail(message?, options?)` validator
// (Validate/string/validateEmail.ts), whose predicate is
// `parseEmail({ email: value, options }).valid`, and the Rust
// `umt_validate_email_validator`.
//
// The name diverges from the TypeScript `validateEmail` to avoid clashing with
// the existing ParseEmail family in validate.go. An empty level string defaults
// to "basic", matching the TypeScript default options `{ level: "basic" }` and
// the Rust default ParseEmailLevel::Basic. Validation delegates to
// ParseEmailWithLevel so that the supported levels stay identical to the rest of
// this package. The rule carries the supplied failure message and is intended
// to be passed in the option slice of ValidateString / StringValidator.
func ValidateEmailRule(message string, level string) ValidateReturnType[string] {
	if level == "" {
		level = "basic"
	}
	return ValidateReturnType[string]{
		TypeName: "string",
		Message:  message,
		Validate: func(value string) bool {
			return ParseEmailWithLevel(value, level).Valid
		},
	}
}
