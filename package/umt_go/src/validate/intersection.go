package validate

// Intersection creates an intersection validator that passes only when every
// member validator passes (logical AND), mirroring the TypeScript `intersection`
// validator (Validate/object/intersection.ts) and the Rust `umt_intersection`
// function (validate/object/intersection.rs).
//
// Each member validator returns a ValidateCoreReturnType[E]; only its Validate
// and Message fields are consulted. The returned validator runs the members in
// order and returns the first failing member's result (propagating that member's
// message) as soon as one rejects the value. When every member accepts the value
// the result is successful with an empty message. In both cases the original
// value flows back through the Type field.
//
// As in the Rust port, Go's type system requires every member to validate the
// same input type T (the TypeScript original accepts heterogeneous validators
// over a single dynamic value); E is the member result's type-tag parameter and
// is independent of T.
func Intersection[T any, E any](validators ...func(value T) ValidateCoreReturnType[E]) func(value T) ValidateCoreReturnType[T] {
	return func(value T) ValidateCoreReturnType[T] {
		for _, validator := range validators {
			result := validator(value)
			if !result.Validate {
				return ValidateCoreReturnType[T]{
					Validate: false,
					Message:  result.Message,
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
