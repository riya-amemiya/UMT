package simple

import (
	"math"
	"sort"
	"time"
)

// QuickSortSimple sorts a slice of integers using the quicksort algorithm.
// Returns a new sorted slice without modifying the original.
//
// Example:
//
//	QuickSortSimple([]int{3, 1, 4, 1, 5}) // [1, 1, 3, 4, 5]
//	QuickSortSimple([]int{})               // []
func QuickSortSimple(arr []int) []int {
	result := make([]int, len(arr))
	copy(result, arr)
	sort.Ints(result)
	return result
}

// BirthdaySimple calculates the age in years from a birth date
// specified by year, month (1-12), and day.
//
// Returns the current age, or 0 if the birth date is in the future.
//
// Example:
//
//	BirthdaySimple(2000, 1, 1) // returns current age for someone born Jan 1, 2000
func BirthdaySimple(year, month, day int) int {
	now := time.Now()
	birthDate := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)

	currentYear := now.Year()
	birthYear := birthDate.Year()

	age := currentYear - birthYear

	// Check if birthday hasn't occurred yet this year
	thisYearBirthday := time.Date(currentYear, birthDate.Month(), birthDate.Day(), 0, 0, 0, 0, time.UTC)
	today := time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC)

	if today.Before(thisYearBirthday) {
		age--
	}

	if age < 0 {
		return 0
	}

	return age
}

// DayOfWeekSimple returns the name of the day of the week for a given date.
// The month parameter uses 1-12 (January=1, December=12).
//
// Returns the full English day name, e.g. "Sunday", "Monday", etc.
//
// Example:
//
//	DayOfWeekSimple(2022, 1, 2) // "Sunday"
//	DayOfWeekSimple(2022, 1, 3) // "Monday"
func DayOfWeekSimple(year, month, day int) string {
	t := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)
	return t.Weekday().String()
}

// NowSimple returns the current date and time formatted as an RFC3339 string.
//
// Example:
//
//	NowSimple() // "2024-01-15T10:30:00Z"
func NowSimple() string {
	return time.Now().UTC().Format(time.RFC3339)
}

// DeviationValueSimple calculates the deviation value (T-score) for a target
// value given a slice of reference scores.
//
// The deviation value is calculated as: ((target - avg) / sd) * 10 + 50
// where avg is the arithmetic mean and sd is the population standard deviation
// of the scores.
//
// Returns 50 when the standard deviation is 0 (all scores are the same).
//
// Example:
//
//	DeviationValueSimple([]float64{40, 50, 60}, 60) // approximately 62.25
//	DeviationValueSimple([]float64{50, 50, 50}, 100) // 50 (sd is 0)
func DeviationValueSimple(scores []float64, target float64) float64 {
	if len(scores) == 0 {
		return 50
	}

	avg := average(scores)
	sd := standardDeviation(scores)

	if sd == 0 {
		return 50
	}

	return ((target - avg) / sd) * 10 + 50
}

// average calculates the arithmetic mean of a slice of float64 values.
func average(nums []float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	sum := 0.0
	for _, v := range nums {
		sum += v
	}
	return sum / float64(len(nums))
}

// standardDeviation calculates the population standard deviation of a slice of float64 values.
func standardDeviation(nums []float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	avg := average(nums)

	squareDiffSum := 0.0
	for _, v := range nums {
		diff := v - avg
		squareDiffSum += diff * diff
	}

	avgSquareDiff := squareDiffSum / float64(len(nums))
	return math.Sqrt(avgSquareDiff)
}
