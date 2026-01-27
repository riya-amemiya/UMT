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

// Pipeline represents a reusable function pipeline. When called with Get(), it
// returns the current stored value. When called with Then(), it applies a
// transformer function and returns a new Pipeline with the result.
//
// Because Go does not support overloaded call operators or union return types
// like TypeScript, the Pipeline is implemented as a struct with Get and Then
// methods instead of a callable interface.
//
// Example:
//
//	p := CreatePipeline(1)
//	p.Get() // returns 1 (as any)
//	p2 := p.Then(func(v any) any { return v.(int) + 1 })
//	p2.Get() // returns 2 (as any)
//
//	// Chaining:
//	result := CreatePipeline(1).
//	    Then(func(v any) any { return v.(int) + 1 }).
//	    Then(func(v any) any { return v.(int) * 2 }).
//	    Then(func(v any) any { return v.(int) - 1 }).
//	    Get()
//	// result == 3
type Pipeline struct {
	value any
}

// CreatePipeline creates a new Pipeline with the given initial value.
func CreatePipeline(initialValue any) *Pipeline {
	return &Pipeline{value: initialValue}
}

// Get returns the current stored value from the pipeline.
func (p *Pipeline) Get() any {
	return p.value
}

// Then applies the transformer function to the current value and returns a new
// Pipeline containing the result.
func (p *Pipeline) Then(transformer func(any) any) *Pipeline {
	return CreatePipeline(transformer(p.value))
}
