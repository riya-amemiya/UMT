package simple_test

import (
	"math"
	"sort"
	"testing"
	"time"

	"github.com/riya-amemiya/umt-go/src/simple"
)

func TestQuickSortSimple(t *testing.T) {
	tests := []struct {
		name     string
		input    []int
		expected []int
	}{
		{"sorted", []int{1, 2, 3, 4, 5}, []int{1, 2, 3, 4, 5}},
		{"reverse", []int{5, 4, 3, 2, 1}, []int{1, 2, 3, 4, 5}},
		{"unsorted", []int{3, 1, 4, 1, 5}, []int{1, 1, 3, 4, 5}},
		{"empty", []int{}, []int{}},
		{"single", []int{42}, []int{42}},
		{"negatives", []int{-3, -1, -4, -1, -5}, []int{-5, -4, -3, -1, -1}},
		{"duplicates", []int{2, 2, 2, 2}, []int{2, 2, 2, 2}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			original := make([]int, len(tt.input))
			copy(original, tt.input)
			result := simple.QuickSortSimple(tt.input)
			if !sort.IntsAreSorted(result) {
				t.Errorf("QuickSortSimple(%v) = %v, not sorted", original, result)
			}
			if len(result) != len(tt.expected) {
				t.Errorf("QuickSortSimple(%v) length = %d, want %d", original, len(result), len(tt.expected))
			}
		})
	}

	t.Run("does not modify original", func(t *testing.T) {
		input := []int{3, 1, 4, 1, 5}
		original := make([]int, len(input))
		copy(original, input)
		simple.QuickSortSimple(input)
		for i := range input {
			if input[i] != original[i] {
				t.Errorf("Original modified: got %v, want %v", input, original)
				break
			}
		}
	})
}

func TestDayOfWeekSimple(t *testing.T) {
	tests := []struct {
		year, month, day int
		expected         string
	}{
		{2022, 1, 2, "Sunday"},
		{2022, 1, 3, "Monday"},
		{2024, 2, 29, "Thursday"},
		{2000, 1, 1, "Saturday"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result := simple.DayOfWeekSimple(tt.year, tt.month, tt.day)
			if result != tt.expected {
				t.Errorf("DayOfWeekSimple(%d, %d, %d) = %q, want %q",
					tt.year, tt.month, tt.day, result, tt.expected)
			}
		})
	}
}

func TestNowSimple(t *testing.T) {
	result := simple.NowSimple()
	if result == "" {
		t.Error("NowSimple() returned empty string")
	}
	_, err := time.Parse(time.RFC3339, result)
	if err != nil {
		t.Errorf("NowSimple() = %q, not valid RFC3339: %v", result, err)
	}
}

func TestDeviationValueSimple(t *testing.T) {
	t.Run("basic", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := simple.DeviationValueSimple(scores, 50)
		if math.Abs(result-50) > 1e-10 {
			t.Errorf("DeviationValueSimple(scores, 50) = %v, want 50", result)
		}
	})

	t.Run("above average", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := simple.DeviationValueSimple(scores, 70)
		if result <= 50 {
			t.Errorf("DeviationValueSimple(scores, 70) = %v, expected > 50", result)
		}
	})

	t.Run("below average", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := simple.DeviationValueSimple(scores, 30)
		if result >= 50 {
			t.Errorf("DeviationValueSimple(scores, 30) = %v, expected < 50", result)
		}
	})

	t.Run("all same scores", func(t *testing.T) {
		scores := []float64{50, 50, 50}
		result := simple.DeviationValueSimple(scores, 100)
		if result != 50 {
			t.Errorf("DeviationValueSimple(all same, 100) = %v, want 50", result)
		}
	})

	t.Run("empty scores", func(t *testing.T) {
		result := simple.DeviationValueSimple([]float64{}, 100)
		if result != 50 {
			t.Errorf("DeviationValueSimple(empty, 100) = %v, want 50", result)
		}
	})
}

func TestBirthdaySimple(t *testing.T) {
	now := time.Now()
	age := simple.BirthdaySimple(2000, 1, 1)
	expectedAge := now.Year() - 2000
	if now.Month() == 1 && now.Day() < 1 {
		expectedAge--
	}
	if age != expectedAge && age != expectedAge-1 {
		t.Errorf("BirthdaySimple(2000, 1, 1) = %d, expected around %d", age, expectedAge)
	}
}
