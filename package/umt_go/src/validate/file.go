package validate

// ValidateFile validates file content represented as a byte slice. It mirrors
// the TypeScript `file(message?)` runtime (Validate/file/core.ts), which checks
// `value instanceof File || value instanceof Blob`, and the Rust
// `umt_validate_file` / `umt_file_validator` pair.
//
// Go has no File / Blob globals, so file content is represented as []byte. Any
// byte slice is structurally valid file content, so this always reports the
// value as valid (mirroring how the TypeScript validator accepts every File and
// the Rust port accepts every Vec<u8>), carrying the supplied message.
func ValidateFile(value []byte, message string) ValidateCoreReturnType[[]byte] {
	return ValidateCoreReturnType[[]byte]{
		Validate: true,
		Message:  message,
		Type:     value,
	}
}

// FileValidator creates a file validator function, mirroring the TypeScript
// `file(message?)` factory and the Rust `umt_file_validator`. The returned
// validator always reports the byte slice as valid.
func FileValidator(message string) func(value []byte) ValidateCoreReturnType[[]byte] {
	return func(value []byte) ValidateCoreReturnType[[]byte] {
		return ValidateFile(value, message)
	}
}
