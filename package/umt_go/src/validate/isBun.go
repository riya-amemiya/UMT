//go:build !js || !wasm

package validate

// IsBun reports whether the current environment is the Bun runtime.
//
// The TypeScript original checks that the runtime-specific `Bun` global is
// defined. That global only exists when JavaScript runs inside Bun, so any
// non-JavaScript target (server, CLI, native binary) is never Bun and this
// implementation returns false. The Bun detection that inspects the `Bun`
// global lives in the js/wasm build (isBun_js.go).
func IsBun() bool {
	return false
}
