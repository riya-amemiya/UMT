package validate

// AnyReturnType is the result produced by an Any validator. It exposes the
// literal "any" tag through the Type field so downstream helpers can map it back
// to an open type, mirroring the TypeScript `AnyReturnType` (Validate/any/core.ts)
// and the Rust `AnyReturnType` (validate/any/core.rs).
type AnyReturnType struct {
	Validate bool
	Message  string
	Type     string
}

// Any creates a validator that accepts any value. The returned validator always
// succeeds, mirroring the TypeScript `any()` factory and the Rust `umt_any`
// function. The value parameter is generic so the validator can sit in any
// position of a schema while still participating in compositional helpers.
func Any[T any]() func(value T) AnyReturnType {
	return func(_ T) AnyReturnType {
		return AnyReturnType{
			Validate: true,
			Message:  "",
			Type:     "any",
		}
	}
}
