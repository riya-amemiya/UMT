package random

// WeightedItem pairs a value with its selection weight.
type WeightedItem[T any] struct {
	Value  T
	Weight float64
}

// WeightedChoice returns a random element using cumulative weights and binary
// search. Items with non-positive weights are skipped from the pool.
//
// Example:
//
//	WeightedChoice([]WeightedItem[string]{
//	    {Value: "a", Weight: 1},
//	    {Value: "b", Weight: 4},
//	})
func WeightedChoice[T any](items []WeightedItem[T]) T {
	cumulative := make([]float64, 0, len(items))
	total := 0.0
	for _, item := range items {
		if item.Weight > 0 {
			total += item.Weight
		}
		cumulative = append(cumulative, total)
	}
	target := randFloat64() * total
	lo := 0
	hi := len(cumulative) - 1
	for lo < hi {
		mid := (lo + hi) >> 1
		if cumulative[mid] <= target {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return items[lo].Value
}
