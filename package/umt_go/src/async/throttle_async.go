package async

import (
	"errors"
	"sync"
	"time"
)

// ThrottleAsyncFunc is a throttled async wrapper produced by ThrottleAsync. Call
// invokes the wrapper with an argument and returns a future for the result;
// Cancel clears the in-flight tracking and the lockout window.
//
// Concurrent calls while an invocation is in flight all receive the very same
// future, so the underlying function runs only once per window. After an
// invocation settles, calls made before the wait window elapses are rejected
// with "throttleAsync window not elapsed". This mirrors the TypeScript
// throttleAsync utility. Because Go generics cannot express arbitrary argument
// tuples, the wrapper takes a single argument of type A instead of a variadic
// list.
type ThrottleAsyncFunc[A any, R any] struct {
	fn   func(A) (R, error)
	wait int

	mu          sync.Mutex
	inflight    *Deferred[R]
	lockedUntil time.Time
}

// ThrottleAsync creates a throttled wrapper around fn with the given wait window
// in milliseconds.
//
// Example:
//
//	fetchUser := ThrottleAsync(loadUser, 1000)
//	value, err := fetchUser.Call(id).Await()
func ThrottleAsync[A any, R any](fn func(A) (R, error), wait int) *ThrottleAsyncFunc[A, R] {
	return &ThrottleAsyncFunc[A, R]{fn: fn, wait: wait}
}

// Call returns the shared in-flight future when one exists, a rejected future
// when the lockout window has not elapsed, or otherwise starts a new invocation
// and returns its future.
func (t *ThrottleAsyncFunc[A, R]) Call(arg A) *Deferred[R] {
	t.mu.Lock()

	if t.inflight != nil {
		inflight := t.inflight
		t.mu.Unlock()
		return inflight
	}
	if time.Now().Before(t.lockedUntil) {
		t.mu.Unlock()
		rejected := NewDeferred[R]()
		rejected.Reject(errors.New("throttleAsync window not elapsed"))
		return rejected
	}

	current := NewDeferred[R]()
	t.inflight = current
	t.mu.Unlock()

	go func() {
		value, err := t.fn(arg)

		t.mu.Lock()
		if t.inflight == current {
			t.inflight = nil
			t.lockedUntil = time.Now().Add(time.Duration(t.wait) * time.Millisecond)
		}
		t.mu.Unlock()

		if err != nil {
			current.Reject(err)
		} else {
			current.Resolve(value)
		}
	}()

	return current
}

// Cancel clears the in-flight tracking and resets the lockout window, allowing a
// new invocation immediately. An invocation already running keeps executing; its
// settlement no longer affects the lockout because the in-flight identity check
// fails after cancellation.
func (t *ThrottleAsyncFunc[A, R]) Cancel() {
	t.mu.Lock()
	t.inflight = nil
	t.lockedUntil = time.Time{}
	t.mu.Unlock()
}
