package function

// CurriedFunc is the result of Curry. Calling it with arguments accumulates them
// until at least arity arguments have been collected, at which point the
// underlying function runs and its result is returned through the value field of
// the returned CurriedFunc. While fewer than arity arguments have been
// collected, Call returns another CurriedFunc that gathers the remaining
// arguments.
//
// Go cannot express the per-position argument types or the variadic call
// chaining that the TypeScript curry overloads provide, so Curry models the
// runtime behavior with a single []any argument list and an explicit arity,
// matching the original which keys off function_.length.
type CurriedFunc struct {
	fn    func(...any) any
	arity int
	args  []any
	// Done reports whether enough arguments have been collected and the
	// underlying function has run.
	Done bool
	// Value holds the underlying function's result once Done is true.
	Value any
}

// Curry curries fn, whose declared arity is given explicitly because Go has no
// runtime equivalent of JavaScript's function_.length.
//
// The returned CurriedFunc collects arguments across successive Call invocations
// (each Call may pass any number of arguments). Once the total number of
// collected arguments reaches arity, fn is invoked with all of them and the
// result is exposed via the Value field with Done set to true.
//
// Example:
//
//	add := func(a ...any) any { return a[0].(int) + a[1].(int) + a[2].(int) }
//	curried := Curry(add, 3)
//	curried.Call(1).Call(2).Call(3).Value // 6
//	curried.Call(1, 2).Call(3).Value      // 6
//	curried.Call(1, 2, 3).Value           // 6
func Curry(fn func(...any) any, arity int) *CurriedFunc {
	return curried(fn, arity, nil)
}

// curried builds a CurriedFunc, immediately invoking fn when the accumulated
// argument count already meets the arity, mirroring the TypeScript recursion in
// which curried checks arguments_.length >= function_.length on each call.
func curried(fn func(...any) any, arity int, args []any) *CurriedFunc {
	if len(args) >= arity {
		return &CurriedFunc{
			fn:    fn,
			arity: arity,
			args:  args,
			Done:  true,
			Value: fn(args...),
		}
	}
	return &CurriedFunc{
		fn:    fn,
		arity: arity,
		args:  args,
	}
}

// Call appends moreArgs to the accumulated arguments and returns the next
// CurriedFunc in the chain. When the combined argument count reaches the arity,
// the returned CurriedFunc has Done set and Value populated with the underlying
// function's result; otherwise it awaits further Call invocations.
//
// Example:
//
//	greet := func(a ...any) any { return a[0].(string) + a[1].(string) }
//	Curry(greet, 2).Call("Hello, ").Call("world").Value // "Hello, world"
func (c *CurriedFunc) Call(moreArgs ...any) *CurriedFunc {
	combined := make([]any, 0, len(c.args)+len(moreArgs))
	combined = append(combined, c.args...)
	combined = append(combined, moreArgs...)
	return curried(c.fn, c.arity, combined)
}
