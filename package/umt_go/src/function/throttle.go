package function

import (
	"sync"
	"time"
)

// ThrottledFunc is the throttled wrapper produced by Throttle. Call invokes the
// wrapped function at most once per wait window, running it immediately when the
// window has elapsed and otherwise scheduling a single trailing invocation with
// the most recent arguments; Cancel discards any pending invocation.
//
// Go cannot express arbitrary argument tuples the way the TypeScript generic
// signature does, so the wrapped function and Call both take a variadic []any.
type ThrottledFunc struct {
	fn   func(args ...any)
	wait time.Duration

	mu           sync.Mutex
	lastCallTime time.Time
	timer        *time.Timer
	lastArgs     []any
	hasLastArgs  bool
}

// Throttle creates a throttled wrapper around fn that invokes it at most once
// per wait milliseconds. The leading call runs immediately; calls within the
// window are coalesced into a single trailing invocation carrying the latest
// arguments, mirroring the TypeScript throttle.
//
// Example:
//
//	throttled := Throttle(func(args ...any) { fmt.Println("called") }, 300)
//	throttled.Call()
//	throttled.Call()
//	throttled.Cancel()
func Throttle(fn func(args ...any), wait int) *ThrottledFunc {
	return &ThrottledFunc{
		fn:   fn,
		wait: time.Duration(wait) * time.Millisecond,
	}
}

// Call invokes the wrapped function immediately when the wait window has elapsed
// since the previous invocation (clearing any pending trailing timer first), or
// otherwise schedules a single trailing invocation for the remaining time. Every
// Call refreshes the stored arguments so the eventual invocation uses the latest
// ones.
//
// Example:
//
//	throttled := Throttle(func(args ...any) { fmt.Println(args[0]) }, 100)
//	throttled.Call(1) // prints 1 immediately
//	throttled.Call(2) // trailing call prints 2 ~100ms later
func (t *ThrottledFunc) Call(args ...any) {
	t.mu.Lock()

	t.lastArgs = args
	t.hasLastArgs = true

	remaining := t.wait - time.Since(t.lastCallTime)

	if remaining <= 0 {
		if t.timer != nil {
			t.timer.Stop()
			t.timer = nil
		}
		t.invoke()
		t.mu.Unlock()
		return
	}

	if t.timer == nil {
		t.timer = time.AfterFunc(remaining, func() {
			t.mu.Lock()
			t.timer = nil
			t.invoke()
			t.mu.Unlock()
		})
	}

	t.mu.Unlock()
}

// invoke runs the wrapped function with the stored arguments, recording the
// invocation time and clearing the stored arguments. The caller must hold t.mu;
// invoke releases the lock around the user function call and reacquires it so
// the function does not run while the mutex is held.
func (t *ThrottledFunc) invoke() {
	t.lastCallTime = time.Now()
	args := t.lastArgs
	t.lastArgs = nil
	t.hasLastArgs = false
	t.mu.Unlock()
	t.fn(args...)
	t.mu.Lock()
}

// Cancel stops any pending trailing invocation and resets the throttle state,
// mirroring the cancel method of the TypeScript throttle. It is safe to call
// when nothing is pending.
//
// Example:
//
//	throttled := Throttle(func(args ...any) {}, 100)
//	throttled.Call()
//	throttled.Cancel()
func (t *ThrottledFunc) Cancel() {
	t.mu.Lock()
	defer t.mu.Unlock()
	if t.timer != nil {
		t.timer.Stop()
	}
	t.timer = nil
	t.lastArgs = nil
	t.hasLastArgs = false
	t.lastCallTime = time.Time{}
}
