package validate

// UnknownReturnType is the result produced by an Unknown validator. It exposes
// the literal "unknown" tag through the Type field, mirroring the TypeScript
// `UnknownReturnType` (Validate/unknown/core.ts) and the Rust
// `UnknownReturnType` (validate/unknown/core.rs).
type UnknownReturnType struct {
	Validate bool
	Message  string
	Type     string
}

// Unknown creates a validator that accepts any value but reports it under the
// "unknown" tag, keeping callers honest about narrowing before use. The
// returned validator always succeeds, mirroring the TypeScript `unknown()`
// factory and the Rust `umt_unknown` function.
func Unknown[T any]() func(value T) UnknownReturnType {
	return func(_ T) UnknownReturnType {
		return UnknownReturnType{
			Validate: true,
			Message:  "",
			Type:     "unknown",
		}
	}
}
