package random

// rotl32 rotates a 32-bit value left by k bits, matching the bitwise helper
// used by the TypeScript reference (((k % 32) + 32) % 32 rotation).
func rotl32(x uint32, k uint) uint32 {
	rotation := ((k % 32) + 32) % 32
	if rotation == 0 {
		return x
	}
	return (x << rotation) | (x >> (32 - rotation))
}

// xoshiro256Step32 advances a 32-bit xoshiro256**-style state in place and
// returns a float64 in [0, 1). It mirrors the TypeScript xoshiro256 helper,
// which operates on four truncated 32-bit lanes rather than full 64-bit words.
func xoshiro256Step32(state *[4]uint32) float64 {
	sum := state[0] + state[3]
	result := rotl32(sum, 23) + state[0]

	t := state[1] << 17

	state[2] ^= state[0]
	state[3] ^= state[1]
	state[1] ^= state[2]
	state[0] ^= state[3]

	state[2] ^= t
	state[3] = rotl32(state[3], 45)

	return float64(result) / 4294967296.0
}

// hashStringToSeed derives a 32-bit seed from a string using the FNV-1a hash.
func hashStringToSeed(input string) uint32 {
	var hash uint32 = 2166136261
	for _, b := range []byte(input) {
		hash ^= uint32(b)
		hash *= 16777619
	}
	return hash
}

// orDefault returns value unless it is zero, in which case it returns fallback.
func orDefault(value, fallback uint32) uint32 {
	if value == 0 {
		return fallback
	}
	return value
}

// seededState builds the initial four-lane state from a 32-bit seed.
func seededState(initial uint32) [4]uint32 {
	return [4]uint32{
		orDefault(initial, 1),
		orDefault(initial*0x9e3779b9, 2),
		orDefault(initial*0x85ebca6b, 3),
		orDefault(initial*0xc2b2ae35, 4),
	}
}

// SeededRandom returns a deterministic PRNG seeded from a uint32. Each call to
// the returned function produces a float64 in [0, 1) from the same
// xoshiro256**-style stream.
//
// Example:
//
//	rand := SeededRandom(42)
//	rand() // deterministic for the same seed
func SeededRandom(seed uint32) func() float64 {
	state := seededState(seed)
	return func() float64 {
		return xoshiro256Step32(&state)
	}
}

// SeededRandomString returns a deterministic PRNG seeded from a string. The
// string is hashed with FNV-1a to derive the 32-bit seed, then the stream
// behaves identically to SeededRandom.
//
// Example:
//
//	rand := SeededRandomString("hello")
//	rand() // deterministic for the same seed
func SeededRandomString(seed string) func() float64 {
	state := seededState(hashStringToSeed(seed))
	return func() float64 {
		return xoshiro256Step32(&state)
	}
}
