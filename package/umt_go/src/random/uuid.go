package random

import "crypto/rand"

const uuidDigits = "0123456789abcdef"

// bytesToUUID renders 16 bytes into the canonical 8-4-4-4-12 hex form.
func bytesToUUID(bytes [16]byte) string {
	var result [36]byte
	pos := 0
	for i := 0; i < 16; i++ {
		if i == 4 || i == 6 || i == 8 || i == 10 {
			result[pos] = '-'
			pos++
		}
		b := bytes[i]
		result[pos] = uuidDigits[b>>4]
		pos++
		result[pos] = uuidDigits[b&0x0f]
		pos++
	}
	return string(result[:])
}

// RandomUUID generates a UUID v4 string in the canonical 8-4-4-4-12 hex form.
// It sources 16 random bytes from crypto/rand and sets the version (4) and
// variant (10xx) bits per RFC 4122.
//
// Example:
//
//	RandomUUID() // e.g. "1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed"
func RandomUUID() string {
	var bytes [16]byte
	_, _ = rand.Read(bytes[:])
	bytes[6] = (bytes[6] & 0x0f) | 0x40
	bytes[8] = (bytes[8] & 0x3f) | 0x80
	return bytesToUUID(bytes)
}
