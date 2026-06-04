package validate

// PickKeys returns a new object validator that retains only the given keys of the
// source validator's shape, mirroring the TypeScript `pickKeys()` helper
// (Validate/object/pickKeys.ts) and the Rust `umt_pick_keys` function
// (validate/object/pick_keys.rs). It is the analog of `Pick<T, K>`.
//
// Keys are visited in the supplied order; for each key present in the source
// validator's Shape the matching entry is copied into the new shape, matching the
// TypeScript source which copies `sourceShape[key]` for each requested key. A
// requested key absent from the source shape is skipped. The supplied message is
// forwarded to the new object validator.
func PickKeys(validator ObjectValidatorResult, keys []string, message string) ObjectValidatorResult {
	source := validator.Shape
	next := make(ObjectShape, len(keys))
	for _, key := range keys {
		if property, ok := source[key]; ok {
			next[key] = property
		}
	}
	return ObjectValidator(next, message)
}
