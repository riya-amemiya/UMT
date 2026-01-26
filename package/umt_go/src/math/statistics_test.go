package math

import (
	stdmath "math"
	"testing"
)

func TestAverage(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
		closeTo  bool
	}{
		{"two integers", []float64{1, 2}, 1.5, false},
		{"five integers", []float64{1, 2, 3, 4, 5}, 3, false},
		{"two decimals", []float64{1.1, 2.2}, 1.65, true},
		{"five decimals", []float64{1.1, 2.2, 3.3, 4.4, 5.5}, 3.3, true},
		{"single element", []float64{5}, 5, false},
		{"single decimal", []float64{3.14}, 3.14, false},
		{"zeros", []float64{0, 0, 0}, 0, false},
		{"with zeros", []float64{1, 0, 2, 0, 3}, 1.2, false},
		{"negatives", []float64{-1, -2}, -1.5, false},
		{"negative average", []float64{-10, -20, -30}, -20, false},
		{"mixed positive negative", []float64{-1, 1}, 0, false},
		{"mixed cancel out", []float64{-10, 10, -5, 5}, 0, false},
		{"empty", []float64{}, 0, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := Average(tt.args...)
			if tt.closeTo {
				if stdmath.Abs(result-tt.expected) > 1e-9 {
					t.Errorf("Average(%v) = %v, want close to %v", tt.args, result, tt.expected)
				}
			} else {
				if stdmath.Abs(result-tt.expected) > 1e-12 {
					t.Errorf("Average(%v) = %v, want %v", tt.args, result, tt.expected)
				}
			}
		})
	}
}

func TestMedian(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
		isNaN    bool
	}{
		{"even count", []float64{1, 2, 3, 4}, 2.5, false},
		{"odd count", []float64{1, 3, 3, 6, 7, 8, 9}, 6, false},
		{"empty", []float64{}, 0, true},
		{"unsorted", []float64{9, 1, 5, 3, 6}, 5, false},
		{"single element", []float64{5}, 5, false},
		{"single negative", []float64{-10}, -10, false},
		{"two elements", []float64{1, 3}, 2, false},
		{"two elements 2", []float64{10, 20}, 15, false},
		{"all same values", []float64{5, 5, 5, 5}, 5, false},
		{"negatives", []float64{-5, -3, -1, 0, 2}, -1, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := Median(tt.args...)
			if tt.isNaN {
				if !stdmath.IsNaN(result) {
					t.Errorf("Median(%v) = %v, want NaN", tt.args, result)
				}
				return
			}
			if result != tt.expected {
				t.Errorf("Median(%v) = %v, want %v", tt.args, result, tt.expected)
			}
		})
	}
}

func TestMode(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected []float64
	}{
		{"single mode", []float64{1, 2, 2, 3, 3, 3}, []float64{3}},
		{"two modes", []float64{1, 2, 2, 3, 3}, []float64{2, 3}},
		{"all same frequency", []float64{1, 2, 3}, []float64{1, 2, 3}},
		{"empty", []float64{}, []float64{}},
		{"single element", []float64{42}, []float64{42}},
		{"negative numbers", []float64{-1, -2, -2, -3}, []float64{-2}},
		{"decimal numbers", []float64{1.5, 2.5, 2.5, 3.5}, []float64{2.5}},
		{"tied modes sorted", []float64{3, 1, 3, 1, 2, 2}, []float64{1, 2, 3}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := Mode(tt.args...)
			if len(result) != len(tt.expected) {
				t.Errorf("Mode(%v) = %v, want %v", tt.args, result, tt.expected)
				return
			}
			for i := range result {
				if result[i] != tt.expected[i] {
					t.Errorf("Mode(%v) = %v, want %v", tt.args, result, tt.expected)
					return
				}
			}
		})
	}
}

func TestStandardDeviation(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
		epsilon  float64
	}{
		{"sequential 1-5", []float64{1, 2, 3, 4, 5}, stdmath.Sqrt2, 1e-10},
		{"1,2,3", []float64{1, 2, 3}, 0.816496580927726, 1e-12},
		{"non-sequential", []float64{10, 12, 23, 23, 16, 23, 21, 16}, 4.898979485566356, 1e-12},
		{"5 to 25 by 5", []float64{5, 10, 15, 20, 25}, 7.0710678118654755, 1e-10},
		{"all same", []float64{5, 5, 5, 5}, 0, 1e-15},
		{"single value", []float64{42}, 0, 1e-15},
		{"empty", []float64{}, 0, 1e-15},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := StandardDeviation(tt.args...)
			if stdmath.Abs(result-tt.expected) > tt.epsilon {
				t.Errorf("StandardDeviation(%v) = %v, want %v", tt.args, result, tt.expected)
			}
		})
	}
}

func TestDeviationValue(t *testing.T) {
	t.Run("basic", func(t *testing.T) {
		// Using known scores where avg=50, sd=10
		scores := []float64{30, 40, 50, 60, 70}
		avg := Average(scores...)
		sd := StandardDeviation(scores...)

		// Verify our scores have the right properties
		if stdmath.Abs(avg-50) > 1e-10 {
			t.Errorf("Expected avg=50, got %v", avg)
		}

		// For target=100: ((100-50)/sd)*10+50
		result := DeviationValue(scores, 100)
		expected := ((100 - avg) / sd) * 10 + 50
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("DeviationValue(scores, 100) = %v, want %v", result, expected)
		}
	})

	t.Run("target equals average", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := DeviationValue(scores, 50)
		if stdmath.Abs(result-50) > 1e-10 {
			t.Errorf("DeviationValue(scores, 50) = %v, want 50", result)
		}
	})

	t.Run("target below average", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := DeviationValue(scores, 30)
		if result >= 50 {
			t.Errorf("DeviationValue(scores, 30) = %v, expected < 50", result)
		}
	})
}

func TestPercentile(t *testing.T) {
	t.Run("basic percentiles", func(t *testing.T) {
		data := []float64{1, 2, 3, 4, 5}
		if got := Percentile(data, 50); got != 3 {
			t.Errorf("Percentile(data, 50) = %v, want 3", got)
		}
		if got := Percentile(data, 25); got != 2 {
			t.Errorf("Percentile(data, 25) = %v, want 2", got)
		}
		if got := Percentile(data, 75); got != 4 {
			t.Errorf("Percentile(data, 75) = %v, want 4", got)
		}
	})

	t.Run("0th and 100th percentile", func(t *testing.T) {
		data := []float64{1, 2, 3, 4, 5}
		if got := Percentile(data, 0); got != 1 {
			t.Errorf("Percentile(data, 0) = %v, want 1", got)
		}
		if got := Percentile(data, 100); got != 5 {
			t.Errorf("Percentile(data, 100) = %v, want 5", got)
		}
	})

	t.Run("single element", func(t *testing.T) {
		data := []float64{42}
		if got := Percentile(data, 50); got != 42 {
			t.Errorf("Percentile([42], 50) = %v, want 42", got)
		}
	})

	t.Run("empty array returns NaN", func(t *testing.T) {
		if got := Percentile([]float64{}, 50); !stdmath.IsNaN(got) {
			t.Errorf("Percentile([], 50) = %v, want NaN", got)
		}
	})

	t.Run("invalid percentile panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for percentile -1")
			}
		}()
		Percentile([]float64{1, 2, 3}, -1)
	})

	t.Run("invalid percentile > 100 panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for percentile 101")
			}
		}()
		Percentile([]float64{1, 2, 3}, 101)
	})

	t.Run("unsorted array", func(t *testing.T) {
		if got := Percentile([]float64{5, 1, 3, 2, 4}, 50); got != 3 {
			t.Errorf("Percentile([5,1,3,2,4], 50) = %v, want 3", got)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		if got := Percentile([]float64{-5, -3, -1, 1, 3}, 50); got != -1 {
			t.Errorf("Percentile([-5,-3,-1,1,3], 50) = %v, want -1", got)
		}
	})

	t.Run("duplicate values", func(t *testing.T) {
		if got := Percentile([]float64{1, 1, 2, 2, 3, 3}, 50); got != 2 {
			t.Errorf("Percentile([1,1,2,2,3,3], 50) = %v, want 2", got)
		}
	})
}

func TestCorrelationCoefficient(t *testing.T) {
	t.Run("perfect positive correlation", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{2, 4, 6, 8, 10})
		if stdmath.Abs(result-1) > 1e-10 {
			t.Errorf("Got %v, want 1", result)
		}
	})

	t.Run("perfect negative correlation", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{5, 4, 3, 2, 1})
		if stdmath.Abs(result-(-1)) > 1e-10 {
			t.Errorf("Got %v, want -1", result)
		}
	})

	t.Run("no variance returns NaN", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{1, 1, 1, 1, 1})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("different lengths panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for different length arrays")
			}
		}()
		CorrelationCoefficient([]float64{1, 2, 3}, []float64{1, 2})
	})

	t.Run("empty arrays", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{}, []float64{})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1}, []float64{2})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("partial correlation", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{2, 3, 5, 4, 6})
		if result <= 0 || result >= 1 {
			t.Errorf("Got %v, expected between 0 and 1", result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{-1, -2, -3}, []float64{-2, -4, -6})
		if stdmath.Abs(result-1) > 1e-10 {
			t.Errorf("Got %v, want 1", result)
		}
	})

	t.Run("decimal numbers", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1.5, 2.5, 3.5}, []float64{3.0, 5.0, 7.0})
		if stdmath.Abs(result-1) > 1e-10 {
			t.Errorf("Got %v, want 1", result)
		}
	})

	t.Run("x has no variance", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1, 1, 1}, []float64{1, 2, 3})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("y has no variance", func(t *testing.T) {
		result := CorrelationCoefficient([]float64{1, 2, 3}, []float64{2, 2, 2})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})
}
