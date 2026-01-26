package str

import "testing"

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
			result := DeleteSpaces(tt.input)
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
			result := ReverseString(tt.input)
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
			result := Truncate(tt.input, tt.length, tt.suffix)
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
		Truncate("Hello", -1, "...")
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
			result := HasNoLetters(tt.input)
			if result != tt.expected {
				t.Errorf("HasNoLetters(%q) = %v, want %v", tt.input, result, tt.expected)
			}
		})
	}
}
