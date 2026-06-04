package async

import (
	"errors"
	"sync"
	"time"
)

// DebounceAsyncFunc is a debounced async wrapper produced by DebounceAsync. Call
// invokes the wrapper with an argument and returns a future for the eventual
// result; Cancel rejects every caller still waiting on the current debounce
// window.
//
// Calls within the wait window reset the timer and share a single underlying
// invocation, with the most recent argument winning, mirroring the TypeScript
// debounceAsync utility. Because Go generics cannot express arbitrary argument
// tuples, the wrapper takes a single argument of type A instead of a variadic
// list.
type DebounceAsyncFunc[A any, R any] struct {
	fn   func(A) (R, error)
	wait int

	mu        sync.Mutex
	timer     *time.Timer
	pending   []*Deferred[R]
	latestArg A
}

// DebounceAsync creates a debounced wrapper around fn with the given wait window
// in milliseconds.
//
// Example:
//
//	search := DebounceAsync(query, 300)
//	value, err := search.Call("foo").Await()
func DebounceAsync[A any, R any](fn func(A) (R, error), wait int) *DebounceAsyncFunc[A, R] {
	return &DebounceAsyncFunc[A, R]{fn: fn, wait: wait}
}

// Call schedules an invocation, resetting the debounce timer, and returns a
// Deferred that settles with the shared result once the timer fires.
func (d *DebounceAsyncFunc[A, R]) Call(arg A) *Deferred[R] {
	d.mu.Lock()
	defer d.mu.Unlock()

	if d.timer != nil {
		d.timer.Stop()
	}

	deferred := NewDeferred[R]()
	d.pending = append(d.pending, deferred)
	d.latestArg = arg

	d.timer = time.AfterFunc(time.Duration(d.wait)*time.Millisecond, func() {
		d.mu.Lock()
		d.timer = nil
		resolvers := d.pending
		d.pending = nil
		arg := d.latestArg
		d.mu.Unlock()

		value, err := d.fn(arg)
		for _, resolver := range resolvers {
			if err != nil {
				resolver.Reject(err)
			} else {
				resolver.Resolve(value)
			}
		}
	})

	return deferred
}

// Cancel stops any pending timer and rejects every caller still waiting on the
// current debounce window with a "debounceAsync cancelled" error. It is a no-op
// when nothing is pending.
func (d *DebounceAsyncFunc[A, R]) Cancel() {
	d.mu.Lock()
	if d.timer != nil {
		d.timer.Stop()
		d.timer = nil
	}
	rejecters := d.pending
	d.pending = nil
	d.mu.Unlock()

	for _, rejecter := range rejecters {
		rejecter.Reject(errors.New("debounceAsync cancelled"))
	}
}
