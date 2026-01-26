package cryptoutil

import (
	"bytes"
	"testing"
)

func TestEncodeBase58(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"Hello", "Hello", "9Ajdvzr"},
		{"Hello World", "Hello World", "JxF12TrwUP45BMd"},
		{"single char", "a", "2g"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := EncodeBase58([]byte(tt.input))
			if result != tt.expected {
				t.Errorf("EncodeBase58(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestEncodeBase58LeadingZeros(t *testing.T) {
	input := []byte{0, 0, 0, 1}
	result := EncodeBase58(input)
	if len(result) < 3 || result[:3] != "111" {
		t.Errorf("expected leading '1's for zero bytes, got %q", result)
	}
}

func TestDecodeBase58(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"Hello", "9Ajdvzr", "Hello"},
		{"Hello World", "JxF12TrwUP45BMd", "Hello World"},
		{"single char", "2g", "a"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := DecodeBase58(tt.input)
			if err != nil {
				t.Fatalf("DecodeBase58(%q) returned error: %v", tt.input, err)
			}
			if string(result) != tt.expected {
				t.Errorf("DecodeBase58(%q) = %q, want %q", tt.input, string(result), tt.expected)
			}
		})
	}
}

func TestDecodeBase58ToString(t *testing.T) {
	result, err := DecodeBase58ToString("9Ajdvzr")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result != "Hello" {
		t.Errorf("DecodeBase58ToString(\"9Ajdvzr\") = %q, want \"Hello\"", result)
	}
}

func TestDecodeBase58InvalidChar(t *testing.T) {
	_, err := DecodeBase58("0OIl") // 0, O, I, l are not in Base58 alphabet
	if err == nil {
		t.Error("expected error for invalid base58 character")
	}
}

func TestBase58RoundTrip(t *testing.T) {
	inputs := []string{"Hello", "Hello World", "a", "Go is great!"}
	for _, input := range inputs {
		encoded := EncodeBase58([]byte(input))
		decoded, err := DecodeBase58(encoded)
		if err != nil {
			t.Fatalf("round trip failed for %q: %v", input, err)
		}
		if !bytes.Equal(decoded, []byte(input)) {
			t.Errorf("round trip failed for %q: got %q", input, string(decoded))
		}
	}
}
