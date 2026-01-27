package math

import (
	"crypto/rand"
	"time"
)

// UuidV7 generates a UUID v7 (Universally Unique Identifier version 7).
// UUID v7 is time-ordered and contains:
//   - 48 bits of Unix timestamp in milliseconds
//   - 74 bits of random data
//   - 4 bits of version (7)
//   - 2 bits of variant (2)
//
// This implementation follows the UUID v7 draft specification.
//
// Example:
//
//	id := UuidV7() // e.g. "018d6e78-e1e5-7c3c-8bf9-ae5942f2ba1c"
func UuidV7() string {
	const digits = "0123456789abcdef"

	unixTsMs := time.Now().UnixMilli()

	var bytes [16]byte

	// First 6 bytes: Unix timestamp in milliseconds (big-endian)
	for i := 0; i < 6; i++ {
		shift := uint((5 - i) * 8)
		bytes[i] = byte((unixTsMs >> shift) & 0xff)
	}

	// Generate 10 random bytes
	var randomBytes [10]byte
	_, _ = rand.Read(randomBytes[:])

	// Version 7 (0x70) + 4 bits from randomBytes[0]
	bytes[6] = 0x70 | (randomBytes[0] & 0x0f)
	// Random byte
	bytes[7] = randomBytes[1]
	// Variant 2 (0x80) + 6 bits from randomBytes[2]
	bytes[8] = 0x80 | (randomBytes[2] & 0x3f)
	// Remaining random bytes (indices 3..9 -> bytes 9..15)
	copy(bytes[9:], randomBytes[3:])

	var uuid [36]byte
	pos := 0
	for i, b := range bytes {
		uuid[pos] = digits[b>>4]
		pos++
		uuid[pos] = digits[b&0x0f]
		pos++
		if i == 3 || i == 5 || i == 7 || i == 9 {
			uuid[pos] = '-'
			pos++
		}
	}

	return string(uuid[:])
}
