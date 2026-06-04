package validate

// ValidateNumber validates a number value directly and applies optional
// validation rules, returning a ValidateCoreReturnType[float64]. It mirrors the
// TypeScript `number(option?, message?)` runtime (Validate/number/core.ts),
// which delegates to `core("number")`, and the Rust `umt_validate_number` /
// `umt_number_validator` pair.
//
// Following the TypeScript source of truth, the base validator does not reject
// non-finite values such as NaN (in JavaScript `typeof NaN === "number"`, so it
// passes the type check). Each rule runs in order; the first failing rule
// returns its message. When every rule passes the result is successful with an
// empty message.
func ValidateNumber(value float64, option []ValidateReturnType[float64], message string) ValidateCoreReturnType[float64] {
	return Core[float64]("number")(value, option, message)
}

// NumberValidator creates a number validator function, mirroring the TypeScript
// `number(option?, message?)` factory and the Rust `umt_number_validator`.
func NumberValidator(option []ValidateReturnType[float64], message string) func(value float64) ValidateCoreReturnType[float64] {
	return func(value float64) ValidateCoreReturnType[float64] {
		return ValidateNumber(value, option, message)
	}
}
