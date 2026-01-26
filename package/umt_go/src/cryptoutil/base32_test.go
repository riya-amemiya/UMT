package cryptoutil

import (
	"bytes"
	"testing"
)

func TestEncodeBase32(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"Hello", "Hello", "JBSWY3DP"},
		{"Hello World", "Hello World", "JBSWY3DPEBLW64TMMQ======"},
		{"empty string", "", ""},
		{"single char", "a", "ME======"},
		{"test", "test", "ORSXG5A="},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := EncodeBase32([]byte(tt.input))
			if result != tt.expected {
				t.Errorf("EncodeBase32(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestDecodeBase32(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"Hello", "JBSWY3DP", "Hello"},
		{"Hello World", "JBSWY3DPEBLW64TMMQ======", "Hello World"},
		{"empty string", "", ""},
		{"single char", "ME======", "a"},
		{"test", "ORSXG5A=", "test"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := DecodeBase32(tt.input)
			if err != nil {
				t.Fatalf("DecodeBase32(%q) returned error: %v", tt.input, err)
			}
			if string(result) != tt.expected {
				t.Errorf("DecodeBase32(%q) = %q, want %q", tt.input, string(result), tt.expected)
			}
		})
	}
}

func TestDecodeBase32ToString(t *testing.T) {
	result, err := DecodeBase32ToString("JBSWY3DP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "Hello" {
		t.Errorf("DecodeBase32ToString(\"JBSWY3DP\") = %q, want \"Hello\"", result)
	}
}

func TestDecodeBase32InvalidChar(t *testing.T) {
	_, err := DecodeBase32("JBSWY3DP!")
	if err == nil {
		t.Error("expected error for invalid base32 character")
	}
}

func TestBase32RoundTrip(t *testing.T) {
	inputs := []string{"Hello", "Hello World", "", "a", "test", "Go is great!"}
	for _, input := range inputs {
		encoded := EncodeBase32([]byte(input))
		decoded, err := DecodeBase32(encoded)
		if err != nil {
			t.Fatalf("round trip failed for %q: %v", input, err)
		}
		if !bytes.Equal(decoded, []byte(input)) {
			t.Errorf("round trip failed for %q: got %q", input, string(decoded))
		}
	}
}
