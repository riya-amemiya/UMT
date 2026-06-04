//go:build js && wasm

package validate

import "syscall/js"

// IsNode reports whether the current environment is Node.js.
//
// It mirrors the TypeScript original, which returns true only when both the
// `process` and `require` globals are defined. Under js/wasm the program runs
// inside a JavaScript host, so it probes the global object for those two
// bindings; a non-Node.js JavaScript host (such as a browser) lacks `require`
// and therefore reports false.
func IsNode() bool {
	defer func() {
		// js.Global lookups never panic for missing properties (they return
		// the undefined value), so recover here only guards against an
		// unexpected host environment, mirroring the try/catch in the source.
		_ = recover()
	}()
	global := js.Global()
	return !global.Get("process").IsUndefined() &&
		!global.Get("require").IsUndefined()
}
