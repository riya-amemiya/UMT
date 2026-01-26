package tool

import "encoding/json"

// Pipe applies a sequence of functions to an initial value, threading the result
// of each function into the next. This is equivalent to function composition
// applied left-to-right (a pipeline).
//
// Example:
//
//	result := Pipe(1,
//	    func(v any) any { return v.(int) + 1 },
//	    func(v any) any { return v.(int) * 2 },
//	)
//	// result == 4
func Pipe(value any, fns ...func(any) any) any {
	result := value
	for _, fn := range fns {
		result = fn(result)
	}
	return result
}

// ParseJson parses a JSON string into the target value. It is a thin wrapper
// around json.Unmarshal that accepts a string instead of []byte.
//
// The target must be a pointer to the value that should hold the parsed result.
// Returns an error if the JSON string is invalid or cannot be decoded into target.
//
// Example:
//
//	var m map[string]string
//	err := ParseJson(`{"key":"value"}`, &m)
func ParseJson(jsonStr string, target any) error {
	return json.Unmarshal([]byte(jsonStr), target)
}

// Unwrap returns the dereferenced value of a pointer. If the pointer is nil,
// Unwrap panics with the provided message. This mirrors the Rust-style unwrap
// pattern and the TypeScript unwrap utility in the original codebase.
//
// Example:
//
//	v := 42
//	result := Unwrap(&v, "expected a value") // returns 42
//
//	var p *int
//	result := Unwrap(p, "value was nil") // panics with "value was nil"
func Unwrap[T any](value *T, message string) T {
	if value != nil {
		return *value
	}
	panic(message)
}
