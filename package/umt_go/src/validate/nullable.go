package validate

// Nullable wraps a validator so it also accepts a null value, mirroring the
// TypeScript `nullable()` validator (Validate/object/nullable.ts) and the Rust
// `umt_nullable` function (validate/object/nullable.rs).
//
// The wrapped validator is an `any`-accepting closure (the uniform shape used by
// every composition wrapper here, produced by Adapt for scalar validators and
// returned directly by Optional, Union, Intersection, and Nullable itself). The
// returned validator also accepts an `any` value so it can sit in an ObjectShape
// via PropertyAny and compose with the other wrappers.
//
// A nil value represents JavaScript `null` (and the Rust `None`): it is reported
// valid with an empty message and the literal "null" tag through the Type field,
// mirroring the TypeScript `NullReturn`. A present (non-nil) value is delegated to
// the wrapped validator and its result flows through unchanged, matching the Rust
// port which forwards the inner result for the `Some` branch.
func Nullable(validator func(value any) ValidateCoreReturnType[any]) func(value any) ValidateCoreReturnType[any] {
	return func(value any) ValidateCoreReturnType[any] {
		if value == nil {
			return ValidateCoreReturnType[any]{
				Validate: true,
				Message:  "",
				Type:     "null",
			}
		}
		return validator(value)
	}
}

// Adapt converts a typed validator (taking a value of type T and returning a
// ValidateCoreReturnType[E]) into the uniform `any`-accepting closure consumed by
// Nullable, Optional, Union, and Intersection. The incoming `any` value is
// type-asserted to T; a value whose dynamic type is not T is reported invalid
// (mirroring the TypeScript runtime where a wrong-typed value fails its
// validator's `typeof` check), carrying the supplied message. On a matching type
// the wrapped result's Validate, Message, and type tag flow through unchanged.
//
// This is the bridge that lets typed validators such as StringValidator(...),
// NumberValidator(...), or a nested object validator's Validate participate in the
// `any`-based composition layer. The typed object validator (ObjectValidatorFunc)
// can be adapted with AdaptObject instead, which preserves its map element type.
func Adapt[T any, E any](validator func(value T) ValidateCoreReturnType[E], message string) func(value any) ValidateCoreReturnType[any] {
	return func(value any) ValidateCoreReturnType[any] {
		typed, ok := value.(T)
		if !ok {
			return ValidateCoreReturnType[any]{
				Validate: false,
				Message:  message,
				Type:     value,
			}
		}
		result := validator(typed)
		return ValidateCoreReturnType[any]{
			Validate: result.Validate,
			Message:  result.Message,
			Type:     result.Type,
		}
	}
}

// AdaptObject converts an object validator's Validate function into the uniform
// `any`-accepting closure. An incoming value is accepted as the object only when
// it is a `map[string]any`; any other dynamic type is reported invalid with the
// supplied message, mirroring the TypeScript object runtime which rejects a
// non-dictionary value. This lets a nested object validator be wrapped by
// Nullable / Optional or composed by Union / Intersection.
func AdaptObject(validator ObjectValidatorFunc, message string) func(value any) ValidateCoreReturnType[any] {
	return func(value any) ValidateCoreReturnType[any] {
		typed, ok := value.(map[string]any)
		if !ok {
			return ValidateCoreReturnType[any]{
				Validate: false,
				Message:  message,
				Type:     value,
			}
		}
		result := validator(typed)
		return ValidateCoreReturnType[any]{
			Validate: result.Validate,
			Message:  result.Message,
			Type:     result.Type,
		}
	}
}
