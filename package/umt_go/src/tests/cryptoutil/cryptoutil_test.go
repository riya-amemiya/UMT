package cryptoutil_test

import (
	"bytes"
	"strings"
	"testing"

	"github.com/riya-amemiya/umt-go/src/cryptoutil"
)

// =============================================================================
// EncodeBase32
// =============================================================================

func TestEncodeBase32Simple(t *testing.T) {
	result := cryptoutil.EncodeBase32([]byte("Hello"))
	if result != "JBSWY3DP" {
		t.Errorf("EncodeBase32(\"Hello\") = %q, want %q", result, "JBSWY3DP")
	}
}

func TestEncodeBase32Empty(t *testing.T) {
	result := cryptoutil.EncodeBase32([]byte(""))
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
			result := cryptoutil.EncodeBase32([]byte(tt.input))
			if result != tt.expected {
				t.Errorf("EncodeBase32(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestEncodeBase32Bytes(t *testing.T) {
	input := []byte{72, 101, 108, 108, 111} // "Hello"
	result := cryptoutil.EncodeBase32(input)
	if result != "JBSWY3DP" {
		t.Errorf("EncodeBase32(Hello bytes) = %q, want %q", result, "JBSWY3DP")
	}
}

func TestEncodeBase32SpecialChars(t *testing.T) {
	result := cryptoutil.EncodeBase32([]byte("\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf")) // "こんにちは"
	if result != "4OAZHY4CSPRYDK7DQGQ6HANP" {
		t.Errorf("EncodeBase32(Japanese) = %q, want %q", result, "4OAZHY4CSPRYDK7DQGQ6HANP")
	}
}

func TestEncodeBase32LongText(t *testing.T) {
	text := "The quick brown fox jumps over the lazy dog"
	expected := "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
	result := cryptoutil.EncodeBase32([]byte(text))
	if result != expected {
		t.Errorf("EncodeBase32(long text) = %q, want %q", result, expected)
	}
}

// =============================================================================
// DecodeBase32
// =============================================================================

func TestDecodeBase32Simple(t *testing.T) {
	result, err := cryptoutil.DecodeBase32("JBSWY3DP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(result) != "Hello" {
		t.Errorf("DecodeBase32(\"JBSWY3DP\") = %q, want %q", string(result), "Hello")
	}
}

func TestDecodeBase32Empty(t *testing.T) {
	result, err := cryptoutil.DecodeBase32("")
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
			result, err := cryptoutil.DecodeBase32(tt.input)
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
			_, err := cryptoutil.DecodeBase32(tt.input)
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
	result, err := cryptoutil.DecodeBase32(encoded)
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
			result, err := cryptoutil.DecodeBase32(tt.input)
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
	result, err := cryptoutil.DecodeBase32ToString("JBSWY3DP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "Hello" {
		t.Errorf("got %q, want %q", result, "Hello")
	}
}

func TestDecodeBase32ToStringEmpty(t *testing.T) {
	result, err := cryptoutil.DecodeBase32ToString("")
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
			result, err := cryptoutil.DecodeBase32ToString(tt.input)
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
	result, err := cryptoutil.DecodeBase32ToString("4OAZHY4CSPRYDK7DQGQ6HANP")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf" // "こんにちは"
	if result != expected {
		t.Errorf("got %q, want Japanese string", result)
	}
}

func TestDecodeBase32ToStringInvalid(t *testing.T) {
	_, err := cryptoutil.DecodeBase32ToString("JBSWY3D@")
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "Invalid base32 character: @") {
		t.Errorf("unexpected error message: %q", err.Error())
	}
}

func TestDecodeBase32ToStringLongText(t *testing.T) {
	encoded := "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO==="
	result, err := cryptoutil.DecodeBase32ToString(encoded)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "The quick brown fox jumps over the lazy dog"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}

// =============================================================================
// Base32 Round-trip
// =============================================================================

func TestBase32RoundTrip(t *testing.T) {
	inputs := []string{"Hello", "", "f", "fo", "foo", "foob", "fooba", "foobar", "Go is great!"}
	for _, input := range inputs {
		encoded := cryptoutil.EncodeBase32([]byte(input))
		decoded, err := cryptoutil.DecodeBase32(encoded)
		if err != nil {
			t.Fatalf("round trip failed for %q: %v", input, err)
		}
		if !bytes.Equal(decoded, []byte(input)) {
			t.Errorf("round trip failed for %q: got %q", input, string(decoded))
		}
	}
}

// =============================================================================
// EncodeBase58
// =============================================================================

func TestEncodeBase58Simple(t *testing.T) {
	result := cryptoutil.EncodeBase58([]byte("Hello"))
	if result != "9Ajdvzr" {
		t.Errorf("EncodeBase58(\"Hello\") = %q, want %q", result, "9Ajdvzr")
	}
}

func TestEncodeBase58Empty(t *testing.T) {
	result := cryptoutil.EncodeBase58([]byte(""))
	if result != "" {
		t.Errorf("EncodeBase58(\"\") = %q, want %q", result, "")
	}
}

func TestEncodeBase58SingleChars(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"a", "2g"},
		{"b", "2h"},
		{"c", "2i"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			result := cryptoutil.EncodeBase58([]byte(tt.input))
			if result != tt.expected {
				t.Errorf("EncodeBase58(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestEncodeBase58Bytes(t *testing.T) {
	input := []byte{72, 101, 108, 108, 111} // "Hello"
	result := cryptoutil.EncodeBase58(input)
	if result != "9Ajdvzr" {
		t.Errorf("EncodeBase58(Hello bytes) = %q, want %q", result, "9Ajdvzr")
	}
}

func TestEncodeBase58LeadingZeros(t *testing.T) {
	input := []byte{0, 0, 72, 101, 108, 108, 111}
	result := cryptoutil.EncodeBase58(input)
	if result != "119Ajdvzr" {
		t.Errorf("EncodeBase58(leading zeros) = %q, want %q", result, "119Ajdvzr")
	}
}

func TestEncodeBase58LongText(t *testing.T) {
	text := "The quick brown fox jumps over the lazy dog"
	expected := "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx"
	result := cryptoutil.EncodeBase58([]byte(text))
	if result != expected {
		t.Errorf("EncodeBase58(long text) = %q, want %q", result, expected)
	}
}

func TestEncodeBase58SpecialChars(t *testing.T) {
	result := cryptoutil.EncodeBase58([]byte("\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf")) // "こんにちは"
	if result != "7NAasPYBzpyEe5hmwr1KL" {
		t.Errorf("EncodeBase58(Japanese) = %q, want %q", result, "7NAasPYBzpyEe5hmwr1KL")
	}
}

func TestEncodeBase58BinaryData(t *testing.T) {
	input := []byte{255, 254, 253, 252, 251}
	result := cryptoutil.EncodeBase58(input)
	if result != "Vt9aq46" {
		t.Errorf("EncodeBase58(binary) = %q, want %q", result, "Vt9aq46")
	}
}

// =============================================================================
// DecodeBase58
// =============================================================================

func TestDecodeBase58Simple(t *testing.T) {
	result, err := cryptoutil.DecodeBase58("9Ajdvzr")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(result) != "Hello" {
		t.Errorf("DecodeBase58(\"9Ajdvzr\") = %q, want %q", string(result), "Hello")
	}
}

func TestDecodeBase58Empty(t *testing.T) {
	result, err := cryptoutil.DecodeBase58("")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(result) != 0 {
		t.Errorf("DecodeBase58(\"\") length = %d, want 0", len(result))
	}
}

func TestDecodeBase58SingleChars(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"2g", "a"},
		{"2h", "b"},
		{"2i", "c"},
	}
	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result, err := cryptoutil.DecodeBase58(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if string(result) != tt.expected {
				t.Errorf("DecodeBase58(%q) = %q, want %q", tt.input, string(result), tt.expected)
			}
		})
	}
}

func TestDecodeBase58LeadingOnes(t *testing.T) {
	result, err := cryptoutil.DecodeBase58("119Ajdvzr")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result[0] != 0 || result[1] != 0 {
		t.Errorf("expected leading zero bytes, got %v", result[:2])
	}
	if string(result[2:]) != "Hello" {
		t.Errorf("trailing data = %q, want %q", string(result[2:]), "Hello")
	}
}

func TestDecodeBase58InvalidChar(t *testing.T) {
	tests := []struct {
		input   string
		badChar string
	}{
		{"9Ajdvz0", "0"},
		{"9AjdvzO", "O"},
		{"9AjdvzI", "I"},
		{"9Ajdvzl", "l"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			_, err := cryptoutil.DecodeBase58(tt.input)
			if err == nil {
				t.Fatalf("expected error for %q", tt.input)
			}
			expected := "Invalid base58 character: " + tt.badChar
			if !strings.Contains(err.Error(), expected) {
				t.Errorf("expected error containing %q, got %q", expected, err.Error())
			}
		})
	}
}

func TestDecodeBase58LongText(t *testing.T) {
	encoded := "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx"
	result, err := cryptoutil.DecodeBase58(encoded)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "The quick brown fox jumps over the lazy dog"
	if string(result) != expected {
		t.Errorf("DecodeBase58(long) = %q, want %q", string(result), expected)
	}
}

func TestDecodeBase58BinaryData(t *testing.T) {
	result, err := cryptoutil.DecodeBase58("Vt9aq46")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := []byte{255, 254, 253, 252, 251}
	if !bytes.Equal(result, expected) {
		t.Errorf("DecodeBase58(binary) = %v, want %v", result, expected)
	}
}

// =============================================================================
// DecodeBase58ToString
// =============================================================================

func TestDecodeBase58ToStringSimple(t *testing.T) {
	result, err := cryptoutil.DecodeBase58ToString("9Ajdvzr")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "Hello" {
		t.Errorf("got %q, want %q", result, "Hello")
	}
}

func TestDecodeBase58ToStringEmpty(t *testing.T) {
	result, err := cryptoutil.DecodeBase58ToString("")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "" {
		t.Errorf("got %q, want %q", result, "")
	}
}

func TestDecodeBase58ToStringSingleChars(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"2g", "a"},
		{"2h", "b"},
		{"2i", "c"},
	}
	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result, err := cryptoutil.DecodeBase58ToString(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("DecodeBase58ToString(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestDecodeBase58ToStringSpecialChars(t *testing.T) {
	result, err := cryptoutil.DecodeBase58ToString("7NAasPYBzpyEe5hmwr1KL")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf" // "こんにちは"
	if result != expected {
		t.Errorf("got %q, want Japanese string", result)
	}
}

func TestDecodeBase58ToStringInvalid(t *testing.T) {
	tests := []struct {
		input   string
		badChar string
	}{
		{"9Ajdvz0", "0"},
		{"9AjdvzO", "O"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			_, err := cryptoutil.DecodeBase58ToString(tt.input)
			if err == nil {
				t.Fatalf("expected error for %q", tt.input)
			}
			expected := "Invalid base58 character: " + tt.badChar
			if !strings.Contains(err.Error(), expected) {
				t.Errorf("expected error containing %q, got %q", expected, err.Error())
			}
		})
	}
}

func TestDecodeBase58ToStringLongText(t *testing.T) {
	encoded := "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx"
	result, err := cryptoutil.DecodeBase58ToString(encoded)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := "The quick brown fox jumps over the lazy dog"
	if result != expected {
		t.Errorf("got %q, want %q", result, expected)
	}
}

// =============================================================================
// Base58 Round-trip
// =============================================================================

func TestBase58RoundTrip(t *testing.T) {
	inputs := []string{"Hello", "a", "b", "c", "Go is great!"}
	for _, input := range inputs {
		encoded := cryptoutil.EncodeBase58([]byte(input))
		decoded, err := cryptoutil.DecodeBase58(encoded)
		if err != nil {
			t.Fatalf("round trip failed for %q: %v", input, err)
		}
		if !bytes.Equal(decoded, []byte(input)) {
			t.Errorf("round trip failed for %q: got %q", input, string(decoded))
		}
	}
}

func TestBase58RoundTripBinary(t *testing.T) {
	inputs := [][]byte{
		{0, 0, 72, 101, 108, 108, 111},
		{255, 254, 253, 252, 251},
		{1, 2, 3, 4, 5},
	}
	for _, input := range inputs {
		encoded := cryptoutil.EncodeBase58(input)
		decoded, err := cryptoutil.DecodeBase58(encoded)
		if err != nil {
			t.Fatalf("round trip failed: %v", err)
		}
		if !bytes.Equal(decoded, input) {
			t.Errorf("round trip failed: %v -> %q -> %v", input, encoded, decoded)
		}
	}
}
