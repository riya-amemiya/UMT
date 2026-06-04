package validate

// NeverReturnType is the result produced by a Never validator. It exposes the
// literal "never" tag through the Type field, mirroring the TypeScript
// `NeverReturnType` (Validate/never/core.ts) and the Rust `never` validator
// whose `type_value` carries the "never" tag (validate/never.rs).
type NeverReturnType struct {
	Validate bool
	Message  string
	Type     string
}

// Never creates a validator that fails for every input, useful for marking
// positions in a schema that must never be filled. The returned validator
// always reports a failing result carrying the supplied message, mirroring the
// TypeScript `never(message?)` factory and the Rust `umt_never` function.
func Never[T any](message string) func(value T) NeverReturnType {
	return func(_ T) NeverReturnType {
		return NeverReturnType{
			Validate: false,
			Message:  message,
			Type:     "never",
		}
	}
}
