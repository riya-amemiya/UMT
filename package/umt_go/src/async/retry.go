package async

import (
	"context"
	"errors"
	"math/rand"
)

// RetryOptions configures Retry. Zero-value fields fall back to the same
// defaults as the TypeScript retry utility: Retries 3, Delay 1000ms, Backoff
// "fixed", Jitter false, and a ShouldRetry that always returns true.
//
// ShouldRetry receives the error and the zero-based attempt number and decides
// whether another attempt should be made. OnRetry, when set, is invoked with the
// same arguments immediately before each backoff wait. Ctx, when non-nil, aborts
// the retry loop: if it is already cancelled the loop stops before the first
// attempt, returning the explicit cancellation cause when one was supplied (via
// context.WithCancelCause) and a generic "Aborted" error otherwise.
type RetryOptions struct {
	Retries     int
	Delay       int
	Backoff     string
	Jitter      bool
	ShouldRetry func(err error, attempt int) bool
	OnRetry     func(err error, attempt int)
	Ctx         context.Context

	retriesSet bool
	delaySet   bool
}

// WithRetries sets the maximum number of retries (attempts after the first).
func (o RetryOptions) WithRetries(retries int) RetryOptions {
	o.Retries = retries
	o.retriesSet = true
	return o
}

// WithDelay sets the base delay in milliseconds.
func (o RetryOptions) WithDelay(delay int) RetryOptions {
	o.Delay = delay
	o.delaySet = true
	return o
}

func computeDelay(baseDelay, attemptNumber int, backoff string, jitter bool) float64 {
	waitMs := float64(baseDelay)
	if backoff == "linear" {
		waitMs = float64(baseDelay) * float64(attemptNumber)
	} else if backoff == "exponential" {
		waitMs = float64(baseDelay) * float64(int(1)<<(attemptNumber-1))
	}
	if jitter {
		waitMs *= rand.Float64()
	}
	return waitMs
}

// Retry invokes fn until it succeeds or the retry budget is exhausted, returning
// the first successful result. It supports fixed, linear, or exponential
// backoff, optional jitter, a ShouldRetry predicate, an OnRetry callback, and
// context-based cancellation, mirroring the TypeScript retry utility.
//
// Example:
//
//	value, err := Retry(func() (string, error) {
//	    return fetch("/api")
//	}, RetryOptions{Backoff: "exponential", Jitter: true}.WithRetries(5))
func Retry[T any](fn func() (T, error), options ...RetryOptions) (T, error) {
	opts := RetryOptions{}
	if len(options) > 0 {
		opts = options[0]
	}

	retries := 3
	if opts.retriesSet {
		retries = opts.Retries
	}
	delay := 1000
	if opts.delaySet {
		delay = opts.Delay
	}
	backoff := opts.Backoff
	if backoff == "" {
		backoff = "fixed"
	}
	shouldRetry := opts.ShouldRetry
	if shouldRetry == nil {
		shouldRetry = func(error, int) bool { return true }
	}

	attemptNumber := 0
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

		result, err := fn()
		if err == nil {
			return result, nil
		}

		remaining := retries - attemptNumber
		if remaining <= 0 || !shouldRetry(err, attemptNumber) {
			var zero T
			return zero, err
		}
		if opts.OnRetry != nil {
			opts.OnRetry(err, attemptNumber)
		}
		Sleep(int(computeDelay(delay, attemptNumber+1, backoff, opts.Jitter)))
		attemptNumber++
	}
}
