package validate

// Partial returns a new object validator whose every property is wrapped with
// Optional, mirroring the TypeScript `partial()` helper
// (Validate/object/partial.ts) and the Rust `umt_partial` function
// (validate/object/partial.rs). It is the analog of `Partial<T>`: every property
// becomes acceptable when absent.
//
// Each entry of the source validator's Shape is copied; an entry that is already
// optional (IsOptional == true) is kept as-is so the wrapper is idempotent,
// matching the TypeScript source which re-wraps only non-optional entries. The
// supplied message is forwarded to the new object validator.
func Partial(validator ObjectValidatorResult, message string) ObjectValidatorResult {
	source := validator.Shape
	next := make(ObjectShape, len(source))
	for key, property := range source {
		if property.IsOptional {
			next[key] = property
		} else {
			next[key] = Optional(property)
		}
	}
	return ObjectValidator(next, message)
}
