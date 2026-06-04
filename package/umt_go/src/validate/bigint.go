package validate

// ValidateBigint validates a bigint value directly and applies optional
// validation rules. It mirrors the TypeScript `bigint(option?, message?)`
// runtime (Validate/bigint/core.ts) and the Rust `umt_validate_bigint` /
// `umt_bigint_validator` pair.
//
// Go has no arbitrary-precision integer in the standard library equivalent to a
// JavaScript bigint or Rust's i128, so int64 is used as the widest native
// integer (mirroring the choice made for the Rust port, which uses i128 because
// it is the widest native Rust integer). Values outside the int64 range cannot
// be represented; this is a limitation of the native-integer choice.
//
// A statically-typed int64 is always a valid integer, so the type-check branch
// of the TypeScript original is unreachable here. Each rule runs in order; the
// first failing rule returns its message. When every rule passes the result is
// successful with an empty message.
func ValidateBigint(value int64, option []ValidateReturnType[int64], message string) ValidateCoreReturnType[int64] {
	_ = message
	for _, rule := range option {
		if !rule.Validate(value) {
			return ValidateCoreReturnType[int64]{
				Validate: false,
				Message:  rule.Message,
				Type:     value,
			}
		}
	}
	return ValidateCoreReturnType[int64]{
		Validate: true,
		Message:  "",
		Type:     value,
	}
}

// BigintValidator creates a bigint validator function, mirroring the TypeScript
// `bigint(option?, message?)` factory and the Rust `umt_bigint_validator`. See
// ValidateBigint for the int64 width limitation.
func BigintValidator(option []ValidateReturnType[int64], message string) func(value int64) ValidateCoreReturnType[int64] {
	return func(value int64) ValidateCoreReturnType[int64] {
		return ValidateBigint(value, option, message)
	}
}
