package validate

import "math"

// numberRulesIsDoubleStrict mirrors the TypeScript `isDouble(value, false)`
// strict-mode helper (Validate/isDouble.ts), which is `Number.isFinite(x) &&
// !Number.isInteger(x)`: it reports true only for a finite, non-integer value.
// Non-finite values (NaN, +/-Inf) and integral values both return false. The
// even/odd rules below use this to reject decimals before the parity check,
// exactly as the TS runtime does.
func numberRulesIsDoubleStrict(value float64) bool {
	if math.IsNaN(value) || math.IsInf(value, 0) {
		return false
	}
	return value != math.Trunc(value)
}

// Double creates a number validation rule that passes only for floating-point
// (non-integer) values. It mirrors the TypeScript `double(message?)` factory
// (Validate/number/double.ts), which delegates to the loose `isDouble(value)`
// helper, and the Rust `umt_double`.
//
// For a numeric value the loose `isDouble` predicate reduces to "finite and not
// an integer", so non-finite values (NaN, +/-Inf) and whole numbers fail while
// genuine decimals such as 1.5 or 0.1 pass. The returned rule is meant to be
// supplied in the `option` slice of NumberValidator / ValidateNumber.
func Double(message string) ValidateReturnType[float64] {
	return ValidateReturnType[float64]{
		TypeName: "number",
		Message:  message,
		Validate: func(value float64) bool {
			if math.IsNaN(value) || math.IsInf(value, 0) {
				return false
			}
			return value != math.Trunc(value)
		},
	}
}

// Even creates a number validation rule that passes for even integers. It
// mirrors the TypeScript `even(message?)` factory (Validate/number/even.ts) and
// the Rust `umt_even`.
//
// The TS runtime first rejects finite non-integers via `isDouble(value, false)`
// and then evaluates `value % 2 === 0`. This Go port reproduces that order: a
// finite decimal is rejected up front, and the parity test uses floating-point
// modulo so that NaN and +/-Inf (whose modulo is NaN, never exactly 0) are
// rejected as well, matching the JavaScript semantics. Zero and negative even
// numbers pass.
func Even(message string) ValidateReturnType[float64] {
	return ValidateReturnType[float64]{
		TypeName: "number",
		Message:  message,
		Validate: func(value float64) bool {
			if numberRulesIsDoubleStrict(value) {
				return false
			}
			return math.Mod(value, 2) == 0
		},
	}
}

// Odd creates a number validation rule that passes for odd integers. It mirrors
// the TypeScript `odd(message?)` factory (Validate/number/odd.ts) and the Rust
// `umt_odd`.
//
// As with Even, finite non-integers are rejected first via the strict
// `isDouble` check, then the parity test `value % 2 !== 0` runs with
// floating-point modulo. NaN and +/-Inf yield a NaN modulo, and `NaN != 0` is
// true in Go just as in JavaScript; to keep these non-numeric edge cases out
// they are screened by the leading non-finite guard inside the strict helper,
// so only finite integers reach the parity comparison. Zero is even, hence not
// odd; negative odd numbers pass.
func Odd(message string) ValidateReturnType[float64] {
	return ValidateReturnType[float64]{
		TypeName: "number",
		Message:  message,
		Validate: func(value float64) bool {
			if numberRulesIsDoubleStrict(value) {
				return false
			}
			if math.IsNaN(value) || math.IsInf(value, 0) {
				return false
			}
			return math.Mod(value, 2) != 0
		},
	}
}

// Prime creates a number validation rule that passes for prime numbers. It
// mirrors the TypeScript `prime(message?)` factory (Validate/number/prime.ts),
// which delegates to `isPrimeNumber`, and the Rust `umt_prime`.
//
// Values that are not finite integers, or that are <= 1, fail immediately. The
// primality test then uses the 6k +/- 1 trial-division strategy of the TS
// `isPrimeNumber` helper: 2 and 3 are prime, multiples of 2 or 3 are rejected,
// and remaining candidates are tested against divisors of the form 6k +/- 1 up
// to sqrt(n).
func Prime(message string) ValidateReturnType[float64] {
	return ValidateReturnType[float64]{
		TypeName: "number",
		Message:  message,
		Validate: func(value float64) bool {
			if value <= 1 {
				return false
			}
			if math.IsNaN(value) || math.IsInf(value, 0) || value != math.Trunc(value) {
				return false
			}
			n := int64(value)
			if n <= 3 {
				return true
			}
			if n%2 == 0 || n%3 == 0 {
				return false
			}
			for i := int64(5); i*i <= n; i += 6 {
				if n%i == 0 || n%(i+2) == 0 {
					return false
				}
			}
			return true
		},
	}
}

// MinValue creates a number validation rule that passes when the value is
// greater than or equal to min. It mirrors the TypeScript `minValue(minValue,
// message?)` factory (Validate/number/minValue.ts) and the Rust
// `umt_min_value`. The comparison is the plain `value >= min`, matching the
// TS/Rust runtime.
func MinValue(min float64, message string) ValidateReturnType[float64] {
	return ValidateReturnType[float64]{
		TypeName: "number",
		Message:  message,
		Validate: func(value float64) bool {
			return value >= min
		},
	}
}

// MaxValue creates a number validation rule that passes when the value is less
// than or equal to max. It mirrors the TypeScript `maxValue(maxValue,
// message?)` factory (Validate/number/maxValue.ts) and the Rust
// `umt_max_value`. The comparison is the plain `value <= max`, matching the
// TS/Rust runtime.
func MaxValue(max float64, message string) ValidateReturnType[float64] {
	return ValidateReturnType[float64]{
		TypeName: "number",
		Message:  message,
		Validate: func(value float64) bool {
			return value <= max
		},
	}
}
