package function

import (
	"sync"
	"time"
)

// DebouncedFunc is the debounced wrapper produced by Debounce. Call schedules an
// invocation that fires only after the wait window has elapsed since the most
// recent Call; Cancel discards any pending invocation.
//
// Go cannot express arbitrary argument tuples the way the TypeScript generic
// signature does, so the wrapped function and Call both take a variadic []any.
type DebouncedFunc struct {
	fn       func(args ...any)
	wait     time.Duration
	leading  bool
	trailing bool

	mu           sync.Mutex
	timer        *time.Timer
	lastArgs     []any
	hasLastArgs  bool
	lastCallTime time.Time
}

// DebounceOption configures a DebouncedFunc.
type DebounceOption func(*DebouncedFunc)

// WithLeading sets whether the wrapped function is invoked on the leading edge
// of the wait window, mirroring the leading option of the TypeScript debounce.
// Leading defaults to false.
func WithLeading(leading bool) DebounceOption {
	return func(d *DebouncedFunc) {
		d.leading = leading
	}
}

// WithTrailing sets whether the wrapped function is invoked on the trailing edge
// of the wait window, mirroring the trailing option of the TypeScript debounce.
// Trailing defaults to true.
func WithTrailing(trailing bool) DebounceOption {
	return func(d *DebouncedFunc) {
		d.trailing = trailing
	}
}

// Debounce creates a debounced wrapper around fn that delays invoking it until
// wait milliseconds have elapsed since the last Call. By default it invokes on
// the trailing edge only; use WithLeading and WithTrailing to change the edges.
//
// Example:
//
//	debounced := Debounce(func(args ...any) { fmt.Println("called") }, 300)
//	debounced.Call()
//	debounced.Call()
//	debounced.Cancel()
func Debounce(fn func(args ...any), wait int, options ...DebounceOption) *DebouncedFunc {
	d := &DebouncedFunc{
		fn:       fn,
		wait:     time.Duration(wait) * time.Millisecond,
		leading:  false,
		trailing: true,
	}
	for _, option := range options {
		option(d)
	}
	return d
}

// Call records the latest arguments and call time. On the first call of a window
// it optionally fires the leading invocation and starts the trailing timer.
// Subsequent calls within the window only refresh the arguments and call time,
// so the trailing invocation uses the most recent arguments once the window
// finally elapses.
//
// Example:
//
//	debounced := Debounce(func(args ...any) { fmt.Println(args[0]) }, 100)
//	debounced.Call(1)
//	debounced.Call(2)
//	debounced.Call(3) // only 3 is printed, ~100ms later
func (d *DebouncedFunc) Call(args ...any) {
	d.mu.Lock()

	d.lastArgs = args
	d.hasLastArgs = true
	d.lastCallTime = time.Now()

	isFirstCall := d.timer == nil

	if d.leading && isFirstCall {
		leadingArgs := d.lastArgs
		d.lastArgs = nil
		d.hasLastArgs = false
		d.mu.Unlock()
		d.fn(leadingArgs...)
		d.mu.Lock()
	}

	if isFirstCall {
		d.scheduleCheck()
	}

	d.mu.Unlock()
}

// scheduleCheck arms a timer for the time remaining in the current window. When
// it fires, if the window has elapsed it clears the timer and runs the trailing
// invocation (when enabled and arguments are pending); otherwise it reschedules
// for the new remaining time, mirroring the recursive scheduleCheck of the
// TypeScript debounce. The caller must hold d.mu.
func (d *DebouncedFunc) scheduleCheck() {
	remaining := d.wait - time.Since(d.lastCallTime)
	d.timer = time.AfterFunc(remaining, func() {
		d.mu.Lock()
		if time.Since(d.lastCallTime) >= d.wait {
			d.timer = nil
			if d.trailing && d.hasLastArgs {
				trailingArgs := d.lastArgs
				d.lastArgs = nil
				d.hasLastArgs = false
				d.mu.Unlock()
				d.fn(trailingArgs...)
				return
			}
			d.mu.Unlock()
			return
		}
		d.scheduleCheck()
		d.mu.Unlock()
	})
}

// Cancel stops any pending invocation and clears the recorded arguments and call
// time, mirroring the cancel method of the TypeScript debounce. It is safe to
// call when nothing is pending.
//
// Example:
//
//	debounced := Debounce(func(args ...any) {}, 100)
//	debounced.Call()
//	debounced.Cancel() // pending trailing invocation is discarded
func (d *DebouncedFunc) Cancel() {
	d.mu.Lock()
	defer d.mu.Unlock()
	if d.timer != nil {
		d.timer.Stop()
	}
	d.timer = nil
	d.lastArgs = nil
	d.hasLastArgs = false
	d.lastCallTime = time.Time{}
}
