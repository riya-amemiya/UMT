package errutil

// Result is a discriminated union representing either a successful value or an
// error. When Type is "success" the operation succeeded and Value carries the
// result with Error left as the zero value of E. When Type is "error" the
// operation failed, Error carries the failure and Value is the zero value of V.
// This mirrors the TypeScript Result<V, E> union (SuccessType<V> | ErrorType<E>).
type Result[V any, E any] struct {
	Type  string
	Value V
	Error E
}

// SuccessFunction creates a success Result wrapping the given value. It mirrors
// the TypeScript successFunction which returns { type: "success", value }.
//
// Example:
//
//	r := SuccessFunction[int, error](42)
//	// r.Type == "success", r.Value == 42
func SuccessFunction[V any, E any](value V) Result[V, E] {
	return Result[V, E]{Type: "success", Value: value}
}

// ErrorFunction creates an error Result wrapping the given error. It mirrors the
// TypeScript errorFunction which returns { type: "error", error }.
//
// Example:
//
//	r := ErrorFunction[int](errors.New("boom"))
//	// r.Type == "error", r.Error.Error() == "boom"
func ErrorFunction[V any, E any](err E) Result[V, E] {
	return Result[V, E]{Type: "error", Error: err}
}

// SafeExecuteResult runs callback and returns a success Result containing its
// value, or, if the callback panics, an error Result wrapping the recovered
// panic value. It mirrors the TypeScript safeExecute which catches any thrown
// value and wraps it in a Result. The recovered panic value is stored directly
// into Error, matching the TypeScript behavior of preserving the raw thrown
// value (whether an Error, a string, or any other type).
//
// Example:
//
//	r := SafeExecuteResult(func() int { return 42 })
//	// r.Type == "success", r.Value == 42
//
//	r := SafeExecuteResult(func() int { panic("boom") })
//	// r.Type == "error", r.Error == "boom"
func SafeExecuteResult[V any](callback func() V) (result Result[V, any]) {
	defer func() {
		if r := recover(); r != nil {
			result = ErrorFunction[V, any](r)
		}
	}()
	return SuccessFunction[V, any](callback())
}

// MapResult transforms the value inside a successful Result using the provided
// mapping function. If the Result is an error, it is returned unchanged. It
// mirrors the TypeScript mapResult.
//
// Example:
//
//	success := SuccessFunction[int, error](5)
//	MapResult(success, func(n int) int { return n * 2 })
//	// Result{Type: "success", Value: 10}
func MapResult[V any, E any, U any](
	result Result[V, E],
	function func(value V) U,
) Result[U, E] {
	if result.Type == "success" {
		return SuccessFunction[U, E](function(result.Value))
	}
	return Result[U, E]{Type: "error", Error: result.Error}
}

// FlatMapResult transforms the value inside a successful Result using a function
// that itself returns a Result. If the original Result is an error, it is
// returned unchanged. It mirrors the TypeScript flatMapResult.
//
// The error types of the original Result and the mapping function's Result are
// unified into a single type parameter E, matching the TypeScript Result<U, E |
// F> return type collapsing to a single error type when both share it.
//
// Example:
//
//	success := SuccessFunction[int, string](5)
//	FlatMapResult(success, func(n int) Result[int, string] {
//	    return SuccessFunction[int, string](n * 2)
//	})
//	// Result{Type: "success", Value: 10}
func FlatMapResult[V any, E any, U any](
	result Result[V, E],
	function func(value V) Result[U, E],
) Result[U, E] {
	if result.Type == "success" {
		return function(result.Value)
	}
	return Result[U, E]{Type: "error", Error: result.Error}
}

// MatchResult pattern-matches on a Result, calling onSuccess with the value when
// the Result is a success and onError with the error when it is an error,
// returning whichever handler's result. It mirrors the TypeScript matchResult.
//
// Example:
//
//	r := SuccessFunction[int, error](42)
//	MatchResult(r,
//	    func(v int) string { return fmt.Sprintf("Got %d", v) },
//	    func(e error) string { return "Failed: " + e.Error() },
//	)
//	// "Got 42"
func MatchResult[V any, E any, S any](
	result Result[V, E],
	onSuccess func(value V) S,
	onError func(err E) S,
) S {
	if result.Type == "success" {
		return onSuccess(result.Value)
	}
	return onError(result.Error)
}
