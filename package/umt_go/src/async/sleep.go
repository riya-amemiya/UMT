package async

import "time"

// Sleep blocks for the specified number of milliseconds and then returns.
// It mirrors the TypeScript sleep utility, which returns a promise that
// resolves after the given delay. A non-positive duration returns
// immediately.
//
// Example:
//
//	async.Sleep(1000) // blocks for one second
func Sleep(ms int) {
	if ms <= 0 {
		return
	}
	time.Sleep(time.Duration(ms) * time.Millisecond)
}
