package str

import "testing"

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
			result := TrimCharacters(tt.input, tt.chars)
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
			result := TrimStartCharacters(tt.input, tt.chars)
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
			result := TrimEndCharacters(tt.input, tt.chars)
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
			result := PadStart(tt.input, tt.length, tt.pad)
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
			result := PadEnd(tt.input, tt.length, tt.pad)
			if result != tt.expected {
				t.Errorf("PadEnd(%q, %d, %q) = %q, want %q", tt.input, tt.length, tt.pad, result, tt.expected)
			}
		})
	}
}
