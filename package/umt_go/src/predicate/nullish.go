package predicate

import "reflect"

// IsNullish reports whether value is the Go equivalent of TypeScript's null or
// undefined. A nil interface value is nullish, as is a typed nil pointer,
// slice, map, channel, function, or interface stored in the interface. Every
// other value, including 0, "", false, NaN, an empty struct, or an empty
// non-nil slice, is not nullish — matching the TypeScript predicate that only
// treats null and undefined as nullish.
//
// Example:
//
//	IsNullish(nil) // true
//	IsNullish(0)   // false
//	IsNullish("")  // false
func IsNullish(value any) bool {
	if value == nil {
		return true
	}
	switch reflect.TypeOf(value).Kind() {
	case reflect.Ptr,
		reflect.Slice,
		reflect.Map,
		reflect.Chan,
		reflect.Func,
		reflect.Interface:
		return reflect.ValueOf(value).IsNil()
	default:
		return false
	}
}

// IsNotNullish reports whether value is not nullish, the negation of IsNullish.
// It is the Go equivalent of the TypeScript predicate that narrows away null
// and undefined while keeping falsy values such as 0, "", false, and NaN.
//
// Example:
//
//	IsNotNullish(nil) // false
//	IsNotNullish(0)   // true
//	IsNotNullish("")  // true
func IsNotNullish(value any) bool {
	return !IsNullish(value)
}
