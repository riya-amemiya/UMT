package random

import (
	"crypto/rand"
	"math"
	"math/big"
)

// randFloat64 returns a cryptographically sourced float64 in the half-open
// interval [0, 1), mirroring the behavior of JavaScript's Math.random.
func randFloat64() float64 {
	// 2^53 distinct values give full float64 mantissa precision in [0, 1).
	n, err := rand.Int(rand.Reader, big.NewInt(1<<53))
	if err != nil {
		panic(err)
	}
	return float64(n.Int64()) / float64(uint64(1)<<53)
}

// RandomBoolean returns a random boolean. The probability of true defaults to 0.5.
// Pass an optional probability in the range 0..1 to bias the result.
//
// Example:
//
//	RandomBoolean()    // true or false equally
//	RandomBoolean(0.9) // true ~90% of the time
func RandomBoolean(probability ...float64) bool {
	p := 0.5
	if len(probability) > 0 {
		p = probability[0]
	}
	return randFloat64() < p
}

// RandomChoice returns a uniformly random element from the input slice.
// The slice must be non-empty.
//
// Example:
//
//	RandomChoice([]string{"a", "b", "c"})
func RandomChoice[T any](items []T) T {
	index := int(math.Floor(randFloat64() * float64(len(items))))
	return items[index]
}

// RandomFloat returns a random float in the half-open interval [min, max).
//
// Example:
//
//	RandomFloat(0, 1) // 0.000... up to but not including 1
func RandomFloat(min, max float64) float64 {
	return randFloat64()*(max-min) + min
}

// RandomInt returns a random integer in the closed interval [min, max].
// The lower bound is ceiled and the upper bound is floored before sampling.
//
// Example:
//
//	RandomInt(1, 6) // 1, 2, 3, 4, 5, or 6
func RandomInt(min, max float64) int {
	lo := math.Ceil(min)
	hi := math.Floor(max)
	return int(math.Floor(randFloat64()*(hi-lo+1)) + lo)
}
