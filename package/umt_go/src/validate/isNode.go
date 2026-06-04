//go:build !js || !wasm

package validate

// IsNode reports whether the current environment is Node.js.
//
// The TypeScript original checks that both the `process` and `require` globals
// are defined. Those globals only exist when JavaScript runs inside Node.js, so
// any non-JavaScript target (server, CLI, native binary) is never Node.js and
// this implementation returns false. The Node.js detection that inspects the
// `process` and `require` globals lives in the js/wasm build (isNode_js.go).
func IsNode() bool {
	return false
}
