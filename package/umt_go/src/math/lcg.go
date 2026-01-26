package math

// LinearCongruentialGenerator creates a pseudo-random number generator
// using the Linear Congruential Generator algorithm.
// Returns a function that produces float64 values in [0, 1).
//
// The LCG formula is: state = (multiplier * state + increment) % modulus
// Default parameters: multiplier=1664525, increment=1013904223, modulus=2^32
//
// Example:
//
//	rng := LinearCongruentialGenerator(12345)
//	val := rng() // produces a float64 in [0, 1)
func LinearCongruentialGenerator(seed int64) func() float64 {
	state := seed
	const (
		multiplier int64 = 1664525
		increment  int64 = 1013904223
		modulus    int64 = 4294967296 // 2^32
	)

	return func() float64 {
		state = (multiplier*state + increment) % modulus
		return float64(state) / float64(modulus)
	}
}
