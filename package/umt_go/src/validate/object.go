package validate

// PropertyValidator is a per-property validator stored in an ObjectShape. It
// mirrors a TypeScript `ObjectShape` entry (Validate/object/core.ts), which is a
// function `(value: any) => ValidateCoreReturnType<unknown>` that may additionally
// carry the `isOptional` flag and `inner` reference attached by `optional()`.
//
// Validate runs the property check and returns a ValidateCoreReturnType carrying
// the validate flag, the failure message, and a type tag. IsOptional and Inner
// reproduce the metadata the TypeScript `optional()` wrapper attaches to its
// function so Partial and Required can wrap/unwrap a shape entry the same way
// (Go cannot attach properties to a bare function value, so the metadata travels
// in this struct instead). For a non-optional entry IsOptional is false and Inner
// is nil.
//
// The Rust port (validate/object/mod.rs, partial.rs, required.rs) instead changes
// the shape map's value type between `Fn(&V) -> bool` and `Fn(&Option<V>) -> bool`
// to encode optionality, and synthesizes its own message. This Go port follows
// the TypeScript source of truth verified by the *.test.ts files, where each
// property validator returns its own `{validate, message, type}` result and that
// message is what `object()` propagates on failure.
type PropertyValidator struct {
	Validate   func(value any) ValidateCoreReturnType[any]
	IsOptional bool
	Inner      *PropertyValidator
}

// ObjectShape is the map of property name to per-property validator consumed by
// ObjectValidator / ValidateObject, mirroring the TypeScript `ObjectShape`
// interface and the Rust shape map.
type ObjectShape = map[string]PropertyValidator

// ObjectValidatorFunc is the function form of an object validator. It validates a
// `map[string]any` value against an ObjectShape and returns a
// ValidateCoreReturnType carrying the value map back through the Type field.
type ObjectValidatorFunc = func(value map[string]any) ValidateCoreReturnType[map[string]any]

// ObjectValidatorResult pairs an ObjectValidatorFunc with the ObjectShape it was
// built from. The shape is retained (mirroring the `.shape` property attached to
// the TypeScript `object()` return value) so derived helpers such as PickKeys,
// OmitKeys, Partial, and Required can compose new object validators from an
// existing one. ObjectValidator returns this struct so its Validate field is
// callable while Shape stays reachable.
type ObjectValidatorResult struct {
	Shape    ObjectShape
	Validate ObjectValidatorFunc
}

// ValidateObject validates a map against an ObjectShape, mirroring the runtime of
// the TypeScript `object()` validator (Validate/object/core.ts) and the Rust
// `umt_validate_object` function (validate/object/mod.rs).
//
// When shape is nil or empty every value is accepted, matching `object()` with no
// option map. For each property validator in the shape the matching value is
// looked up in the value map (a missing key yields nil, which the property
// validator receives just as the TypeScript runtime passes `undefined`); the
// first property whose validator reports `validate == false` short-circuits and
// the result carries that validator's message. When every property passes the
// result is successful with an empty message. In all cases the original value map
// flows back through the Type field.
//
// The non-dictionary rejection branch of the TypeScript runtime (returning the
// factory message when the input is not a plain object) is represented here by
// the map type itself: a value reaching this function is statically a
// `map[string]any`, so the dictionary check cannot fail at runtime. The supplied
// message is retained for parity with that statically unreachable branch.
func ValidateObject(value map[string]any, shape ObjectShape, message string) ValidateCoreReturnType[map[string]any] {
	_ = message
	for key, property := range shape {
		result := property.Validate(value[key])
		if !result.Validate {
			return ValidateCoreReturnType[map[string]any]{
				Validate: false,
				Message:  result.Message,
				Type:     value,
			}
		}
	}
	return ValidateCoreReturnType[map[string]any]{
		Validate: true,
		Message:  "",
		Type:     value,
	}
}

// ObjectValidator creates an object validator from an ObjectShape, mirroring the
// TypeScript `object(option?, message?)` factory and the Rust object validator
// surface. The returned value exposes the original Shape (so PickKeys, OmitKeys,
// Partial, and Required can rebuild a shape from it) and a Validate function that
// runs ValidateObject against the shape.
func ObjectValidator(shape ObjectShape, message string) ObjectValidatorResult {
	if shape == nil {
		shape = ObjectShape{}
	}
	return ObjectValidatorResult{
		Shape: shape,
		Validate: func(value map[string]any) ValidateCoreReturnType[map[string]any] {
			return ValidateObject(value, shape, message)
		},
	}
}

// Property adapts a scalar/composition validator into a PropertyValidator usable
// inside an ObjectShape. The supplied validator takes a value of type T and
// returns a ValidateCoreReturnType[E]; Property type-asserts each incoming `any`
// value to T before delegating. A value whose dynamic type is not T is reported
// invalid (mirroring the TypeScript runtime where a wrong-typed property value
// fails its validator's `typeof` check), carrying the supplied message. This is
// the bridge that lets typed validators such as StringValidator(...) /
// NumberValidator(...) / a nested ObjectValidator's Validate populate a shape.
func Property[T any, E any](validator func(value T) ValidateCoreReturnType[E], message string) PropertyValidator {
	return PropertyValidator{
		Validate: func(value any) ValidateCoreReturnType[any] {
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
		},
	}
}

// PropertyAny adapts a validator that already accepts an `any` value (such as the
// closures produced by Nullable and Optional) into a PropertyValidator without an
// intermediate type assertion. It is the shape-entry constructor for wrappers
// that handle nil themselves.
func PropertyAny(validator func(value any) ValidateCoreReturnType[any]) PropertyValidator {
	return PropertyValidator{Validate: validator}
}
