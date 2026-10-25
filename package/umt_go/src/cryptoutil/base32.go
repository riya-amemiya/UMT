package cryptoutil

import (
	"fmt"
	"strings"
)

const base32Alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"

var base32DecodeTable = func() (table [256]byte) {
	for i := range table {
		table[i] = 0xFF
	}
	for i := 0; i < len(base32Alphabet); i++ {
		table[base32Alphabet[i]] = byte(i)
	}
	return
}()

// EncodeBase32 encodes a byte slice to a Base32 string (RFC 4648).
func EncodeBase32(data []byte) string {
	var result strings.Builder
	buffer := 0
	bufferLength := 0

	for _, b := range data {
		buffer = (buffer << 8) | int(b)
		bufferLength += 8

		for bufferLength >= 5 {
			bufferLength -= 5
			result.WriteByte(base32Alphabet[(buffer>>bufferLength)&0x1f])
		}
	}

	if bufferLength > 0 {
		result.WriteByte(base32Alphabet[(buffer<<(5-bufferLength))&0x1f])
	}

	// Add padding
	paddingLength := (8 - (result.Len() % 8)) % 8
	for i := 0; i < paddingLength; i++ {
		result.WriteByte('=')
	}

	return result.String()
}

// DecodeBase32 decodes a Base32 string to a byte slice.
func DecodeBase32(input string) ([]byte, error) {
	result := make([]byte, 0, ((len(input)+7)/8)*5)
	buffer := 0
	bufferLength := 0

	for _, ch := range input {
		if ch == '=' {
			continue
		}

		value := byte(0xFF)
		if uint32(ch) < 256 {
			value = base32DecodeTable[ch]
		}
		if value == 0xFF {
			return nil, fmt.Errorf("Invalid base32 character: %c", ch)
		}

		buffer = (buffer << 5) | int(value)
		bufferLength += 5

		if bufferLength >= 8 {
			bufferLength -= 8
			result = append(result, byte((buffer>>bufferLength)&0xff))
		}
	}

	return result, nil
}

// DecodeBase32ToString decodes a Base32 string to a UTF-8 string.
func DecodeBase32ToString(input string) (string, error) {
	data, err := DecodeBase32(input)
	if err != nil {
		return "", err
	}
	return string(data), nil
}
