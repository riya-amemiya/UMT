package validate

// ValidateString validates a string value directly and applies optional
// validation rules, returning a ValidateCoreReturnType[string]. It mirrors the
// TypeScript `string(option?, message?)` runtime (Validate/string/core.ts),
// which delegates to `core("string")`, and the Rust `umt_validate_string` /
// `umt_string_validator` pair.
//
// Each rule runs in order; the first failing rule returns its message. When
// every rule passes the result is successful with an empty message (matching the
// TypeScript runtime, which clears the message on success even when a custom
// message was supplied to the factory).
func ValidateString(value string, option []ValidateReturnType[string], message string) ValidateCoreReturnType[string] {
	return Core[string]("string")(value, option, message)
}

// StringValidator creates a string validator function, mirroring the TypeScript
// `string(option?, message?)` factory and the Rust `umt_string_validator`.
func StringValidator(option []ValidateReturnType[string], message string) func(value string) ValidateCoreReturnType[string] {
	return func(value string) ValidateCoreReturnType[string] {
		return ValidateString(value, option, message)
	}
}
