package str

import "testing"

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
			result := ToBase64(tt.input)
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
			result, err := FromBase64(tt.input)
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
			"\U0001F30A\U0001F30D\U0001F30E", // emojis
		}
		for _, s := range roundtrips {
			encoded := ToBase64(s)
			decoded, err := FromBase64(encoded)
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
			result := ToHalfWidth(tt.input)
			if result != tt.expected {
				t.Errorf("ToHalfWidth(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}
