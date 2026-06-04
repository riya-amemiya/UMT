package number

import "math"

// ToPercentage calculates the percentage of value relative to total, rounded to
// the given number of decimal places.
//
// It returns 0 when total is 0 to avoid division by zero. The number of decimal
// places defaults to 2 when omitted. Rounding mirrors JavaScript's Math.round
// (round half toward positive infinity), so the result matches the TypeScript
// implementation for both positive and negative values.
//
// Example:
//
//	ToPercentage(25, 100)    // 25
//	ToPercentage(1, 3)       // 33.33
//	ToPercentage(1, 3, 0)    // 33
//	ToPercentage(0, 0)       // 0
//	ToPercentage(50, 200, 1) // 25
func ToPercentage(value, total float64, decimals ...int) float64 {
	if total == 0 {
		return 0
	}

	places := 2
	if len(decimals) > 0 {
		places = decimals[0]
	}

	factor := math.Pow(10, float64(places))
	return math.Floor((value/total)*100*factor+0.5) / factor
}
