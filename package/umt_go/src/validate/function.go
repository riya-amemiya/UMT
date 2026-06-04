package validate

import "errors"

// ValidateFunc validates a callable value directly. It mirrors the TypeScript
// `func()` runtime (Validate/function/core.ts), which only enforces that the
// value is callable, and the Rust `umt_validate_func` function. A statically-
// typed Go func value is callable by construction, so this always reports the
// value as valid and carries it through the Type field.
//
// Argument and return-value contracts can only be checked when the function is
// actually invoked; use FuncImplement to obtain a wrapper that asserts the
// input/output validators on every call.
func ValidateFunc[F any](value F, message string) ValidateCoreReturnType[F] {
	return ValidateCoreReturnType[F]{
		Validate: true,
		Message:  message,
		Type:     value,
	}
}

// FuncImplement wraps a concrete single-argument function with runtime
// validation of its input and output, mirroring the TypeScript
// `validator.implement(fn)` helper (Validate/function/core.ts) and the Rust
// `umt_func_implement` function.
//
// The returned closure validates the argument against input (when non-nil) and
// the return value against output (when non-nil). When a validator rejects a
// value the closure returns an error carrying the validator's message, falling
// back to the same default messages used by the TypeScript and Rust
// implementations ("function input at index 0 failed validation" and
// "function output failed validation"). When no validators are supplied the
// call result is returned untouched.
func FuncImplement[I, O any](
	function func(I) O,
	input *ValidateReturnType[I],
	output *ValidateReturnType[O],
) func(I) (O, error) {
	return func(argument I) (O, error) {
		if input != nil && !input.Validate(argument) {
			message := input.Message
			if message == "" {
				message = "function input at index 0 failed validation"
			}
			var zero O
			return zero, errors.New(message)
		}
		returnValue := function(argument)
		if output != nil && !output.Validate(returnValue) {
			message := output.Message
			if message == "" {
				message = "function output failed validation"
			}
			var zero O
			return zero, errors.New(message)
		}
		return returnValue, nil
	}
}
