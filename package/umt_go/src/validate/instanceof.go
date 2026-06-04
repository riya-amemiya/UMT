package validate

// InstanceOf reports whether a value is an instance of the type T, mirroring the
// TypeScript `instanceOf` validator (Validate/instanceof/core.ts), which checks
// a value against a constructor at runtime, and the Rust `umt_instance_of`
// function, which performs a runtime type check on a `&dyn Any` trait object.
//
// JavaScript decides this with the `instanceof` operator and accepts subclasses.
// Go has no class inheritance, so the check is concrete-type identity via a type
// assertion: it returns true when the dynamic type of value is exactly T (or a
// type that implements T when T is an interface), false otherwise.
func InstanceOf[T any](value any) bool {
	_, ok := value.(T)
	return ok
}
