package math

// Xoshiro256 creates a pseudo-random number generator using the xoshiro256** algorithm.
// Returns a function that produces float64 values in [0, 1).
// The state is initialized from the seed using the splitmix64 algorithm.
//
// Example:
//
//	rng := Xoshiro256(42)
//	val := rng() // produces a float64 in [0, 1)
func Xoshiro256(seed int64) func() float64 {
	// Initialize state from seed using splitmix64
	smState := uint64(seed)
	splitmix64 := func() uint64 {
		smState += 0x9e3779b97f4a7c15
		z := smState
		z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9
		z = (z ^ (z >> 27)) * 0x94d049bb133111eb
		return z ^ (z >> 31)
	}

	s0 := splitmix64()
	s1 := splitmix64()
	s2 := splitmix64()
	s3 := splitmix64()

	rotl := func(x uint64, k uint) uint64 {
		return (x << k) | (x >> (64 - k))
	}

	return func() float64 {
		result := rotl(s1*5, 7) * 9
		t := s1 << 17

		s2 ^= s0
		s3 ^= s1
		s1 ^= s2
		s0 ^= s3

		s2 ^= t
		s3 = rotl(s3, 45)

		return float64(result>>11) / float64(uint64(1)<<53)
	}
}
