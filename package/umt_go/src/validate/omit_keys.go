package validate

// OmitKeys returns a new object validator that drops the given keys from the
// source validator's shape, mirroring the TypeScript `omitKeys()` helper
// (Validate/object/omitKeys.ts) and the Rust `umt_omit_keys` function
// (validate/object/omit_keys.rs). It is the analog of `Omit<T, K>`.
//
// Every entry of the source validator's Shape whose key is not in keys is copied
// into the new shape, matching the TypeScript source which copies each
// non-omitted entry. Requested keys absent from the source shape have no effect.
// The supplied message is forwarded to the new object validator.
func OmitKeys(validator ObjectValidatorResult, keys []string, message string) ObjectValidatorResult {
	omitted := make(map[string]struct{}, len(keys))
	for _, key := range keys {
		omitted[key] = struct{}{}
	}
	source := validator.Shape
	next := make(ObjectShape, len(source))
	for key, property := range source {
		if _, drop := omitted[key]; !drop {
			next[key] = property
		}
	}
	return ObjectValidator(next, message)
}
