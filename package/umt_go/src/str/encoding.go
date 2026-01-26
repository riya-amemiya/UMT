package str

import (
	"encoding/base64"
	"errors"
	"strings"
)

// ToBase64 encodes a string to Base64.
// It handles UTF-8 strings including multibyte characters.
func ToBase64(s string) string {
	return base64.StdEncoding.EncodeToString([]byte(s))
}

// FromBase64 decodes a Base64 string back to a regular string.
// Returns an error if the input is not a valid Base64 string.
func FromBase64(s string) (string, error) {
	if s == "" {
		return "", nil
	}

	decoded, err := base64.StdEncoding.DecodeString(s)
	if err != nil {
		return "", errors.New("Invalid Base64 string")
	}

	return string(decoded), nil
}

// ToHalfWidth converts full-width alphanumeric characters to half-width.
// This handles full-width digits (0-9), uppercase (A-Z), and lowercase (a-z).
func ToHalfWidth(s string) string {
	var b strings.Builder
	b.Grow(len(s))

	for _, r := range s {
		switch {
		case r >= '\uFF10' && r <= '\uFF19': // full-width digits 0-9
			b.WriteRune(r - 0xFEE0)
		case r >= '\uFF21' && r <= '\uFF3A': // full-width A-Z
			b.WriteRune(r - 0xFEE0)
		case r >= '\uFF41' && r <= '\uFF5A': // full-width a-z
			b.WriteRune(r - 0xFEE0)
		default:
			b.WriteRune(r)
		}
	}

	return b.String()
}
