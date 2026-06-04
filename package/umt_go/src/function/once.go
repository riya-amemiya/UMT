package function

import "sync"

// Once returns a function restricted to invoking fn only on its first call.
// Every subsequent call ignores its arguments and returns the result captured
// from the first invocation, mirroring the TypeScript once utility.
//
// The returned function is safe for concurrent use; concurrent first calls run
// fn exactly once and all observe the same result.
//
// Example:
//
//	count := 0
//	initialize := Once(func(a ...any) int {
//		count++
//		return 42
//	})
//	initialize()      // 42, runs fn (count == 1)
//	initialize()      // 42, fn not run again (count still 1)
func Once[R any](fn func(args ...any) R) func(args ...any) R {
	var (
		once   sync.Once
		result R
	)
	return func(args ...any) R {
		once.Do(func() {
			result = fn(args...)
		})
		return result
	}
}
