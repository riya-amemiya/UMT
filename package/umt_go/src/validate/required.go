package validate

// Required returns a new object validator with every Optional layer stripped from
// its properties, mirroring the TypeScript `required()` helper
// (Validate/object/required.ts) and the Rust `umt_required` function
// (validate/object/required.rs). It is the inverse of Partial and the analog of
// `Required<T>`: every property requires a present value again.
//
// Each entry of the source validator's Shape is copied; an optional entry
// (IsOptional == true with a non-nil Inner) is replaced by its inner validator,
// undoing a Partial wrapping exactly, while non-optional entries are kept as-is.
// The supplied message is forwarded to the new object validator.
func Required(validator ObjectValidatorResult, message string) ObjectValidatorResult {
	source := validator.Shape
	next := make(ObjectShape, len(source))
	for key, property := range source {
		if property.IsOptional && property.Inner != nil {
			next[key] = *property.Inner
		} else {
			next[key] = property
		}
	}
	return ObjectValidator(next, message)
}
