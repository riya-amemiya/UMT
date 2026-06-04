package validate

// Optional wraps a property validator so it also accepts an absent (undefined)
// value, mirroring the TypeScript `optional()` validator
// (Validate/object/optional.ts) and the Rust `umt_partial` per-property wrapper
// (validate/object/partial.rs), which both let an absent value pass while
// delegating a present value to the inner validator.
//
// The returned PropertyValidator treats a nil incoming value as JavaScript
// `undefined` (the Rust `None`): it is reported valid with an empty message and
// the literal "undefined" tag through the Type field, mirroring the TypeScript
// `UndefinedReturn`. A present (non-nil) value is delegated to the wrapped
// property validator and its result flows through unchanged.
//
// The result carries IsOptional == true and Inner pointing at the wrapped
// validator, reproducing the `isOptional` flag and `inner` reference the
// TypeScript `optional()` attaches to its function so Required can unwrap the
// layer and Partial can stay idempotent (Go cannot attach properties to a bare
// function, so this metadata travels on the PropertyValidator struct).
func Optional(validator PropertyValidator) PropertyValidator {
	inner := validator
	return PropertyValidator{
		Validate: func(value any) ValidateCoreReturnType[any] {
			if value == nil {
				return ValidateCoreReturnType[any]{
					Validate: true,
					Message:  "",
					Type:     "undefined",
				}
			}
			return inner.Validate(value)
		},
		IsOptional: true,
		Inner:      &inner,
	}
}
