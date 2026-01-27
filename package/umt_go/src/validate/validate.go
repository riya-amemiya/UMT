package validate

import (
	"fmt"
	"math"
	"reflect"
	"regexp"
	"strings"
)

// IsNumber checks if a value is a numeric type (int, float, etc.).
func IsNumber(v any) bool {
	if v == nil {
		return false
	}
	switch v.(type) {
	case int, int8, int16, int32, int64,
		uint, uint8, uint16, uint32, uint64,
		float32, float64:
		return true
	default:
		return false
	}
}

// IsString checks if a value is a string type.
func IsString(v any) bool {
	if v == nil {
		return false
	}
	_, ok := v.(string)
	return ok
}

// IsArray checks if a value is a slice or array type using reflect.
func IsArray(v any) bool {
	if v == nil {
		return false
	}
	kind := reflect.TypeOf(v).Kind()
	return kind == reflect.Slice || kind == reflect.Array
}

// IsDouble checks if a float64 value has a decimal part (is not an integer).
func IsDouble(v float64) bool {
	if math.IsNaN(v) || math.IsInf(v, 0) {
		return false
	}
	return v != math.Trunc(v)
}

// IsEqual performs a shallow equality check (same pointer / same value for primitives).
// For Go, this uses == on comparable types. For non-comparable types, returns false
// unless they are the exact same interface value.
func IsEqual(a, b any) bool {
	// Handle nil
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	// Use reflect.DeepEqual for a shallow-like check on primitives.
	// Note: In Go, == on interfaces compares values for comparable types.
	// We use DeepEqual here because it handles all types safely.
	return reflect.DeepEqual(a, b)
}

// IsDeepEqual performs a deep equality check using reflect.DeepEqual.
func IsDeepEqual(a, b any) bool {
	return reflect.DeepEqual(a, b)
}

// IsPrimeNumber checks if an integer is a prime number.
func IsPrimeNumber(n int) bool {
	if n <= 1 {
		return false
	}
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

// IsPerfectSquare checks if a non-negative integer is a perfect square.
func IsPerfectSquare(n int) bool {
	if n < 0 {
		return false
	}
	sqrt := int(math.Sqrt(float64(n)))
	return sqrt*sqrt == n
}

// IsValueNaN checks if a float64 value is NaN.
func IsValueNaN(v float64) bool {
	return math.IsNaN(v)
}

// IsDictionaryObject checks if a value is a map type.
func IsDictionaryObject(v any) bool {
	if v == nil {
		return false
	}
	return reflect.TypeOf(v).Kind() == reflect.Map
}

// IsNotEmpty checks if a value is non-empty.
// For maps, slices, arrays, and strings, it checks length > 0.
// For structs, it checks if there are any fields.
// For other types, it checks if the value is non-zero.
func IsNotEmpty(v any) bool {
	if v == nil {
		return false
	}
	val := reflect.ValueOf(v)
	switch val.Kind() {
	case reflect.Map, reflect.Slice, reflect.Array, reflect.String:
		return val.Len() > 0
	case reflect.Struct:
		return val.NumField() > 0
	default:
		return !val.IsZero()
	}
}

// EmailParts holds the parsed local and domain parts of an email address.
type EmailParts struct {
	Local  string
	Domain string
}

// ParseEmailResult holds the result of email parsing.
type ParseEmailResult struct {
	Valid bool
	Parts *EmailParts
}

// Email regex patterns for different RFC levels
var emailPatterns = map[string]*regexp.Regexp{
	"basic": regexp.MustCompile(
		`^(?P<local>[a-zA-Z0-9.!#$%&'*+/=?^_` + "`" + `{|}~-]+)@(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)$`,
	),
}

// ParseEmail parses an email address and returns its local and domain parts.
// It uses the "basic" level validation pattern.
func ParseEmail(email string) (EmailParts, error) {
	pattern := emailPatterns["basic"]
	match := pattern.FindStringSubmatch(email)
	if match == nil {
		return EmailParts{}, fmt.Errorf("invalid email address: %s", email)
	}

	localIdx := pattern.SubexpIndex("local")
	domainIdx := pattern.SubexpIndex("domain")

	return EmailParts{
		Local:  match[localIdx],
		Domain: match[domainIdx],
	}, nil
}

// ParseEmailWithLevel parses an email address at the specified RFC compliance level.
// Supported levels: "basic", "rfc822", "rfc2822", "rfc5321", "rfc5322"
func ParseEmailWithLevel(email string, level string) ParseEmailResult {
	level = strings.ToLower(level)

	var pattern *regexp.Regexp
	switch level {
	case "basic":
		pattern = emailPatterns["basic"]
	case "rfc822":
		pattern = regexp.MustCompile(
			`^(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*(?P<local>"(?:[^"\\]|\\[\s\S]){0,62}"|[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~-]{1,64}(?:\.[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~-]+)*)(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*@(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*$`,
		)
	case "rfc2822":
		// RFC 2822 requires TLD, applies length check, rejects consecutive dots
		if len(email) > 998 {
			return ParseEmailResult{Valid: false, Parts: nil}
		}
		if strings.Contains(email, "..") {
			return ParseEmailResult{Valid: false, Parts: nil}
		}
		pattern = regexp.MustCompile(
			`^(?P<local>[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~-](?:[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~.+-]{0,62}[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~-])?)@(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*\.[a-zA-Z]{2,})$`,
		)
	case "rfc5321":
		// RFC 5321 has 256-byte path limit, requires TLD
		if len(email) > 256 {
			return ParseEmailResult{Valid: false, Parts: nil}
		}
		if strings.Contains(email, "..") {
			return ParseEmailResult{Valid: false, Parts: nil}
		}
		atIdx := strings.LastIndex(email, "@")
		if atIdx > 0 && atIdx < len(email)-1 {
			localPart := email[:atIdx]
			if len(localPart) > 64 {
				return ParseEmailResult{Valid: false, Parts: nil}
			}
		}
		pattern = regexp.MustCompile(
			`^(?P<local>(?:[a-zA-Z0-9!#$%&'*+/=?^_` + "`" + `{|}~-]+(?:\.[a-zA-Z0-9!#$%&'*+/=?^_` + "`" + `{|}~-]+)*|"(?:[^"\\]|\\[\s\S]){0,62}"))@(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+|\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9a-fA-F:]+)\])$`,
		)
	case "rfc5322":
		// RFC 5322 with comments, quoted strings, etc.
		if len(email) > 998 {
			return ParseEmailResult{Valid: false, Parts: nil}
		}
		pattern = regexp.MustCompile(
			`^(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*(?P<local>"(?:[^"\\]|\\[\s\S]){0,62}"(?:\."(?:[^"\\]|\\[\s\S]){0,62}")*|"(?:[^"\\]|\\[\s\S]){0,62}"(?:\.[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~+-]{1,64}(?:\.[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~+-]{1,64})*)+|[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~+-]{1,64}(?:\.[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~+-]{1,64})*(?:\."(?:[^"\\]|\\[\s\S]){0,62}")+|[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~+-]{1,64}(?:(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*\.(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*[a-zA-Z0-9!#$%&'*/=?^_` + "`" + `{|}~+-]{1,64})*)(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*@(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*\.(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+|\[(?:(?:[0-9]{1,3}\.){3}[0-9]{1,3}|IPv6:[0-9a-fA-F:]+)\])(?:\s|\((?:[^()\\]|\\[\s\S])*(?:\((?:[^()\\]|\\[\s\S])*\)(?:[^()\\]|\\[\s\S])*)*\))*$`,
		)
	default:
		return ParseEmailResult{Valid: false, Parts: nil}
	}

	match := pattern.FindStringSubmatch(email)
	if match == nil {
		return ParseEmailResult{Valid: false, Parts: nil}
	}

	localIdx := pattern.SubexpIndex("local")
	domainIdx := pattern.SubexpIndex("domain")

	return ParseEmailResult{
		Valid: true,
		Parts: &EmailParts{
			Local:  match[localIdx],
			Domain: match[domainIdx],
		},
	}
}
