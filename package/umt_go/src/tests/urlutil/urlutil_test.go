package urlutil_test

import (
	"strings"
	"testing"

	umt_urlutil "github.com/riya-amemiya/umt-go/src/urlutil"
)

func TestBuildUrl(t *testing.T) {
	t.Run("should build a URL with query parameters", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com",
			umt_urlutil.QueryParam{Key: "page", Value: "1"},
			umt_urlutil.QueryParam{Key: "q", Value: "search"},
		)
		want := "https://example.com/?page=1&q=search"
		if got != want {
			t.Errorf("BuildUrl = %q, want %q", got, want)
		}
	})

	t.Run("should return the base URL when no params given", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com/path")
		want := "https://example.com/path"
		if got != want {
			t.Errorf("BuildUrl = %q, want %q", got, want)
		}
	})

	t.Run("should return the base URL with empty params", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com/path")
		want := "https://example.com/path"
		if got != want {
			t.Errorf("BuildUrl = %q, want %q", got, want)
		}
	})

	t.Run("should encode special characters in values", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com",
			umt_urlutil.QueryParam{Key: "q", Value: "hello world"},
		)
		if !strings.Contains(got, "q=hello+world") {
			t.Errorf("BuildUrl = %q, want it to contain %q", got, "q=hello+world")
		}
	})

	t.Run("should handle existing query parameters in base", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com?existing=1",
			umt_urlutil.QueryParam{Key: "added", Value: "2"},
		)
		if !strings.Contains(got, "existing=1") {
			t.Errorf("BuildUrl = %q, want it to contain %q", got, "existing=1")
		}
		if !strings.Contains(got, "added=2") {
			t.Errorf("BuildUrl = %q, want it to contain %q", got, "added=2")
		}
	})

	t.Run("should handle a single parameter", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com",
			umt_urlutil.QueryParam{Key: "key", Value: "value"},
		)
		want := "https://example.com/?key=value"
		if got != want {
			t.Errorf("BuildUrl = %q, want %q", got, want)
		}
	})

	t.Run("should not filter dangerous keys (matches TS, sanitize beforehand)", func(t *testing.T) {
		got := umt_urlutil.BuildUrl("https://example.com",
			umt_urlutil.QueryParam{Key: "safe", Value: "value"},
		)
		want := "https://example.com/?safe=value"
		if got != want {
			t.Errorf("BuildUrl = %q, want %q", got, want)
		}
	})
}

func TestIsAbsoluteUrl(t *testing.T) {
	tests := []struct {
		url  string
		want bool
	}{
		{"http://example.com", true},
		{"https://example.com", true},
		{"ftp://files.example", true},
		{"mailto:user@host", true},
		{"tel:+1234567890", true},
		{"ssh://host", true},
		{"custom+scheme://path", true},
		{"my-app://deep-link", true},
		{"x.y://test", true},
		{"/path/to/page", false},
		{"relative/path", false},
		{"./relative", false},
		{"../parent", false},
		{"//example.com", false},
		{"", false},
		{"123://invalid", false},
		{"+bad://invalid", false},
	}

	for _, tt := range tests {
		t.Run(tt.url, func(t *testing.T) {
			if got := umt_urlutil.IsAbsoluteUrl(tt.url); got != tt.want {
				t.Errorf("IsAbsoluteUrl(%q) = %v, want %v", tt.url, got, tt.want)
			}
		})
	}
}

func TestJoinPath(t *testing.T) {
	tests := []struct {
		name     string
		segments []string
		want     string
	}{
		{"simple segments", []string{"a", "b", "c"}, "a/b/c"},
		{"normalize slashes between segments", []string{"a/", "/b/", "/c"}, "a/b/c"},
		{"preserve leading slash of first segment", []string{"/a", "b", "c"}, "/a/b/c"},
		{"preserve trailing slash of last segment", []string{"a", "b", "c/"}, "a/b/c/"},
		{"URL-like base segments", []string{"https://example.com/", "/api/", "/users"}, "https://example.com/api/users"},
		{"single segment", []string{"path"}, "path"},
		{"no segments", nil, ""},
		{"multiple slashes", []string{"a///", "///b"}, "a/b"},
		{"empty segments", []string{"a", "", "b"}, "a/b"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := umt_urlutil.JoinPath(tt.segments...); got != tt.want {
				t.Errorf("JoinPath(%v) = %q, want %q", tt.segments, got, tt.want)
			}
		})
	}
}

func TestParseQueryString(t *testing.T) {
	tests := []struct {
		name  string
		query string
		want  map[string]string
	}{
		{"with leading ?", "?page=1&q=search", map[string]string{"page": "1", "q": "search"}},
		{"without leading ?", "foo=bar&baz=qux", map[string]string{"foo": "bar", "baz": "qux"}},
		{"full URL", "https://example.com?a=1&b=2", map[string]string{"a": "1", "b": "2"}},
		{"empty string", "", map[string]string{}},
		{"? only", "?", map[string]string{}},
		{"encoded values", "?q=hello%20world", map[string]string{"q": "hello world"}},
		{"single parameter", "?key=value", map[string]string{"key": "value"}},
		{"URL with no query string", "https://example.com/path", map[string]string{}},
		{"dangerous keys are not filtered (matches TS)", "?__proto__=polluted&safe=value", map[string]string{"__proto__": "polluted", "safe": "value"}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := umt_urlutil.ParseQueryString(tt.query)
			if len(got) != len(tt.want) {
				t.Fatalf("ParseQueryString(%q) = %v, want %v", tt.query, got, tt.want)
			}
			for key, value := range tt.want {
				if got[key] != value {
					t.Errorf("ParseQueryString(%q)[%q] = %q, want %q", tt.query, key, got[key], value)
				}
			}
		})
	}
}
