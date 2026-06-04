// Package predicate provides predicate combinators and nil/pattern checks that
// mirror the TypeScript Predicate module. Predicates are modeled as
// func(args ...any) bool so the combinators preserve the variadic,
// multiple-argument behavior of their TypeScript counterparts.
package predicate

// Every returns a predicate that reports true only when every supplied
// predicate returns true for the given arguments. Evaluation short-circuits on
// the first predicate that returns false, so later predicates are not called.
// With no predicates the returned predicate always reports true.
//
// Example:
//
//	isPositiveEven := Every(
//	    func(a ...any) bool { return a[0].(int) > 0 },
//	    func(a ...any) bool { return a[0].(int)%2 == 0 },
//	)
//	isPositiveEven(4)  // true
//	isPositiveEven(-2) // false
func Every(predicates ...func(args ...any) bool) func(args ...any) bool {
	return func(args ...any) bool {
		for _, predicate := range predicates {
			if !predicate(args...) {
				return false
			}
		}
		return true
	}
}

// Some returns a predicate that reports true when at least one supplied
// predicate returns true for the given arguments. Evaluation short-circuits on
// the first predicate that returns true, so later predicates are not called.
// With no predicates the returned predicate always reports false.
//
// Example:
//
//	isZeroOrNegative := Some(
//	    func(a ...any) bool { return a[0].(int) == 0 },
//	    func(a ...any) bool { return a[0].(int) < 0 },
//	)
//	isZeroOrNegative(0)  // true
//	isZeroOrNegative(-1) // true
//	isZeroOrNegative(1)  // false
func Some(predicates ...func(args ...any) bool) func(args ...any) bool {
	return func(args ...any) bool {
		for _, predicate := range predicates {
			if predicate(args...) {
				return true
			}
		}
		return false
	}
}

// Not returns a predicate that negates the result of fn, forwarding the
// arguments to fn unchanged.
//
// Example:
//
//	isEven := func(a ...any) bool { return a[0].(int)%2 == 0 }
//	isOdd := Not(isEven)
//	isOdd(3) // true
func Not(fn func(args ...any) bool) func(args ...any) bool {
	return func(args ...any) bool {
		return !fn(args...)
	}
}
