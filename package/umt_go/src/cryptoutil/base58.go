package cryptoutil

import (
	"fmt"
	"math/big"
	"strings"
)

const base58Alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"

// EncodeBase58 encodes a byte slice to a Base58 string (Bitcoin alphabet).
func EncodeBase58(data []byte) string {
	bigNum := new(big.Int).SetBytes(data)
	zero := big.NewInt(0)
	base := big.NewInt(58)
	mod := new(big.Int)

	var encoded strings.Builder
	for bigNum.Cmp(zero) > 0 {
		bigNum.DivMod(bigNum, base, mod)
		encoded.WriteByte(base58Alphabet[mod.Int64()])
	}

	// Reverse
	result := []byte(encoded.String())
	for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
		result[i], result[j] = result[j], result[i]
	}

	// Count leading zeros
	leadingZeros := 0
	for _, b := range data {
		if b != 0 {
			break
		}
		leadingZeros++
	}

	return strings.Repeat("1", leadingZeros) + string(result)
}

// DecodeBase58 decodes a Base58 string to a byte slice.
func DecodeBase58(input string) ([]byte, error) {
	bigNum := big.NewInt(0)
	base := big.NewInt(58)

	for _, ch := range input {
		value := strings.IndexRune(base58Alphabet, ch)
		if value == -1 {
			return nil, fmt.Errorf("invalid base58 character: %c", ch)
		}
		bigNum.Mul(bigNum, base)
		bigNum.Add(bigNum, big.NewInt(int64(value)))
	}

	bytes := bigNum.Bytes()

	// Count leading '1's (which represent zero bytes)
	leadingOnes := 0
	for _, ch := range input {
		if ch != '1' {
			break
		}
		leadingOnes++
	}

	result := make([]byte, leadingOnes+len(bytes))
	copy(result[leadingOnes:], bytes)

	return result, nil
}

// DecodeBase58ToString decodes a Base58 string to a UTF-8 string.
func DecodeBase58ToString(input string) (string, error) {
	data, err := DecodeBase58(input)
	if err != nil {
		return "", err
	}
	return string(data), nil
}
