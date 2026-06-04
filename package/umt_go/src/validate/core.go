package validate

// ValidateCoreReturnType is the core validation result type returned by every
// validator factory in this package. It mirrors the TypeScript
// `ValidateCoreReturnType<T>` interface (Validate/type.ts) and the Rust
// `ValidateCoreReturnType<T>` struct (validate/core/mod.rs).
//
// Validate reports whether the value passed validation. Message carries the
// failure message ("" on success, mirroring the TS/Rust runtime which clears
// the message on a successful result). Type carries the validated value back to
// the caller so composing helpers (object/union/intersection) can recover it,
// matching the `type` field on the TypeScript return value.
type ValidateCoreReturnType[T any] struct {
	Validate bool
	Message  string
	Type     T
}

// ValidateReturnType describes a single additional validation rule applied by a
// validator factory. It mirrors the TypeScript `ValidateReturnType<T>`
// interface and the Rust `ValidateReturnType<T>` struct: TypeName tags the
// expected type, Validate is the predicate run against the value, and Message
// is the failure message reported when the predicate returns false.
type ValidateReturnType[T any] struct {
	TypeName string
	Validate func(value T) bool
	Message  string
}

// Core creates a validator that applies a sequence of validation rules to a
// value, mirroring the curried TypeScript `core<T>(type)(value, option,
// message)` helper (Validate/core/index.ts) and the Rust `umt_validate_core`
// function (validate/core/mod.rs).
//
// The returned function runs each rule in order; the first rule whose predicate
// fails short-circuits and returns a failing result carrying that rule's
// message. When every rule passes the result is successful with an empty
// message.
//
// The TypeScript original additionally rejects values whose `typeof` differs
// from the expected type tag. In Go the value is statically typed as T, so that
// runtime type mismatch cannot occur; the type tag is retained only for parity
// with the TS/Rust signature and to label the rules. The supplied message is
// used unchanged for the (statically unreachable) type-mismatch branch via the
// direct validator forms, matching how the TS/Rust scalar validators forward it.
func Core[T any](typeName string) func(value T, option []ValidateReturnType[T], message string) ValidateCoreReturnType[T] {
	_ = typeName
	return func(value T, option []ValidateReturnType[T], message string) ValidateCoreReturnType[T] {
		_ = message
		for _, rule := range option {
			if !rule.Validate(value) {
				return ValidateCoreReturnType[T]{
					Validate: false,
					Message:  rule.Message,
					Type:     value,
				}
			}
		}
		return ValidateCoreReturnType[T]{
			Validate: true,
			Message:  "",
			Type:     value,
		}
	}
}
