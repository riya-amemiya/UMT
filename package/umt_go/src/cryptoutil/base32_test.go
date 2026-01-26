package cryptoutil

import (
	"bytes"
	"strings"
	"testing"
)

// =============================================================================
// EncodeBase32
// =============================================================================

func TestEncodeBase32Simple(t *testing.T) {
	result := EncodeBase32([]byte("Hello"))
	if result != "JBSWY3DP" {
		t.Errorf("EncodeBase32(\"Hello\") = %q, want %q", result, "JBSWY3DP")
	}
}

func TestEncodeBase32Empty(t *testing.T) {
	result := EncodeBase32([]byte(""))
	if result != "" {
		t.Errorf("EncodeBase32(\"\") = %q, want %q", result, "")
	}
}

func TestEncodeBase32Padding(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"f", "MY======"},
		{"fo", "MZXQ===="},
		{"foo", "MZXW6==="},
		{"foob", "MZXW6YQ="},
		{"fooba", "MZXW6YTB"},
		{"foobar", "MZXW6YTBOI======"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			result := EncodeBase32([]byte(tt.input))
			if result != tt.expected {
				t.Errorf("EncodeBase32(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestEncodeBase32Bytes(t *testing.T) {
	input := []byte{72, 101, 108, 108, 111} // "Hello"
	result := EncodeBase32(input)
	if result != "JBSWY3DP" {
		t.Errorf("EncodeBase32(Hello bytes) = %q, want %q", result, "JBSWY3DP")
	}
}

func TestEncodeBase32SpecialChars(t *testing.T) {
	result := EncodeBase32([]byte("\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf")) // "こんにちは"
	if result != "4OAZHY4CSPRYDK7DQGQ6HANP" {
		t.Errorf("EncodeBase32(Japanese) = %q, want %q", result, "4OAZHY4CSPRYDK7DQGQ6HANP")
	}
}

func TestEncodeBase32LongText(t *testing.T) {
	text := "The quick brown fox jumps over the lazy dog"
	expected := "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
	result := EncodeBase32([]byte(text))
	if result != expected {
		t.Errorf("EncodeBase32(long text) = %q, want %q", result, expected)
	}
}

// =============================================================================
// DecodeBase32
// =============================================================================

func TestDecodeBase32Simple(t *testing.T) {
	result, err := DecodeBase32("JBSWY3DP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(result) != "Hello" {
		t.Errorf("DecodeBase32(\"JBSWY3DP\") = %q, want %q", string(result), "Hello")
	}
}

func TestDecodeBase32Empty(t *testing.T) {
	result, err := DecodeBase32("")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(result) != 0 {
		t.Errorf("DecodeBase32(\"\") length = %d, want 0", len(result))
	}
}

func TestDecodeBase32WithPadding(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"MY======", "f"},
		{"MZXQ====", "fo"},
		{"MZXW6===", "foo"},
		{"MZXW6YQ=", "foob"},
		{"MZXW6YTB", "fooba"},
		{"MZXW6YTBOI======", "foobar"},
	}
	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result, err := DecodeBase32(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if string(result) != tt.expected {
				t.Errorf("DecodeBase32(%q) = %q, want %q", tt.input, string(result), tt.expected)
			}
		})
	}
}

func TestDecodeBase32InvalidChar(t *testing.T) {
	tests := []struct {
		input   string
		badChar string
	}{
		{"JBSWY3D@", "@"},
		{"JBSWY3D1", "1"},
		{"JBSWY3D0", "0"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			_, err := DecodeBase32(tt.input)
			if err == nil {
				t.Fatalf("expected error for %q", tt.input)
			}
			expected := "Invalid base32 character: " + tt.badChar
			if !strings.Contains(err.Error(), expected) {
				t.Errorf("expected error containing %q, got %q", expected, err.Error())
			}
		})
	}
}

func TestDecodeBase32LongText(t *testing.T) {
	encoded := "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
	result, err := DecodeBase32(encoded)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "The quick brown fox jumps over the lazy dog"
	if string(result) != expected {
		t.Errorf("DecodeBase32(long) = %q, want %q", string(result), expected)
	}
}

func TestDecodeBase32WithoutPadding(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"JBSWY3DP", "Hello"},
		{"MZXW6YTB", "fooba"},
	}
	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result, err := DecodeBase32(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if string(result) != tt.expected {
				t.Errorf("DecodeBase32(%q) = %q, want %q", tt.input, string(result), tt.expected)
			}
		})
	}
}

// =============================================================================
// DecodeBase32ToString
// =============================================================================

func TestDecodeBase32ToStringSimple(t *testing.T) {
	result, err := DecodeBase32ToString("JBSWY3DP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "Hello" {
		t.Errorf("got %q, want %q", result, "Hello")
	}
}

func TestDecodeBase32ToStringEmpty(t *testing.T) {
	result, err := DecodeBase32ToString("")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "" {
		t.Errorf("got %q, want %q", result, "")
	}
}

func TestDecodeBase32ToStringPadding(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"MY======", "f"},
		{"MZXQ====", "fo"},
		{"MZXW6===", "foo"},
		{"MZXW6YQ=", "foob"},
		{"MZXW6YTB", "fooba"},
		{"MZXW6YTBOI======", "foobar"},
	}
	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result, err := DecodeBase32ToString(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("DecodeBase32ToString(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestDecodeBase32ToStringSpecialChars(t *testing.T) {
	result, err := DecodeBase32ToString("4OAZHY4CSPRYDK7DQGQ6HANP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf" // "こんにちは"
	if result != expected {
		t.Errorf("got %q, want Japanese string", result)
	}
}

func TestDecodeBase32ToStringInvalid(t *testing.T) {
	_, err := DecodeBase32ToString("JBSWY3D@")
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "Invalid base32 character: @") {
		t.Errorf("unexpected error message: %q", err.Error())
	}
}

func TestDecodeBase32ToStringLongText(t *testing.T) {
	encoded := "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
	result, err := DecodeBase32ToString(encoded)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "The quick brown fox jumps over the lazy dog"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}

// =============================================================================
// Round-trip
// =============================================================================

func TestBase32RoundTrip(t *testing.T) {
	inputs := []string{"Hello", "", "f", "fo", "foo", "foob", "fooba", "foobar", "Go is great!"}
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
