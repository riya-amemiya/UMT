package async

import (
	"fmt"
	"time"
)

// Timeout runs the operation in a goroutine and waits up to ms milliseconds for
// it to finish. If the operation completes first, its value and error are
// returned unchanged. If the timeout elapses first, the zero value of T and a
// "Timed out after Nms" error are returned, mirroring the TypeScript timeout
// utility that rejects when the wrapped promise does not settle in time.
//
// The operation goroutine is not cancelled when the timeout fires; its eventual
// result is simply discarded, matching the JavaScript behavior where a timed-out
// promise keeps running.
//
// Example:
//
//	value, err := Timeout(func() (string, error) {
//	    return slowCall(), nil
//	}, 5000)
func Timeout[T any](operation func() (T, error), ms int) (T, error) {
	type outcome struct {
		value T
		err   error
	}
	resultCh := make(chan outcome, 1)
	go func() {
		value, err := operation()
		resultCh <- outcome{value: value, err: err}
	}()

	timer := time.NewTimer(time.Duration(ms) * time.Millisecond)
	defer timer.Stop()

	select {
	case res := <-resultCh:
		return res.value, res.err
	case <-timer.C:
		var zero T
		return zero, fmt.Errorf("Timed out after %dms", ms)
	}
}
