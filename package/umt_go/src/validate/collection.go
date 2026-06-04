package validate

// This file ports the collection / composition validators from the TypeScript
// source of truth (Validate/array/arrayOf.ts, Validate/map/core.ts,
// Validate/set/core.ts) and the Rust port (validate/array/array_of.rs,
// validate/map/mod.rs, validate/set/mod.rs).
//
// Every element / entry validator is modeled as a function returning a
// ValidateCoreReturnType, exactly as in the Rust port (where the per-element
// closure has signature `Fn(&T) -> ValidateCoreReturnType<E>`): only the
// Validate and Message fields are consulted, the carried Type is ignored. This
// mirrors the TypeScript runtime, which casts each element validator down to
// `{ validate: boolean; message: string }` before calling it.
//
// The TypeScript "value is not an array / not a Map / not a Set" guards collapse
// in Go because the argument is statically typed as []E, map[K]V, or a set
// (modeled as map[T]struct{}). The custom message attached to those guards is
// therefore unreachable through the direct/factory entry points below; it is
// retained on the factory signatures for parity with the TS/Rust API and to keep
// the success/failure message handling identical, matching how the Rust port
// keeps (map) or drops (array, set) the message argument.

// ValidateArray validates a slice by applying a single per-element validator to
// every element, mirroring the TypeScript `arrayOf(validator, message?)` runtime
// (Validate/array/arrayOf.ts) and the Rust `umt_array_of` (array/array_of.rs).
//
// The element validator returns a ValidateCoreReturnType[E]; only its Validate
// and Message fields are consulted. Iteration runs in slice order and
// short-circuits at the first element whose Validate is false, propagating that
// element's Message. On success the Message is the empty string. In both cases
// the original slice is returned unchanged through the Type field, matching the
// TypeScript test that asserts `validator(data).type === data`.
//
// Like the Rust port, the TypeScript "not an array" branch has no Go analog: the
// argument is statically typed as []E. The message parameter is accepted for
// parity with the TS/Rust signature; it is unused on this code path because the
// type guard is unreachable.
func ValidateArray[E any](values []E, validator func(value E) ValidateCoreReturnType[E], message string) ValidateCoreReturnType[[]E] {
	_ = message
	for _, value := range values {
		result := validator(value)
		if !result.Validate {
			return ValidateCoreReturnType[[]E]{
				Validate: false,
				Message:  result.Message,
				Type:     values,
			}
		}
	}
	return ValidateCoreReturnType[[]E]{
		Validate: true,
		Message:  "",
		Type:     values,
	}
}

// ArrayOf creates an array validator from a single per-element validator,
// mirroring the TypeScript `arrayOf` factory and the Rust `umt_array_of`. The
// returned function validates a whole []E and surfaces the first failing
// element's message, exactly like ValidateArray.
func ArrayOf[E any](validator func(value E) ValidateCoreReturnType[E], message string) func(values []E) ValidateCoreReturnType[[]E] {
	return func(values []E) ValidateCoreReturnType[[]E] {
		return ValidateArray(values, validator, message)
	}
}

// ArrayValidator is an alias of ArrayOf kept for naming parity with the other
// collection factories (MapValidator, SetValidator). It mirrors the TypeScript
// `arrayOf` factory and the Rust `umt_array_of`.
func ArrayValidator[E any](validator func(value E) ValidateCoreReturnType[E], message string) func(values []E) ValidateCoreReturnType[[]E] {
	return ArrayOf(validator, message)
}

// ValidateMap validates a map with optional per-entry key and value validators,
// mirroring the TypeScript `map(keyValidator?, valueValidator?, message?)`
// runtime (Validate/map/core.ts) and the Rust `umt_validate_map` (map/mod.rs).
//
// Each supplied validator returns a ValidateCoreReturnType; only its Validate
// and Message fields are consulted. Every entry is checked key-first then value,
// short-circuiting at the first failing validator and propagating that
// validator's Message. When every entry passes, the Message is cleared to the
// empty string, matching the TypeScript source of truth (which returns
// `message: ""` on success). Passing a nil validator skips that side, mirroring
// the optional `keyValidator` / `valueValidator` parameters in TypeScript and
// the `Option<KF>` / `Option<VF>` arguments in Rust.
//
// The TypeScript "not a Map" branch has no Go analog (the argument is statically
// typed as map[K]V); the message parameter is retained for parity but is only
// reachable through the never-failing type guard, which Go elides.
//
// Go map iteration order is unspecified, so when several entries would fail the
// surfaced message may correspond to any one of them; the TypeScript and Rust
// ports share this nondeterminism over unordered collections.
func ValidateMap[K comparable, V any](value map[K]V, keyValidator func(key K) ValidateCoreReturnType[K], valueValidator func(value V) ValidateCoreReturnType[V], message string) ValidateCoreReturnType[map[K]V] {
	_ = message
	for entryKey, entryValue := range value {
		if keyValidator != nil {
			keyResult := keyValidator(entryKey)
			if !keyResult.Validate {
				return ValidateCoreReturnType[map[K]V]{
					Validate: false,
					Message:  keyResult.Message,
					Type:     value,
				}
			}
		}
		if valueValidator != nil {
			valueResult := valueValidator(entryValue)
			if !valueResult.Validate {
				return ValidateCoreReturnType[map[K]V]{
					Validate: false,
					Message:  valueResult.Message,
					Type:     value,
				}
			}
		}
	}
	return ValidateCoreReturnType[map[K]V]{
		Validate: true,
		Message:  "",
		Type:     value,
	}
}

// MapValidator creates a map validator function, mirroring the TypeScript `map`
// factory and the Rust `umt_validate_map`. Pass nil for either validator to skip
// validating that side of every entry.
func MapValidator[K comparable, V any](keyValidator func(key K) ValidateCoreReturnType[K], valueValidator func(value V) ValidateCoreReturnType[V], message string) func(value map[K]V) ValidateCoreReturnType[map[K]V] {
	return func(value map[K]V) ValidateCoreReturnType[map[K]V] {
		return ValidateMap(value, keyValidator, valueValidator, message)
	}
}

// ValidateSet validates a set with an optional per-element validator, mirroring
// the TypeScript `setOf(itemValidator?, message?)` runtime (Validate/set/core.ts)
// and the Rust `umt_validate_set` (set/mod.rs).
//
// A Go set is modeled as a map[T]struct{} (the idiomatic Go set), matching the
// Rust port's HashSet. The supplied validator returns a ValidateCoreReturnType;
// only its Validate and Message fields are consulted. When a validator is given,
// every element must satisfy it; iteration short-circuits at the first failure
// and surfaces that element's Message. On success the Message is the empty
// string. Passing a nil validator accepts any set, mirroring the optional
// `itemValidator` parameter in TypeScript and the `Option<F>` argument in Rust.
//
// The TypeScript "not a Set" branch has no Go analog (the argument is statically
// typed as a set); the message parameter is retained for parity. Set iteration
// order is unspecified in Go, so when several elements would fail the surfaced
// message may correspond to any one of them, matching the TypeScript and Rust
// nondeterminism over unordered collections.
func ValidateSet[T comparable](values map[T]struct{}, validator func(value T) ValidateCoreReturnType[T], message string) ValidateCoreReturnType[map[T]struct{}] {
	_ = message
	if validator != nil {
		for value := range values {
			result := validator(value)
			if !result.Validate {
				return ValidateCoreReturnType[map[T]struct{}]{
					Validate: false,
					Message:  result.Message,
					Type:     values,
				}
			}
		}
	}
	return ValidateCoreReturnType[map[T]struct{}]{
		Validate: true,
		Message:  "",
		Type:     values,
	}
}

// SetValidator creates a set validator function, mirroring the TypeScript
// `setOf` factory and the Rust `umt_set_validator`. Pass nil to accept any set
// without validating its elements.
func SetValidator[T comparable](validator func(value T) ValidateCoreReturnType[T], message string) func(values map[T]struct{}) ValidateCoreReturnType[map[T]struct{}] {
	return func(values map[T]struct{}) ValidateCoreReturnType[map[T]struct{}] {
		return ValidateSet(values, validator, message)
	}
}
