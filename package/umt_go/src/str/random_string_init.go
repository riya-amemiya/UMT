package str

// RandomStringInitialization returns a function that generates random strings
// of a given size using the provided character set. If charset is empty, the
// default alphanumeric character set is used.
//
// Usage:
//
//	gen := RandomStringInitialization("ABC123")
//	s := gen(10) // produces a 10-character string from "ABC123"
func RandomStringInitialization(charset string) func(size int) string {
	if charset == "" {
		charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
	}
	return func(size int) string {
		return RandomStringWithChars(size, charset)
	}
}
