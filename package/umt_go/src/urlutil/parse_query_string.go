package urlutil

import (
	"net/url"
	"strings"
)

// ParseQueryString parses a query string into a key/value map.
//
// It accepts either a full URL or a raw query string (with or without a leading
// "?"), mirroring the TypeScript parseQueryString. When the input contains
// "://" it is treated as a URL and only the query portion is parsed. Parsing
// follows the application/x-www-form-urlencoded rules of URLSearchParams: pairs
// are split on "&", names and values are split on the first "=", "+" decodes to
// a space, and percent-escapes are decoded. A name without "=" maps to an empty
// value; duplicate names keep the last value, matching the Record returned by
// the TypeScript original. Bytes that cannot be percent-decoded are kept
// verbatim, like URLSearchParams.
//
// Prototype-pollution note: like the TypeScript original, this function does not
// filter dangerous keys ("__proto__", "constructor", "prototype"); sanitize the
// result before trusting user-controlled input.
//
// Example:
//
//	ParseQueryString("?page=1&q=search")          // {"page": "1", "q": "search"}
//	ParseQueryString("foo=bar&baz=qux")           // {"foo": "bar", "baz": "qux"}
//	ParseQueryString("https://example.com?a=1&b=2") // {"a": "1", "b": "2"}
func ParseQueryString(query string) map[string]string {
	searchString := query
	if strings.Contains(query, "://") {
		if parsed, err := url.Parse(query); err == nil {
			searchString = parsed.RawQuery
		} else {
			searchString = ""
		}
	}

	searchString = strings.TrimPrefix(searchString, "?")

	result := make(map[string]string)
	if searchString == "" {
		return result
	}

	for _, pair := range strings.Split(searchString, "&") {
		if pair == "" {
			continue
		}
		name := pair
		value := ""
		if equalIndex := strings.IndexByte(pair, '='); equalIndex >= 0 {
			name = pair[:equalIndex]
			value = pair[equalIndex+1:]
		}
		result[decodeFormComponent(name)] = decodeFormComponent(value)
	}

	return result
}

// decodeFormComponent decodes a single application/x-www-form-urlencoded token,
// turning "+" into a space and resolving percent-escapes. Tokens that fail to
// decode are returned with only the "+" substitution applied, matching the
// lenient behavior of URLSearchParams.
func decodeFormComponent(component string) string {
	if decoded, err := url.QueryUnescape(component); err == nil {
		return decoded
	}
	return strings.ReplaceAll(component, "+", " ")
}
