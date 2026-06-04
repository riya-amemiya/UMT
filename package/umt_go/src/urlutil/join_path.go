package urlutil

import (
	"regexp"
	"strings"
)

var (
	leadingSlashes  = regexp.MustCompile(`^/+`)
	trailingSlashes = regexp.MustCompile(`/+$`)
)

// JoinPath joins multiple path segments into a single path, normalizing slashes
// between segments.
//
// It replicates the TypeScript joinPath: the leading slash of the first segment
// is preserved, the trailing slash of the last segment is preserved, and all
// intermediate slashes are collapsed to a single slash. Empty segments (after
// stripping) are dropped. With no segments it returns "".
//
// Example:
//
//	JoinPath("https://example.com/", "/api/", "/users") // "https://example.com/api/users"
//	JoinPath("/a/", "/b/", "/c")                        // "/a/b/c"
//	JoinPath("a", "b", "c")                             // "a/b/c"
//	JoinPath()                                          // ""
func JoinPath(segments ...string) string {
	if len(segments) == 0 {
		return ""
	}

	normalized := make([]string, 0, len(segments))
	for index, segment := range segments {
		if index > 0 {
			segment = leadingSlashes.ReplaceAllString(segment, "")
		}
		if index < len(segments)-1 {
			segment = trailingSlashes.ReplaceAllString(segment, "")
		}
		if len(segment) > 0 {
			normalized = append(normalized, segment)
		}
	}

	return strings.Join(normalized, "/")
}
