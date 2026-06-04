//go:build js && wasm

package validate

import "syscall/js"

// IsBrowser reports whether the current environment is a browser.
//
// It mirrors the TypeScript original, which returns true only when both the
// `window` and `document` globals are defined. Under js/wasm the program runs
// inside a JavaScript host, so it probes the global object for those two
// bindings; a non-browser JavaScript host (such as Node.js) lacks `document`
// and therefore reports false.
func IsBrowser() bool {
	defer func() {
		// js.Global lookups never panic for missing properties (they return
		// the undefined value), so recover here only guards against an
		// unexpected host environment, mirroring the try/catch in the source.
		_ = recover()
	}()
	global := js.Global()
	return !global.Get("window").IsUndefined() &&
		!global.Get("document").IsUndefined()
}
