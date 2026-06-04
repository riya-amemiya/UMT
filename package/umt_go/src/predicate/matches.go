package predicate

import "reflect"

// Matches returns a predicate that reports whether an object contains every key
// of pattern with a value equal to the pattern's value. Values are compared
// with reflect.DeepEqual, the Go analog of the TypeScript strict-equality check
// (===): primitives compare by value and a missing key fails the match, since
// an absent key is not equal to any present pattern value.
//
// Go collapses TypeScript's distinct null and undefined into a single nil, so a
// pattern value of nil matches a present nil value; a key that is absent from
// the object always fails because the key must be present to be compared.
//
// An empty pattern always matches.
//
// Example:
//
//	isAdmin := Matches(map[string]any{"role": "admin"})
//	isAdmin(map[string]any{"name": "Alice", "role": "admin"}) // true
//	isAdmin(map[string]any{"name": "Bob", "role": "user"})    // false
func Matches(pattern map[string]any) func(object map[string]any) bool {
	return func(object map[string]any) bool {
		for key, want := range pattern {
			got, ok := object[key]
			if !ok {
				return false
			}
			if !reflect.DeepEqual(got, want) {
				return false
			}
		}
		return true
	}
}
