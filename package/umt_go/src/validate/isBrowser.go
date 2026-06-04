//go:build !js || !wasm

package validate

// IsBrowser reports whether the current environment is a browser.
//
// The TypeScript original checks that both the `window` and `document`
// globals are defined. Those globals only exist when JavaScript runs inside a
// browser, so any non-browser target (server, CLI, native binary) is never a
// browser and this implementation returns false. The browser detection that
// inspects the `window` and `document` globals lives in the js/wasm build
// (isBrowser_js.go).
func IsBrowser() bool {
	return false
}
