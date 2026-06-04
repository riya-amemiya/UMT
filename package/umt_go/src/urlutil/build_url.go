package urlutil

import (
	"net/url"
	"strings"
)

// QueryParam is a single insertion-ordered key/value query parameter.
//
// The TypeScript buildUrl accepts a Record<string, string> and appends the
// parameters in Object.keys insertion order. A plain Go map cannot preserve
// that order, so the Go port takes an ordered slice of QueryParam instead.
type QueryParam struct {
	Key   string
	Value string
}

// BuildUrl builds a URL with query parameters appended.
//
// It mirrors the observable behavior of the TypeScript buildUrl, which relies
// on the WHATWG URL and URLSearchParams APIs: the base is normalized (a special
// scheme with a host but no path gains a trailing "/", e.g.
// "https://example.com" becomes "https://example.com/"), and each parameter is
// appended in the given order using application/x-www-form-urlencoded encoding
// (spaces become "+"). Parameters are appended after any query already present
// in the base. When base cannot be parsed it is returned unchanged.
//
// Prototype-pollution note: like the TypeScript original, this function does not
// filter dangerous keys ("__proto__", "constructor", "prototype"); sanitize
// user-controlled input before calling.
//
// Example:
//
//	BuildUrl("https://example.com", QueryParam{"page", "1"}, QueryParam{"q", "search"})
//	// "https://example.com/?page=1&q=search"
//
//	BuildUrl("https://example.com/path")
//	// "https://example.com/path"
//
//	BuildUrl("https://example.com", QueryParam{"q", "hello world"})
//	// "https://example.com/?q=hello+world"
func BuildUrl(base string, parameters ...QueryParam) string {
	parsed, err := url.Parse(base)
	if err != nil {
		return base
	}

	// WHATWG URL normalizes a special scheme that has a host but an empty path
	// to a single "/" path. net/url leaves the path empty, so add it back.
	if parsed.Host != "" && parsed.Path == "" {
		parsed.Path = "/"
	}

	existing := parsed.RawQuery
	var appended strings.Builder
	for index := range parameters {
		if appended.Len() > 0 {
			appended.WriteByte('&')
		}
		appended.WriteString(url.QueryEscape(parameters[index].Key))
		appended.WriteByte('=')
		appended.WriteString(url.QueryEscape(parameters[index].Value))
	}

	switch {
	case existing == "" && appended.Len() == 0:
		parsed.RawQuery = ""
	case existing == "":
		parsed.RawQuery = appended.String()
	case appended.Len() == 0:
		parsed.RawQuery = existing
	default:
		parsed.RawQuery = existing + "&" + appended.String()
	}

	return parsed.String()
}
