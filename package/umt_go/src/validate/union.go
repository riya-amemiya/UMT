package validate

// Union creates a union validator that passes when any member validator passes
// (logical OR), mirroring the TypeScript `union` validator
// (Validate/object/union.ts) and the Rust `umt_union` function
// (validate/object/union.rs).
//
// Each member validator returns a ValidateCoreReturnType[E]; only its Validate
// and Message fields are consulted. The returned validator runs the members in
// order and returns a successful result with an empty message as soon as one
// member accepts the value. When no member accepts it, the result is a failure
// carrying the last member's message. In both cases the original value flows back
// through the Type field.
//
// As in the Rust port, Go's type system requires every member to validate the
// same input type T (the TypeScript original accepts heterogeneous validators
// such as `union(string(), number())` over a single dynamic value); E is the
// member result's type-tag parameter and is independent of T.
func Union[T any, E any](validators ...func(value T) ValidateCoreReturnType[E]) func(value T) ValidateCoreReturnType[T] {
	return func(value T) ValidateCoreReturnType[T] {
		lastMessage := ""
		for _, validator := range validators {
			result := validator(value)
			if result.Validate {
				return ValidateCoreReturnType[T]{
					Validate: true,
					Message:  "",
					Type:     value,
				}
			}
			lastMessage = result.Message
		}
		return ValidateCoreReturnType[T]{
			Validate: false,
			Message:  lastMessage,
			Type:     value,
		}
	}
}
