package validate

// ValidateBoolean validates a boolean value directly. A statically-typed Go bool
// is always a valid boolean, so this mirrors the TypeScript `boolean()` runtime
// (Validate/boolean/core.ts, which delegates to `core("boolean")`) and the Rust
// `umt_validate_boolean` function: it always reports the value as valid and
// carries the supplied message on the result.
func ValidateBoolean(value bool, message string) ValidateCoreReturnType[bool] {
	return ValidateCoreReturnType[bool]{
		Validate: true,
		Message:  message,
		Type:     value,
	}
}

// BooleanValidator creates a boolean validator function, mirroring the
// TypeScript `boolean(message?)` factory and the Rust `umt_boolean_validator`.
// The returned validator always succeeds for a statically-typed bool.
func BooleanValidator(message string) func(value bool) ValidateCoreReturnType[bool] {
	return func(value bool) ValidateCoreReturnType[bool] {
		return ValidateBoolean(value, message)
	}
}
