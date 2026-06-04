package async

import (
	"context"
	"errors"
	"fmt"
	"time"
)

// WaitForOptions configures WaitFor. Zero-value Timeout falls back to 5000ms and
// zero-value Interval falls back to 100ms, matching the TypeScript waitFor
// defaults. Ctx, when non-nil, aborts polling: if it is already cancelled the
// loop stops before the first check, and an abort during polling stops it as
// soon as the abort is observed.
type WaitForOptions struct {
	Timeout  int
	Interval int
	Ctx      context.Context
}

// WaitFor repeatedly evaluates condition until it reports a ready value or the
// timeout elapses, then returns that value. The condition returns the candidate
// value, an ok flag indicating whether the value is "ready" (the truthy check in
// the TypeScript version), and an error; a non-nil error stops polling and is
// returned immediately, mirroring waitFor rejecting when its predicate throws.
//
// When the timeout elapses before the condition becomes ready, a
// "waitFor timed out after Nms" error is returned. When Ctx is cancelled the
// explicit cancellation cause is returned when one was supplied (via
// context.WithCancelCause) and a generic "Aborted" error otherwise.
//
// Example:
//
//	value, err := WaitFor(func() (int, bool, error) {
//	    n := poll()
//	    return n, n > 0, nil
//	}, WaitForOptions{Interval: 100})
func WaitFor[T any](condition func() (T, bool, error), options ...WaitForOptions) (T, error) {
	opts := WaitForOptions{}
	if len(options) > 0 {
		opts = options[0]
	}
	timeoutMs := 5000
	if opts.Timeout != 0 {
		timeoutMs = opts.Timeout
	}
	interval := 100
	if opts.Interval != 0 {
		interval = opts.Interval
	}

	startTime := time.Now()

	for {
		if opts.Ctx != nil {
			if err := opts.Ctx.Err(); err != nil {
				var zero T
				if cause := context.Cause(opts.Ctx); cause != nil && cause != context.Canceled && cause != context.DeadlineExceeded {
					return zero, cause
				}
				return zero, errors.New("Aborted")
			}
		}

		value, ok, err := condition()
		if err != nil {
			var zero T
			return zero, err
		}
		if ok {
			return value, nil
		}
		if int(time.Since(startTime).Milliseconds()) >= timeoutMs {
			var zero T
			return zero, fmt.Errorf("waitFor timed out after %dms", timeoutMs)
		}
		Sleep(interval)
	}
}
