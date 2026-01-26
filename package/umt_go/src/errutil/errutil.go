package errutil

import (
	"fmt"
	"time"
)

// Retry executes fn up to maxRetries+1 times (1 initial attempt + maxRetries
// retries). If fn returns nil, Retry returns nil immediately. If fn returns an
// error and there are remaining retries, Retry sleeps for delay before the next
// attempt. If all attempts are exhausted, the last error is returned.
//
// This mirrors the TypeScript retry function which retries an async function
// with configurable retry count and delay.
//
// Example:
//
//	err := Retry(func() error {
//	    return doSomethingFlaky()
//	}, 3, time.Second)
func Retry(fn func() error, maxRetries int, delay time.Duration) error {
	var err error
	for attempt := 0; attempt <= maxRetries; attempt++ {
		err = fn()
		if err == nil {
			return nil
		}
		if attempt < maxRetries {
			time.Sleep(delay)
		}
	}
	return err
}

// SafeExecute runs fn and catches any panic that occurs during execution,
// converting it into an error. If fn executes successfully, the result and a
// nil error are returned. If fn panics, a nil result and an error wrapping the
// panic value are returned.
//
// This mirrors the TypeScript safeExecute function which catches exceptions
// thrown by a callback and wraps them in a Result type.
//
// Example:
//
//	result, err := SafeExecute(func() (any, error) {
//	    return riskyOperation()
//	})
func SafeExecute(fn func() (any, error)) (result any, err error) {
	defer func() {
		if r := recover(); r != nil {
			result = nil
			switch v := r.(type) {
			case error:
				err = v
			case string:
				err = fmt.Errorf("%s", v)
			default:
				err = fmt.Errorf("%v", v)
			}
		}
	}()
	return fn()
}
