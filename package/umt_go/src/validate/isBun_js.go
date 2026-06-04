//go:build js && wasm

package validate

import "syscall/js"

// IsBun reports whether the current environment is the Bun runtime.
//
// It mirrors the TypeScript original, which returns true only when the
// runtime-specific `Bun` global is defined. Under js/wasm the program runs
// inside a JavaScript host, so it probes the global object for that binding; a
// non-Bun JavaScript host (such as Node.js or a browser) lacks `Bun` and
// therefore reports false.
func IsBun() bool {
	defer func() {
		// js.Global lookups never panic for missing properties (they return
		// the undefined value), so recover here only guards against an
		// unexpected host environment, mirroring the try/catch in the source.
		_ = recover()
	}()
	return !js.Global().Get("Bun").IsUndefined()
}
