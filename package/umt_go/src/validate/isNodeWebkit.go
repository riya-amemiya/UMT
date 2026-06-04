package validate

// IsNodeWebkit reports whether the current environment is Node-Webkit (NW.js).
//
// It mirrors the TypeScript original, which returns true only when the runtime
// is simultaneously a browser and Node.js, a combination unique to the NW.js
// hybrid environment. The detection delegates to IsBrowser and IsNode, both of
// which carry the js/wasm build split, so this composition reports true only
// under js/wasm when both the browser (`window`, `document`) and Node.js
// (`process`, `require`) globals are defined, and false on every other target.
func IsNodeWebkit() bool {
	return IsBrowser() && IsNode()
}
