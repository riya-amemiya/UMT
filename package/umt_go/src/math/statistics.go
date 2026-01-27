package math

import (
	stdmath "math"
	"sort"
)

// Average calculates the arithmetic mean of the given numbers.
// Returns 0 for no arguments.
//
// Example:
//
//	Average(1, 2, 3, 4, 5) // 3
//	Average(1.1, 2.2) // 1.65
func Average(nums ...float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	sum := Addition(nums...)
	return Division(sum, float64(len(nums)))
}

// Median calculates the median of the given numbers.
// Returns NaN for no arguments.
//
// Example:
//
//	Median(1, 3, 3, 6, 7, 8, 9) // 6
//	Median(1, 2, 3, 4) // 2.5
func Median(nums ...float64) float64 {
	if len(nums) == 0 {
		return stdmath.NaN()
	}
	sorted := make([]float64, len(nums))
	copy(sorted, nums)
	sort.Float64s(sorted)
	mid := len(sorted) / 2

	if len(sorted)%2 == 0 {
		return (sorted[mid-1] + sorted[mid]) / 2
	}
	return sorted[mid]
}

// Mode finds the most frequently occurring value(s) in the given numbers.
// Returns the modes sorted in ascending order.
//
// Example:
//
//	Mode(1, 2, 2, 3, 3, 3) // [3]
//	Mode(1, 2, 2, 3, 3)    // [2, 3]
//	Mode(1, 2, 3)           // [1, 2, 3]
func Mode(nums ...float64) []float64 {
	if len(nums) == 0 {
		return []float64{}
	}

	freq := make(map[float64]int)
	maxFreq := 0
	for _, v := range nums {
		freq[v]++
		if freq[v] > maxFreq {
			maxFreq = freq[v]
		}
	}

	var modes []float64
	for v, c := range freq {
		if c == maxFreq {
			modes = append(modes, v)
		}
	}
	sort.Float64s(modes)
	return modes
}

// StandardDeviation calculates the population standard deviation of the given values.
//
// Example:
//
//	StandardDeviation(1, 2, 3) // 0.816496580927726
//	StandardDeviation(10, 12, 23, 23, 16, 23, 21, 16) // 4.898979485566356
func StandardDeviation(nums ...float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	avg := Average(nums...)

	squareDiffs := make([]float64, len(nums))
	for i, v := range nums {
		diff := Subtraction(v, avg)
		squareDiffs[i] = Multiplication(diff, diff)
	}

	avgSquareDiff := Average(squareDiffs...)
	return stdmath.Sqrt(avgSquareDiff)
}

// DeviationValue calculates the standard score (deviation value) for a target
// given an array of scores. The formula is: ((target - avg) / sd) * 10 + 50.
//
// Example:
//
//	DeviationValue([]float64{40, 50, 60, 70, 80}, 100) // 100 (if avg=50, sd=10)
func DeviationValue(scores []float64, target float64) float64 {
	avg := Average(scores...)
	sd := StandardDeviation(scores...)
	return ((target - avg) / sd) * 10 + 50
}

// Percentile calculates the nth percentile of values in the data slice.
// Panics if p is not between 0 and 100. Returns NaN for empty data.
//
// Example:
//
//	Percentile([]float64{1, 2, 3, 4, 5}, 50) // 3
//	Percentile([]float64{1, 2, 3, 4, 5}, 25) // 2
//	Percentile([]float64{1, 2, 3, 4, 5}, 75) // 4
func Percentile(data []float64, p float64) float64 {
	if len(data) == 0 {
		return stdmath.NaN()
	}
	if p < 0 || p > 100 {
		panic("Percentile must be between 0 and 100")
	}

	sorted := make([]float64, len(data))
	copy(sorted, data)
	sort.Float64s(sorted)

	index := (p / 100) * float64(len(sorted)-1)
	lower := int(stdmath.Floor(index))
	upper := int(stdmath.Ceil(index))

	if lower == upper {
		return sorted[lower]
	}

	weight := index - float64(lower)
	return sorted[lower] + (sorted[upper]-sorted[lower])*weight
}

// CorrelationCoefficient calculates the Pearson correlation coefficient
// between two slices of numbers. Panics if slices have different lengths.
// Returns NaN for empty slices, single-element slices, or when variance is zero.
//
// Example:
//
//	CorrelationCoefficient([]float64{1,2,3,4,5}, []float64{2,4,6,8,10}) // 1
//	CorrelationCoefficient([]float64{1,2,3,4,5}, []float64{5,4,3,2,1}) // -1
func CorrelationCoefficient(x, y []float64) float64 {
	if len(x) != len(y) {
		panic("Arrays must have the same length")
	}
	if len(x) == 0 || len(x) == 1 {
		return stdmath.NaN()
	}

	meanX := Average(x...)
	meanY := Average(y...)

	var numerator, sumSquaredX, sumSquaredY float64
	for i := range x {
		deltaX := x[i] - meanX
		deltaY := y[i] - meanY
		numerator += deltaX * deltaY
		sumSquaredX += deltaX * deltaX
		sumSquaredY += deltaY * deltaY
	}

	denominator := stdmath.Sqrt(sumSquaredX * sumSquaredY)
	if denominator == 0 {
		return stdmath.NaN()
	}

	return numerator / denominator
}
