package str_test

import (
	"math"
	"regexp"
	"strings"
	"testing"

	"github.com/riya-amemiya/umt-go/src/str"
)

// =============================================================================
// Tests from case_test.go
// =============================================================================

func TestCamelCase(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Convert kebab-case to camelCase
		{"kebab hello-world", "hello-world", "helloWorld"},
		{"kebab foo-bar-baz", "foo-bar-baz", "fooBarBaz"},
		// Convert snake_case to camelCase
		{"snake hello_world", "hello_world", "helloWorld"},
		{"snake foo_bar_baz", "foo_bar_baz", "fooBarBaz"},
		// Convert space-separated words
		{"space hello world", "hello world", "helloWorld"},
		{"space foo bar baz", "foo bar baz", "fooBarBaz"},
		// Handle PascalCase input
		{"pascal HelloWorld", "HelloWorld", "helloWorld"},
		{"pascal FooBarBaz", "FooBarBaz", "fooBarBaz"},
		// Handle already camelCase
		{"camel helloWorld", "helloWorld", "helloWorld"},
		{"camel fooBarBaz", "fooBarBaz", "fooBarBaz"},
		// Handle mixed separators
		{"mixed hello-world_test case", "hello-world_test case", "helloWorldTestCase"},
		{"mixed foo_bar-baz qux", "foo_bar-baz qux", "fooBarBazQux"},
		// Handle numbers
		{"numbers hello-world-2", "hello-world-2", "helloWorld2"},
		{"numbers test_case_123", "test_case_123", "testCase123"},
		// Handle empty string
		{"empty string", "", ""},
		// Handle single word
		{"single hello", "hello", "hello"},
		{"single HELLO", "HELLO", "hELLO"},
		// Handle special characters
		{"special hello@world", "hello@world", "helloWorld"},
		{"special foo#bar$baz", "foo#bar$baz", "fooBarBaz"},
		// Handle leading/trailing separators
		{"leading/trailing -hello-world-", "-hello-world-", "helloWorld"},
		{"leading/trailing _foo_bar_", "_foo_bar_", "fooBar"},
		// Handle multiple consecutive separators
		{"consecutive hello---world", "hello---world", "helloWorld"},
		{"consecutive foo___bar", "foo___bar", "fooBar"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.CamelCase(tt.input)
			if result != tt.expected {
				t.Errorf("CamelCase(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestKebabCase(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Convert camelCase to kebab-case
		{"camel helloWorld", "helloWorld", "hello-world"},
		{"camel fooBarBaz", "fooBarBaz", "foo-bar-baz"},
		// Convert PascalCase
		{"pascal HelloWorld", "HelloWorld", "hello-world"},
		{"pascal FooBarBaz", "FooBarBaz", "foo-bar-baz"},
		// Convert snake_case
		{"snake hello_world", "hello_world", "hello-world"},
		{"snake foo_bar_baz", "foo_bar_baz", "foo-bar-baz"},
		// Convert space-separated
		{"space hello world", "hello world", "hello-world"},
		{"space foo bar baz", "foo bar baz", "foo-bar-baz"},
		// Handle already kebab-case
		{"kebab hello-world", "hello-world", "hello-world"},
		{"kebab foo-bar-baz", "foo-bar-baz", "foo-bar-baz"},
		// Handle mixed separators
		{"mixed helloWorld_test case", "helloWorld_test case", "hello-world-test-case"},
		{"mixed fooBar-baz qux", "fooBar-baz qux", "foo-bar-baz-qux"},
		// Handle numbers
		{"numbers helloWorld2", "helloWorld2", "hello-world2"},
		{"numbers testCase123", "testCase123", "test-case123"},
		// Handle empty string
		{"empty string", "", ""},
		// Handle single word
		{"single hello", "hello", "hello"},
		{"single HELLO", "HELLO", "hello"},
		// Handle special characters
		{"special hello@world", "hello@world", "hello-world"},
		{"special foo#bar$baz", "foo#bar$baz", "foo-bar-baz"},
		// Handle leading/trailing separators
		{"leading/trailing -hello-world-", "-hello-world-", "hello-world"},
		{"leading/trailing _foo_bar_", "_foo_bar_", "foo-bar"},
		// Handle multiple consecutive separators
		{"consecutive hello---world", "hello---world", "hello-world"},
		{"consecutive foo___bar", "foo___bar", "foo-bar"},
		// Handle complex mixed case
		{"complex XMLHttpRequest", "XMLHttpRequest", "xml-http-request"},
		{"complex getElementById", "getElementById", "get-element-by-id"},
		// Handle acronyms
		{"acronym HTML", "HTML", "html"},
		{"acronym XMLParser", "XMLParser", "xml-parser"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.KebabCase(tt.input)
			if result != tt.expected {
				t.Errorf("KebabCase(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestSlugify(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// JSDoc examples
		{"Hello World!", "Hello World!", "hello-world"},
		{"This is a Test", "This is a Test", "this-is-a-test"},
		{"Japanese", "Japanese: \u3053\u3093\u306b\u3061\u306f", "japanese"},
		// Multiple spaces
		{"multiple spaces", "Hello    World", "hello-world"},
		// Leading and trailing spaces
		{"leading/trailing spaces", "  Hello World  ", "hello-world"},
		// Remove special characters
		{"special characters", "Special!@#$%Characters", "special-characters"},
		// Handle underscores
		{"underscores", "snake_case_text", "snake-case-text"},
		// Handle unicode characters
		{"unicode cafe", "caf\u00e9", "cafe"},
		{"unicode naive", "na\u00efve", "naive"},
		// Handle numbers
		{"numbers Test 123", "Test 123", "test-123"},
		{"numbers Version 2.5", "Version 2.5", "version-2-5"},
		// Handle empty string
		{"empty string", "", ""},
		// Handle consecutive hyphens
		{"consecutive hyphens", "Hello---World", "hello-world"},
		// Handle mixed case
		{"mixed case", "CamelCase", "camelcase"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.Slugify(tt.input)
			if result != tt.expected {
				t.Errorf("Slugify(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

// =============================================================================
// Tests from trim_test.go
// =============================================================================

func TestTrimCharacters(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		chars    string
		expected string
	}{
		{"remove from both ends", "---Hello World---", "-", "Hello World"},
		{"keep in middle", "---Hello-World---", "-", "Hello-World"},
		{"empty input", "", "-", ""},
		{"empty chars", "Hello World", "", "Hello World"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.TrimCharacters(tt.input, tt.chars)
			if result != tt.expected {
				t.Errorf("TrimCharacters(%q, %q) = %q, want %q", tt.input, tt.chars, result, tt.expected)
			}
		})
	}
}

func TestTrimStartCharacters(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		chars    string
		expected string
	}{
		{"remove dashes from start", "---Hello", "-", "Hello"},
		{"remove exclamation from start", "!!!world", "!", "world"},
		{"multiple different chars", "...123test", ".123", "test"},
		{"mixed chars", "---...text", ".-", "text"},
		{"no match", "hello", "x", "hello"},
		{"no match multiple", "test", "xyz", "test"},
		{"empty input", "", "x", ""},
		{"empty input empty chars", "", "", ""},
		{"all chars trimmed", "xxxxx", "x", ""},
		{"all dots trimmed", ".....", ".", ""},
		{"empty chars", "hello", "", "hello"},
		{"empty chars with numbers", "123test", "", "123test"},
		// Non-ASCII characters
		{"japanese period", "\u3002\u3002\u3002\u3053\u3093\u306b\u3061\u306f", "\u3002", "\u3053\u3093\u306b\u3061\u306f"},
		{"full-width exclamation", "\uff01\uff01Hello", "\uff01", "Hello"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.TrimStartCharacters(tt.input, tt.chars)
			if result != tt.expected {
				t.Errorf("TrimStartCharacters(%q, %q) = %q, want %q", tt.input, tt.chars, result, tt.expected)
			}
		})
	}
}

func TestTrimEndCharacters(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		chars    string
		expected string
	}{
		{"remove o from end", "hellooo", "o", "hell"},
		{"remove exclamation from end", "banana!!!", "!", "banana"},
		{"multiple different chars", "hello123...", "123.", "hello"},
		{"mixed chars", "test---...", ".-", "test"},
		{"no match", "apple", "x", "apple"},
		{"no match multiple", "test", "xyz", "test"},
		{"empty input", "", "x", ""},
		{"empty input empty chars", "", "", ""},
		{"all chars trimmed", "xxxxx", "x", ""},
		{"all dots trimmed", "....", ".", ""},
		{"empty chars", "hello", "", "hello"},
		{"empty chars with numbers", "test123", "", "test123"},
		// Non-ASCII characters
		{"japanese period", "\u3053\u3093\u306b\u3061\u306f\u3002\u3002\u3002", "\u3002", "\u3053\u3093\u306b\u3061\u306f"},
		{"full-width exclamation", "Hello\uff01\uff01", "\uff01", "Hello"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.TrimEndCharacters(tt.input, tt.chars)
			if result != tt.expected {
				t.Errorf("TrimEndCharacters(%q, %q) = %q, want %q", tt.input, tt.chars, result, tt.expected)
			}
		})
	}
}

func TestPadStart(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		length   int
		pad      string
		expected string
	}{
		// Basic functionality
		{"pad with space", "abc", 5, " ", "  abc"},
		{"pad with zero", "abc", 5, "0", "00abc"},
		// Already at or exceeding target length
		{"shorter target", "abc", 2, " ", "abc"},
		{"equal length", "abc", 3, " ", "abc"},
		{"zero target", "abc", 0, " ", "abc"},
		// Empty string input
		{"empty input zeros", "", 3, "0", "000"},
		{"empty input x", "", 4, "x", "xxxx"},
		// Multi-character padding
		{"repeat xy", "abc", 7, "xy", "xyxyabc"},
		{"repeat 123", "abc", 8, "123", "12312abc"},
		// Truncate padding
		{"truncate 12345", "abc", 6, "12345", "123abc"},
		{"truncate 0000", "abc", 5, "0000", "00abc"},
		// Single char padding
		{"single char x", "abc", 5, "x", "xxabc"},
		// Empty padding string
		{"empty pad", "abc", 5, "", "abc"},
		{"empty pad 2", "test", 10, "", "test"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.PadStart(tt.input, tt.length, tt.pad)
			if result != tt.expected {
				t.Errorf("PadStart(%q, %d, %q) = %q, want %q", tt.input, tt.length, tt.pad, result, tt.expected)
			}
		})
	}
}

func TestPadEnd(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		length   int
		pad      string
		expected string
	}{
		{"pad with space", "abc", 5, " ", "abc  "},
		{"pad with exclamation", "hello", 10, "!", "hello!!!!!"},
		{"equal length", "abc", 3, " ", "abc"},
		{"longer string", "longstring", 5, "!", "longstring"},
		{"multi-char padding", "abc", 10, "de", "abcdededed"},
		{"shorter target", "abc", 2, " ", "abc"},
		{"empty padding", "abc", 5, "", "abc"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.PadEnd(tt.input, tt.length, tt.pad)
			if result != tt.expected {
				t.Errorf("PadEnd(%q, %d, %q) = %q, want %q", tt.input, tt.length, tt.pad, result, tt.expected)
			}
		})
	}
}

// =============================================================================
// Tests from encoding_test.go
// =============================================================================

func TestToBase64(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"normal string", "test", "dGVzdA=="},
		{"empty string", "", ""},
		{"special characters", "@#%", "QCMl"},
		{"japanese characters", "\u3042\u3044\u3046\u3048\u304a", "44GC44GE44GG44GI44GK"},
		{"long string", "This is a long string to test the Base64 conversion", "VGhpcyBpcyBhIGxvbmcgc3RyaW5nIHRvIHRlc3QgdGhlIEJhc2U2NCBjb252ZXJzaW9u"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.ToBase64(tt.input)
			if result != tt.expected {
				t.Errorf("ToBase64(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestFromBase64(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		wantErr  bool
	}{
		{"normal string", "dGVzdA==", "test", false},
		{"empty string", "", "", false},
		{"special characters", "QCMl", "@#%", false},
		{"japanese characters", "44GC44GE44GG44GI44GK", "\u3042\u3044\u3046\u3048\u304a", false},
		// Padding patterns
		{"2 padding chars", "YQ==", "a", false},
		{"1 padding char", "YWE=", "aa", false},
		{"no padding", "YWFh", "aaa", false},
		// Invalid Base64
		{"invalid base64 abc@!#", "abc@!#", "", true},
		{"invalid base64 =abc", "=abc", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := str.FromBase64(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("FromBase64(%q) expected error, got nil", tt.input)
				} else if err.Error() != "Invalid Base64 string" {
					t.Errorf("FromBase64(%q) error = %q, want %q", tt.input, err.Error(), "Invalid Base64 string")
				}
			} else {
				if err != nil {
					t.Errorf("FromBase64(%q) unexpected error: %v", tt.input, err)
				}
				if result != tt.expected {
					t.Errorf("FromBase64(%q) = %q, want %q", tt.input, result, tt.expected)
				}
			}
		})
	}

	// Roundtrip tests
	t.Run("roundtrip", func(t *testing.T) {
		roundtrips := []string{
			"test",
			"@#%",
			"\u3042\u3044\u3046\u3048\u304a",
			"\U0001F30A\U0001F30D\U0001F30E",
		}
		for _, s := range roundtrips {
			encoded := str.ToBase64(s)
			decoded, err := str.FromBase64(encoded)
			if err != nil {
				t.Errorf("Roundtrip error for %q: %v", s, err)
			}
			if decoded != s {
				t.Errorf("Roundtrip failed for %q: got %q", s, decoded)
			}
		}
	})
}

func TestToHalfWidth(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"full-width digits", "\uFF10\uFF11\uFF12\uFF13\uFF14\uFF15\uFF16\uFF17\uFF18\uFF19", "0123456789"},
		{"full-width uppercase", "\uFF21\uFF22\uFF23\uFF24\uFF25\uFF26\uFF27\uFF28\uFF29\uFF2A", "ABCDEFGHIJ"},
		{"full-width lowercase", "\uFF41\uFF42\uFF43\uFF44\uFF45\uFF46\uFF47\uFF48\uFF49\uFF4A", "abcdefghij"},
		{"mixed full and half width", "\uFF21\uFF22\uFF23abc\uFF11\uFF12\uFF13123", "ABCabc123123"},
		{"non-target characters unchanged", "\u6F22\u5B57\u30AB\u30BF\u30AB\u30CA\u3001\u3002", "\u6F22\u5B57\u30AB\u30BF\u30AB\u30CA\u3001\u3002"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.ToHalfWidth(tt.input)
			if result != tt.expected {
				t.Errorf("ToHalfWidth(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

// =============================================================================
// Tests from processing_test.go
// =============================================================================

func TestDeleteSpaces(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"remove spaces", "Hello World", "HelloWorld"},
		{"remove multiple spaces", "   Hello   World   ", "HelloWorld"},
		{"remove tabs", "Hello\tWorld", "HelloWorld"},
		{"remove newlines", "Hello\nWorld\r\n", "HelloWorld"},
		{"remove em space", "Hello\u2003World", "HelloWorld"},
		{"empty string", "", ""},
		{"only spaces", "   ", ""},
		{"multibyte with space", "\u3053\u3093\u306b\u3061\u306f \u4e16\u754c", "\u3053\u3093\u306b\u3061\u306f\u4e16\u754c"},
		{"full-width space", "Hello\u3000World", "HelloWorld"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.DeleteSpaces(tt.input)
			if result != tt.expected {
				t.Errorf("DeleteSpaces(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestReverseString(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"three chars", "abc", "cba"},
		{"empty string", "", ""},
		{"single char", "a", "a"},
		{"two chars", "ab", "ba"},
		{"four chars", "abcd", "dcba"},
		{"five chars", "abcde", "edcba"},
		{"six chars", "abcdef", "fedcba"},
		{"seven chars", "abcdefg", "gfedcba"},
		{"eight chars", "abcdefgh", "hgfedcba"},
		{"whitespace", "hello world", "dlrow olleh"},
		{"spaces only", "  ", "  "},
		{"special chars", "!@#$%", "%$#@!"},
		{"dashes", "a-b-c", "c-b-a"},
		{"numbers", "12345", "54321"},
		{"mixed numbers", "a1b2c3", "3c2b1a"},
		{"unicode hiragana", "\u3042\u3044\u3046", "\u3046\u3044\u3042"},
		{"unicode kanji", "\u65e5\u672c\u8a9e", "\u8a9e\u672c\u65e5"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.ReverseString(tt.input)
			if result != tt.expected {
				t.Errorf("ReverseString(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestTruncate(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		length   int
		suffix   string
		expected string
	}{
		// JSDoc examples
		{"truncate with default suffix", "Hello World", 5, "...", "Hello..."},
		{"truncate with custom suffix", "Hello World", 5, "~", "Hello~"},
		{"no truncation needed", "Hello", 10, "...", "Hello"},
		// Not truncated
		{"short string", "Hi", 5, "...", "Hi"},
		{"exact length", "Hello", 5, "...", "Hello"},
		// Empty suffix
		{"empty suffix", "Hello World", 5, "", "Hello"},
		// Zero length
		{"zero length empty suffix", "Hello", 0, "", ""},
		{"zero length default suffix", "Hello", 0, "...", "..."},
		// Suffix longer than target
		{"suffix longer 2", "Hello World", 2, "...", "He..."},
		{"suffix longer 1", "Hello World", 1, "...", "H..."},
		// Empty string
		{"empty string", "", 5, "...", ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.Truncate(tt.input, tt.length, tt.suffix)
			if result != tt.expected {
				t.Errorf("Truncate(%q, %d, %q) = %q, want %q", tt.input, tt.length, tt.suffix, result, tt.expected)
			}
		})
	}

	// Test negative length panics
	t.Run("negative length panics", func(t *testing.T) {
		defer func() {
			r := recover()
			if r == nil {
				t.Errorf("Truncate with negative length should panic")
			}
			if r != "Length must be non-negative" {
				t.Errorf("Expected panic message %q, got %q", "Length must be non-negative", r)
			}
		}()
		str.Truncate("Hello", -1, "...")
	})
}

func TestHasNoLetters(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected bool
	}{
		// True cases: no letters
		{"empty string", "", true},
		{"numbers only", "12345", true},
		{"special chars only", "!@#$%", true},
		{"whitespace only", "     ", true},
		{"emoji single", "\U0001F389", true},
		{"emoji multiple", "\U0001F600\U0001F603\U0001F604", true},
		{"decorative symbols", "\u2765\uff65\u2022\u2744", true},
		{"full-width symbols", "\uff1f\uff1f\uff1f\uff1f\uff1f", true},
		{"multiline emojis", "\n      \U0001F388\n      \U0001F38A\n      \U0001F389\n    ", true},
		// False cases: has letters
		{"english text", "hello", false},
		{"mixed with numbers", "test 123", false},
		{"japanese text", "\u3053\u3093\u306b\u3061\u306f", false},
		{"accented chars", "Caf\u00e9", false},
		{"text with emoji", "hello \U0001F44B", false},
		{"emoji with text", "\U0001F389 party", false},
		{"multiline mixed", "\n      Hello\n      World\n      \U0001F604\n      123\n    ", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.HasNoLetters(tt.input)
			if result != tt.expected {
				t.Errorf("HasNoLetters(%q) = %v, want %v", tt.input, result, tt.expected)
			}
		})
	}
}

// =============================================================================
// Tests from html_test.go
// =============================================================================

func TestEscapeHtml(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"escape ampersand", "Tom & Jerry", "Tom &amp; Jerry"},
		{"escape less than", "5 < 10", "5 &lt; 10"},
		{"escape greater than", "10 > 5", "10 &gt; 5"},
		{"escape double quotes", `Say "Hello"`, "Say &quot;Hello&quot;"},
		{"escape single quotes", "It's working", "It&#39;s working"},
		{"escape all special characters",
			`<script>alert("XSS & 'injection'");</script>`,
			"&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;"},
		{"empty string", "", ""},
		{"no special characters", "Hello World", "Hello World"},
		{"only special characters", `&<>"'`, "&amp;&lt;&gt;&quot;&#39;"},
		{"repeated ampersands", "&&&", "&amp;&amp;&amp;"},
		{"repeated less than", "<<<", "&lt;&lt;&lt;"},
		{"mixed content",
			"User input: <b>Hello & 'World'</b>",
			"User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;"},
		{"HTML attributes",
			`<img src="test.jpg" alt="Tom & Jerry's picture">`,
			"&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;"},
		{"JavaScript code",
			`if (x < 5 && y > 3) { alert("Hello 'World'"); }`,
			"if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.EscapeHtml(tt.input)
			if result != tt.expected {
				t.Errorf("EscapeHtml(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestUnescapeHtml(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"unescape ampersand", "Tom &amp; Jerry", "Tom & Jerry"},
		{"unescape less than", "5 &lt; 10", "5 < 10"},
		{"unescape greater than", "10 &gt; 5", "10 > 5"},
		{"unescape double quotes", "Say &quot;Hello&quot;", `Say "Hello"`},
		{"unescape single quotes", "It&#39;s working", "It's working"},
		{"unescape hex single quotes", "It&#x27;s working", "It's working"},
		{"unescape all basic entities",
			"&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;",
			`<script>alert("XSS & 'injection'");</script>`},
		{"empty string", "", ""},
		{"no entities", "Hello World", "Hello World"},
		{"only entities", "&amp;&lt;&gt;&quot;&#39;", "&<>\"'"},
		{"repeated ampersand entities", "&amp;&amp;&amp;", "&&&"},
		{"repeated less than entities", "&lt;&lt;&lt;", "<<<"},
		{"mixed content",
			"User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;",
			"User input: <b>Hello & 'World'</b>"},
		{"HTML attributes",
			"&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;",
			`<img src="test.jpg" alt="Tom & Jerry's picture">`},
		{"JavaScript code",
			"if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }",
			`if (x < 5 && y > 3) { alert("Hello 'World'"); }`},
		// Decimal numeric references
		{"decimal A", "&#65;", "A"},
		{"decimal euro", "&#8364;", "\u20AC"},
		{"decimal grin emoji", "&#128512;", "\U0001F600"},
		// Hexadecimal numeric references
		{"hex A", "&#x41;", "A"},
		{"hex euro", "&#x20AC;", "\u20AC"},
		{"hex grin emoji", "&#x1F600;", "\U0001F600"},
		// Mixed case hex
		{"hex lowercase", "&#x20ac;", "\u20AC"},
		{"hex uppercase", "&#x20AC;", "\u20AC"},
		{"uppercase X not matched", "&#X41;", "&#X41;"},
		// Extended entities
		{"hex slash", "&#x2F;", "/"},
		{"hex backtick", "&#x60;", "`"},
		{"hex equals", "&#x3D;", "="},
		// Invalid numeric references
		{"empty decimal", "&#;", "&#;"},
		{"empty hex", "&#x;", "&#x;"},
		{"invalid decimal", "&#invalid;", "&#invalid;"},
		{"invalid hex", "&#xinvalid;", "&#xinvalid;"},
		// Malformed entities
		{"no semicolon", "&lt", "&lt"},
		{"unknown entity", "&unknown;", "&unknown;"},
		{"ampersand hash only", "&#", "&#"},
		{"nonexistent entity", "&nonexistent;", "&nonexistent;"},
		{"notinmap entity", "&notinmap;", "&notinmap;"},
		{"test entity", "&test;", "&test;"},
		// Unicode in numeric references
		{"decimal hiragana a", "&#12354;", "\u3042"},
		{"hex hiragana a", "&#x3042;", "\u3042"},
		{"decimal bullet", "&#8226;", "\u2022"},
		{"hex bullet", "&#x2022;", "\u2022"},
		// Preserve already unescaped content
		{"already unescaped", "Already < unescaped & content with 'quotes'", "Already < unescaped & content with 'quotes'"},
		// Complex HTML document
		{"complex HTML doc",
			"&lt;!DOCTYPE html&gt;\n&lt;html&gt;\n&lt;head&gt;\n    &lt;title&gt;Test &amp; Demo&lt;/title&gt;\n&lt;/head&gt;\n&lt;body&gt;\n    &lt;p&gt;Hello &#39;World&#39; &amp; welcome!&lt;/p&gt;\n&lt;/body&gt;\n&lt;/html&gt;",
			"<!DOCTYPE html>\n<html>\n<head>\n    <title>Test & Demo</title>\n</head>\n<body>\n    <p>Hello 'World' & welcome!</p>\n</body>\n</html>"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := str.UnescapeHtml(tt.input)
			if result != tt.expected {
				t.Errorf("UnescapeHtml(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestEscapeUnescapeRoundtrip(t *testing.T) {
	testCases := []string{
		"Hello & World",
		`<script>alert("test");</script>`,
		"It's a 'test' & more",
		"5 < 10 > 3",
		"",
		"No special chars",
	}

	for _, tc := range testCases {
		escaped := str.EscapeHtml(tc)
		unescaped := str.UnescapeHtml(escaped)
		if unescaped != tc {
			t.Errorf("Roundtrip failed for %q: escaped=%q, unescaped=%q", tc, escaped, unescaped)
		}
	}
}

// =============================================================================
// Tests from analysis_test.go
// NOTE: levenshteinDistance is unexported, so TestLevenshteinDistance from
// analysis_test.go is omitted (it tests an unexported function).
// =============================================================================

func TestStringSimilarity(t *testing.T) {
	// Identical strings
	t.Run("identical strings", func(t *testing.T) {
		if got := str.StringSimilarity("hello", "hello"); got != 1 {
			t.Errorf("StringSimilarity(hello, hello) = %f, want 1", got)
		}
		if got := str.StringSimilarity("world", "world"); got != 1 {
			t.Errorf("StringSimilarity(world, world) = %f, want 1", got)
		}
	})

	// Empty strings
	t.Run("empty strings", func(t *testing.T) {
		if got := str.StringSimilarity("", "hello"); got != 0 {
			t.Errorf("StringSimilarity('', hello) = %f, want 0", got)
		}
		if got := str.StringSimilarity("hello", ""); got != 0 {
			t.Errorf("StringSimilarity(hello, '') = %f, want 0", got)
		}
		if got := str.StringSimilarity("", ""); got != 1 {
			t.Errorf("StringSimilarity('', '') = %f, want 1", got)
		}
	})

	// Similar strings
	t.Run("similar strings", func(t *testing.T) {
		sim1 := str.StringSimilarity("cat", "bat")
		if math.Abs(sim1-0.667) > 0.001 {
			t.Errorf("StringSimilarity(cat, bat) = %f, want ~0.667", sim1)
		}

		sim2 := str.StringSimilarity("kitten", "sitting")
		if math.Abs(sim2-0.571) > 0.001 {
			t.Errorf("StringSimilarity(kitten, sitting) = %f, want ~0.571", sim2)
		}
	})

	// Completely different strings
	t.Run("completely different strings", func(t *testing.T) {
		sim := str.StringSimilarity("abc", "xyz")
		if sim != 0 {
			t.Errorf("StringSimilarity(abc, xyz) = %f, want 0", sim)
		}
	})

	// Different lengths
	t.Run("different lengths", func(t *testing.T) {
		sim1 := str.StringSimilarity("cat", "cats")
		if sim1 != 0.75 {
			t.Errorf("StringSimilarity(cat, cats) = %f, want 0.75", sim1)
		}

		sim2 := str.StringSimilarity("hello", "helo")
		if sim2 != 0.8 {
			t.Errorf("StringSimilarity(hello, helo) = %f, want 0.8", sim2)
		}
	})

	// Case sensitive
	t.Run("case sensitive", func(t *testing.T) {
		sim := str.StringSimilarity("Hello", "hello")
		if sim != 0.8 {
			t.Errorf("StringSimilarity(Hello, hello) = %f, want 0.8", sim)
		}
	})

	// Unicode
	t.Run("unicode", func(t *testing.T) {
		sim1 := str.StringSimilarity("caf\u00e9", "cafe")
		if sim1 != 0.75 {
			t.Errorf("StringSimilarity(cafe, cafe) = %f, want 0.75", sim1)
		}
	})

	// Values between 0 and 1
	t.Run("values between 0 and 1", func(t *testing.T) {
		pairs := [][2]string{
			{"hello", "world"},
			{"test", "testing"},
			{"a", "b"},
			{"similar", "similarity"},
		}
		for _, pair := range pairs {
			sim := str.StringSimilarity(pair[0], pair[1])
			if sim < 0 || sim > 1 {
				t.Errorf("StringSimilarity(%q, %q) = %f, expected between 0 and 1", pair[0], pair[1], sim)
			}
		}
	})
}

func TestFuzzySearch(t *testing.T) {
	// JSDoc example
	t.Run("basic fuzzy search", func(t *testing.T) {
		result := str.FuzzySearch("hello", []string{"hello", "world", "helo", "help"})
		if len(result) != 3 {
			t.Fatalf("Expected 3 results, got %d", len(result))
		}

		found := map[string]bool{}
		for _, r := range result {
			found[r.Item] = true
		}
		if !found["hello"] {
			t.Error("Expected 'hello' in results")
		}
		if !found["helo"] {
			t.Error("Expected 'helo' in results")
		}
		if !found["help"] {
			t.Error("Expected 'help' in results")
		}

		for _, r := range result {
			if r.Item == "hello" && r.Score != 1 {
				t.Errorf("Expected score 1 for 'hello', got %f", r.Score)
			}
			if r.Item == "helo" && r.Score != 0.8 {
				t.Errorf("Expected score 0.8 for 'helo', got %f", r.Score)
			}
			if r.Item == "help" && r.Score != 0.6 {
				t.Errorf("Expected score 0.6 for 'help', got %f", r.Score)
			}
		}
	})

	// Exact matches
	t.Run("exact matches", func(t *testing.T) {
		result := str.FuzzySearch("test", []string{"test", "best", "rest"})
		if len(result) == 0 {
			t.Fatal("Expected at least one result")
		}
		if result[0].Item != "test" || result[0].Score != 1 {
			t.Errorf("Expected first result to be test with score 1, got %s with %f", result[0].Item, result[0].Score)
		}
	})

	// Sorted by score descending
	t.Run("sorted by score descending", func(t *testing.T) {
		result := str.FuzzySearch("test", []string{"test", "tests", "testing"})
		for i := 0; i < len(result)-1; i++ {
			if result[i].Score < result[i+1].Score {
				t.Errorf("Results not sorted: index %d score %f < index %d score %f", i, result[i].Score, i+1, result[i+1].Score)
			}
		}
	})

	// Custom threshold
	t.Run("custom threshold", func(t *testing.T) {
		highThreshold := str.FuzzySearchWithThreshold("hello", []string{"hello", "helo", "help"}, 0.9)
		lowThreshold := str.FuzzySearchWithThreshold("hello", []string{"hello", "helo", "help"}, 0.3)
		if len(highThreshold) > len(lowThreshold) {
			t.Errorf("High threshold results (%d) should be <= low threshold results (%d)", len(highThreshold), len(lowThreshold))
		}
	})

	// Case insensitive
	t.Run("case insensitive", func(t *testing.T) {
		result := str.FuzzySearch("Hello", []string{"HELLO", "hello", "Hello"})
		if len(result) != 3 {
			t.Fatalf("Expected 3 results, got %d", len(result))
		}
		for _, r := range result {
			if r.Score != 1 {
				t.Errorf("Expected score 1 for %q, got %f", r.Item, r.Score)
			}
		}
	})

	// Empty query
	t.Run("empty query", func(t *testing.T) {
		result := str.FuzzySearch("", []string{"hello", "world"})
		if len(result) != 0 {
			t.Errorf("Expected empty results for empty query, got %d", len(result))
		}
	})

	// Empty items
	t.Run("empty items", func(t *testing.T) {
		result := str.FuzzySearch("hello", []string{})
		if len(result) != 0 {
			t.Errorf("Expected empty results for empty items, got %d", len(result))
		}
	})

	// Filter out items below threshold
	t.Run("filter below threshold", func(t *testing.T) {
		result := str.FuzzySearchWithThreshold("hello", []string{"world", "xyz"}, 0.8)
		if len(result) != 0 {
			t.Errorf("Expected empty results, got %d", len(result))
		}
	})

	// Single character matches
	t.Run("single character matches", func(t *testing.T) {
		result := str.FuzzySearch("a", []string{"a", "b", "ab"})
		found := false
		for _, r := range result {
			if r.Item == "a" && r.Score == 1 {
				found = true
			}
		}
		if !found {
			t.Error("Expected 'a' with score 1 in results")
		}
	})

	// Special characters
	t.Run("special characters", func(t *testing.T) {
		result := str.FuzzySearch("hello!", []string{"hello!", "hello"})
		if len(result) == 0 {
			t.Fatal("Expected at least one result")
		}
		if result[0].Item != "hello!" || result[0].Score != 1 {
			t.Errorf("Expected first result to be 'hello!' with score 1, got %s with %f", result[0].Item, result[0].Score)
		}
	})
}

func TestRandomString(t *testing.T) {
	// Default length and characters
	t.Run("default length and characters", func(t *testing.T) {
		s := str.RandomString(8)
		if len(s) != 8 {
			t.Errorf("Expected length 8, got %d", len(s))
		}
		matched, _ := regexp.MatchString(`^[0-9A-Za-z]{8}$`, s)
		if !matched {
			t.Errorf("Expected alphanumeric string, got %q", s)
		}
	})

	// Custom character set
	t.Run("custom character set", func(t *testing.T) {
		s := str.RandomStringWithChars(10, "abc123")
		if len(s) != 10 {
			t.Errorf("Expected length 10, got %d", len(s))
		}
		matched, _ := regexp.MatchString(`^[abc123]{10}$`, s)
		if !matched {
			t.Errorf("Expected string from charset abc123, got %q", s)
		}
	})

	// Specified length
	t.Run("specified length", func(t *testing.T) {
		s := str.RandomString(20)
		if len(s) != 20 {
			t.Errorf("Expected length 20, got %d", len(s))
		}
	})
}

// =============================================================================
// NEW: Tests for FormatString
// =============================================================================

func TestFormatString(t *testing.T) {
	t.Run("indexed placeholder basic", func(t *testing.T) {
		result := str.FormatString("Hello {0}!", "World")
		if result != "Hello World!" {
			t.Errorf("Expected %q, got %q", "Hello World!", result)
		}
	})

	t.Run("named placeholder with map", func(t *testing.T) {
		result := str.FormatString("{name} is {age}", map[string]any{"name": "Alice", "age": 30})
		if result != "Alice is 30" {
			t.Errorf("Expected %q, got %q", "Alice is 30", result)
		}
	})

	t.Run("multiple indexed placeholders", func(t *testing.T) {
		result := str.FormatString("{0} + {1} = {2}", 1, 2, 3)
		if result != "1 + 2 = 3" {
			t.Errorf("Expected %q, got %q", "1 + 2 = 3", result)
		}
	})

	t.Run("repeated indexed placeholder", func(t *testing.T) {
		result := str.FormatString("{0} and {0}", "hello")
		if result != "hello and hello" {
			t.Errorf("Expected %q, got %q", "hello and hello", result)
		}
	})

	t.Run("empty template", func(t *testing.T) {
		result := str.FormatString("")
		if result != "" {
			t.Errorf("Expected empty string, got %q", result)
		}
	})

	t.Run("no placeholders", func(t *testing.T) {
		result := str.FormatString("Hello World!")
		if result != "Hello World!" {
			t.Errorf("Expected %q, got %q", "Hello World!", result)
		}
	})

	t.Run("escaped braces", func(t *testing.T) {
		result := str.FormatString("{{name}}")
		if result != "{name}" {
			t.Errorf("Expected %q, got %q", "{name}", result)
		}
	})

	t.Run("named with multiple keys", func(t *testing.T) {
		result := str.FormatString("{greeting}, {name}!", map[string]any{
			"greeting": "Hi",
			"name":     "Bob",
		})
		if result != "Hi, Bob!" {
			t.Errorf("Expected %q, got %q", "Hi, Bob!", result)
		}
	})

	t.Run("missing named key uses placeholder", func(t *testing.T) {
		result := str.FormatString("{name} is {missing}", map[string]any{"name": "Alice"})
		if result != "Alice is {missing}" {
			t.Errorf("Expected %q, got %q", "Alice is {missing}", result)
		}
	})

	t.Run("default value with pipe", func(t *testing.T) {
		result := str.FormatString("{name|Unknown}", map[string]any{})
		if result != "Unknown" {
			t.Errorf("Expected %q, got %q", "Unknown", result)
		}
	})

	t.Run("formatter upper", func(t *testing.T) {
		result := str.FormatString("{name:upper}", map[string]any{"name": "alice"})
		if result != "ALICE" {
			t.Errorf("Expected %q, got %q", "ALICE", result)
		}
	})

	t.Run("formatter lower", func(t *testing.T) {
		result := str.FormatString("{name:lower}", map[string]any{"name": "ALICE"})
		if result != "alice" {
			t.Errorf("Expected %q, got %q", "alice", result)
		}
	})

	t.Run("string type arguments", func(t *testing.T) {
		result := str.FormatString("{0} {1}", "foo", "bar")
		if result != "foo bar" {
			t.Errorf("Expected %q, got %q", "foo bar", result)
		}
	})

	t.Run("integer type arguments", func(t *testing.T) {
		result := str.FormatString("Count: {0}", 42)
		if result != "Count: 42" {
			t.Errorf("Expected %q, got %q", "Count: 42", result)
		}
	})
}

// =============================================================================
// NEW: Tests for RandomStringInitialization
// =============================================================================

func TestRandomStringInitialization(t *testing.T) {
	t.Run("generates correct length", func(t *testing.T) {
		gen := str.RandomStringInitialization("ABC")
		s := gen(10)
		if len(s) != 10 {
			t.Errorf("Expected length 10, got %d", len(s))
		}
	})

	t.Run("uses only charset characters", func(t *testing.T) {
		charset := "ABC"
		gen := str.RandomStringInitialization(charset)
		s := gen(10)
		if len(s) != 10 {
			t.Errorf("Expected length 10, got %d", len(s))
		}
		for _, ch := range s {
			if !strings.ContainsRune(charset, ch) {
				t.Errorf("Character %q not in charset %q, string was %q", string(ch), charset, s)
			}
		}
	})

	t.Run("different charsets produce different character sets", func(t *testing.T) {
		genDigits := str.RandomStringInitialization("0123456789")
		s := genDigits(20)
		matched, _ := regexp.MatchString(`^[0-9]{20}$`, s)
		if !matched {
			t.Errorf("Expected digits-only string, got %q", s)
		}
	})

	t.Run("empty charset uses default alphanumeric", func(t *testing.T) {
		gen := str.RandomStringInitialization("")
		s := gen(10)
		if len(s) != 10 {
			t.Errorf("Expected length 10, got %d", len(s))
		}
		matched, _ := regexp.MatchString(`^[0-9A-Za-z]{10}$`, s)
		if !matched {
			t.Errorf("Expected alphanumeric string from default charset, got %q", s)
		}
	})

	t.Run("zero length returns empty string", func(t *testing.T) {
		gen := str.RandomStringInitialization("ABC")
		s := gen(0)
		if len(s) != 0 {
			t.Errorf("Expected empty string, got %q", s)
		}
	})

	t.Run("single character charset", func(t *testing.T) {
		gen := str.RandomStringInitialization("X")
		s := gen(5)
		if s != "XXXXX" {
			t.Errorf("Expected %q, got %q", "XXXXX", s)
		}
	})

	t.Run("returns a function that can be called multiple times", func(t *testing.T) {
		gen := str.RandomStringInitialization("ABCD")
		s1 := gen(8)
		s2 := gen(8)
		if len(s1) != 8 || len(s2) != 8 {
			t.Errorf("Expected length 8 for both, got %d and %d", len(s1), len(s2))
		}
		// Both should only have ABCD characters
		for _, ch := range s1 {
			if !strings.ContainsRune("ABCD", ch) {
				t.Errorf("s1 contains character %q not in charset", string(ch))
			}
		}
		for _, ch := range s2 {
			if !strings.ContainsRune("ABCD", ch) {
				t.Errorf("s2 contains character %q not in charset", string(ch))
			}
		}
	})

	t.Run("large size", func(t *testing.T) {
		gen := str.RandomStringInitialization("abc123")
		s := gen(1000)
		if len(s) != 1000 {
			t.Errorf("Expected length 1000, got %d", len(s))
		}
		for _, ch := range s {
			if !strings.ContainsRune("abc123", ch) {
				t.Errorf("Character %q not in charset", string(ch))
				break
			}
		}
	})
}
