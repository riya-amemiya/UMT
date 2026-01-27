package math_test

import (
	stdmath "math"
	"strings"
	"testing"

	umt_math "github.com/riya-amemiya/umt-go/src/math"
)

// ============================================================
// Arithmetic tests (from arithmetic_test.go)
// ============================================================

func TestAddition(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
		closeTo  bool
	}{
		{"two positive integers", []float64{2, 3}, 5, false},
		{"two negative integers", []float64{-2, -3}, -5, false},
		{"positive and negative", []float64{2, -3}, -1, false},
		{"0.1 + 0.2", []float64{0.1, 0.2}, 0.3, true},
		{"-0.1 + -0.2", []float64{-0.1, -0.2}, -0.3, true},
		{"0.1 + -0.2", []float64{0.1, -0.2}, -0.1, true},
		{"2 + 0.3", []float64{2, 0.3}, 2.3, true},
		{"-2 + -0.3", []float64{-2, -0.3}, -2.3, true},
		{"2 + -0.3", []float64{2, -0.3}, 1.7, true},
		{"-2 + 0.3", []float64{-2, 0.3}, -1.7, true},
		{"three integers", []float64{1, 2, 3}, 6, false},
		{"three negative integers", []float64{-1, -2, -3}, -6, false},
		{"three decimals", []float64{0.1, 0.2, 0.3}, 0.6, true},
		{"sum to zero", []float64{2, -3, 1}, 0, false},
		{"mixed sum to zero", []float64{-2, 0.5, 1.5}, 0, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Addition(tt.args...)
			if tt.closeTo {
				if stdmath.Abs(result-tt.expected) > 1e-9 {
					t.Errorf("Addition(%v) = %v, want close to %v", tt.args, result, tt.expected)
				}
			} else {
				if result != tt.expected {
					t.Errorf("Addition(%v) = %v, want %v", tt.args, result, tt.expected)
				}
			}
		})
	}
}

func TestSubtraction(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
	}{
		{"1 - 1", []float64{1, 1}, 0},
		{"1.1 - 1", []float64{1.1, 1}, 0.1},
		{"1 - 1.1", []float64{1, 1.1}, -0.1},
		{"1 - 1 - 1", []float64{1, 1, 1}, -1},
		{"5 - 2 - 1", []float64{5, 2, 1}, 2},
		{"10 - 1 - 1 - 1", []float64{10, 1, 1, 1}, 7},
		{"0.3 - 0.1", []float64{0.3, 0.1}, 0.2},
		{"0.7 - 0.1", []float64{0.7, 0.1}, 0.6},
		{"1.0 - 0.9", []float64{1.0, 0.9}, 0.1},
		{"precise decimals", []float64{0.12345, 0.00001}, 0.12344},
		{"precise three args", []float64{1.23456, 0.00001, 0.00002}, 1.23453},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Subtraction(tt.args...)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("Subtraction(%v) = %v, want %v", tt.args, result, tt.expected)
			}
		})
	}
}

func TestMultiplication(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
		closeTo  bool
	}{
		{"two positive integers", []float64{2, 3}, 6, false},
		{"large integers", []float64{10, 20}, 200, false},
		{"three integers", []float64{2, 3, 4}, 24, false},
		{"five integers", []float64{1, 2, 3, 4, 5}, 120, false},
		{"decimal multiplication", []float64{0.1, 0.2}, 0.02, true},
		{"three decimals", []float64{0.1, 0.2, 0.3}, 0.006, true},
		{"larger decimals", []float64{1.23, 4.56}, 5.6088, true},
		{"mixed int and decimal", []float64{0.5, 2}, 1, false},
		{"0.1 * 10", []float64{0.1, 10}, 1, false},
		{"1.5 * 2 * 3", []float64{1.5, 2, 3}, 9, false},
		{"negative * positive", []float64{-2, 3}, -6, false},
		{"positive * negative", []float64{2, -3}, -6, false},
		{"negative * negative", []float64{-2, -3}, 6, false},
		{"zero * number", []float64{0, 5}, 0, false},
		{"number * zero", []float64{5, 0}, 0, false},
		{"zero * zero", []float64{0, 0}, 0, false},
		{"identity * number", []float64{1, 5}, 5, false},
		{"number * identity", []float64{5, 1}, 5, false},
		{"very small", []float64{0.0001, 0.0001}, 0.00000001, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Multiplication(tt.args...)
			if tt.closeTo {
				if stdmath.Abs(result-tt.expected) > 1e-12 {
					t.Errorf("Multiplication(%v) = %v, want close to %v", tt.args, result, tt.expected)
				}
			} else {
				if result != tt.expected {
					t.Errorf("Multiplication(%v) = %v, want %v", tt.args, result, tt.expected)
				}
			}
		})
	}
}

func TestDivision(t *testing.T) {
	tests := []struct {
		name     string
		args     []float64
		expected float64
		closeTo  bool
		isNaN    bool
	}{
		{"10 / 2", []float64{10, 2}, 5, false, false},
		{"30 / 5", []float64{30, 5}, 6, false, false},
		{"1000 / 10", []float64{1000, 10}, 100, false, false},
		{"10 / 100", []float64{10, 100}, 0.1, false, false},
		{"1 / 1000", []float64{1, 1000}, 0.001, false, false},
		{"0.1 / 0.2", []float64{0.1, 0.2}, 0.5, false, false},
		{"10.5 / 2.1", []float64{10.5, 2.1}, 5, false, false},
		{"0.001 / 0.1", []float64{0.001, 0.1}, 0.01, false, false},
		{"0.1 / 0.001", []float64{0.1, 0.001}, 100, false, false},
		{"1.1 / 2.2", []float64{1.1, 2.2}, 0.5, false, false},
		{"-10 / 2", []float64{-10, 2}, -5, false, false},
		{"10 / -2", []float64{10, -2}, -5, false, false},
		{"-10 / -2", []float64{-10, -2}, 5, false, false},
		{"div by zero", []float64{1, 0}, 0, false, true},
		{"zero / 1", []float64{0, 1}, 0, false, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Division(tt.args...)
			if tt.isNaN {
				if !stdmath.IsNaN(result) {
					t.Errorf("Division(%v) = %v, want NaN", tt.args, result)
				}
				return
			}
			if tt.closeTo {
				if stdmath.Abs(result-tt.expected) > 1e-9 {
					t.Errorf("Division(%v) = %v, want close to %v", tt.args, result, tt.expected)
				}
			} else {
				if stdmath.Abs(result-tt.expected) > 1e-12 {
					t.Errorf("Division(%v) = %v, want %v", tt.args, result, tt.expected)
				}
			}
		})
	}
}

func TestQuotient(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"exact division", 4, 2, 2},
		{"exact division 2", 10, 5, 2},
		{"with remainder", 5, 2, 2},
		{"with remainder 2", 7, 3, 2},
		{"with remainder 3", 10, 3, 3},
		{"div by 1", 5, 1, 5},
		{"smaller dividend", 2, 5, 0},
		{"negative dividend", -5, 2, -2},
		{"negative divisor", 5, -2, -2},
		{"both negative", -5, -2, 2},
		{"large numbers", 1000, 3, 333},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Quotient(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("Quotient(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestMax(t *testing.T) {
	t.Run("basic max", func(t *testing.T) {
		if got := umt_math.Max(1, 2, 3); got != 3 {
			t.Errorf("Max(1,2,3) = %v, want 3", got)
		}
		if got := umt_math.Max(3, 2, 1); got != 3 {
			t.Errorf("Max(3,2,1) = %v, want 3", got)
		}
		if got := umt_math.Max(-1, -2, -3); got != -1 {
			t.Errorf("Max(-1,-2,-3) = %v, want -1", got)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		if got := umt_math.Max(1, 1, 1); got != 1 {
			t.Errorf("Max(1,1,1) = %v, want 1", got)
		}
		if got := umt_math.Max(1, 2, 2, 3, 3); got != 3 {
			t.Errorf("Max(1,2,2,3,3) = %v, want 3", got)
		}
	})

	t.Run("single value", func(t *testing.T) {
		if got := umt_math.Max(5); got != 5 {
			t.Errorf("Max(5) = %v, want 5", got)
		}
		if got := umt_math.Max(-5); got != -5 {
			t.Errorf("Max(-5) = %v, want -5", got)
		}
	})

	t.Run("float64 max", func(t *testing.T) {
		if got := umt_math.Max(1.5, 2.5, 1.1); got != 2.5 {
			t.Errorf("Max(1.5,2.5,1.1) = %v, want 2.5", got)
		}
		if got := umt_math.Max(-1.5, -2.5, -1.1); got != -1.1 {
			t.Errorf("Max(-1.5,-2.5,-1.1) = %v, want -1.1", got)
		}
	})
}

func TestMin(t *testing.T) {
	t.Run("basic min", func(t *testing.T) {
		if got := umt_math.Min(1, 2, 3); got != 1 {
			t.Errorf("Min(1,2,3) = %v, want 1", got)
		}
		if got := umt_math.Min(3, 2, 1); got != 1 {
			t.Errorf("Min(3,2,1) = %v, want 1", got)
		}
		if got := umt_math.Min(-1, -2, -3); got != -3 {
			t.Errorf("Min(-1,-2,-3) = %v, want -3", got)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		if got := umt_math.Min(1, 1, 1); got != 1 {
			t.Errorf("Min(1,1,1) = %v, want 1", got)
		}
		if got := umt_math.Min(1, 2, 2, 3, 3); got != 1 {
			t.Errorf("Min(1,2,2,3,3) = %v, want 1", got)
		}
	})

	t.Run("single value", func(t *testing.T) {
		if got := umt_math.Min(5); got != 5 {
			t.Errorf("Min(5) = %v, want 5", got)
		}
		if got := umt_math.Min(-5); got != -5 {
			t.Errorf("Min(-5) = %v, want -5", got)
		}
	})

	t.Run("float64 min", func(t *testing.T) {
		if got := umt_math.Min(1.5, 2.5, 1.1); got != 1.1 {
			t.Errorf("Min(1.5,2.5,1.1) = %v, want 1.1", got)
		}
		if got := umt_math.Min(-1.5, -2.5, -1.1); got != -2.5 {
			t.Errorf("Min(-1.5,-2.5,-1.1) = %v, want -2.5", got)
		}
	})
}

func TestValueSwap(t *testing.T) {
	t.Run("swap integers", func(t *testing.T) {
		a, b := 1, 2
		umt_math.ValueSwap(&a, &b)
		if a != 2 || b != 1 {
			t.Errorf("ValueSwap(&1, &2): a=%d, b=%d, want a=2, b=1", a, b)
		}
	})

	t.Run("swap floats", func(t *testing.T) {
		a, b := 1.5, 2.5
		umt_math.ValueSwap(&a, &b)
		if a != 2.5 || b != 1.5 {
			t.Errorf("ValueSwap(&1.5, &2.5): a=%f, b=%f, want a=2.5, b=1.5", a, b)
		}
	})

	t.Run("swap strings", func(t *testing.T) {
		a, b := "hello", "world"
		umt_math.ValueSwap(&a, &b)
		if a != "world" || b != "hello" {
			t.Errorf("ValueSwap: a=%s, b=%s, want a=world, b=hello", a, b)
		}
	})

	t.Run("swap equal values", func(t *testing.T) {
		a, b := 5, 5
		umt_math.ValueSwap(&a, &b)
		if a != 5 || b != 5 {
			t.Errorf("ValueSwap(&5, &5): a=%d, b=%d, want a=5, b=5", a, b)
		}
	})

	t.Run("swap negative values", func(t *testing.T) {
		a, b := -5, 5
		umt_math.ValueSwap(&a, &b)
		if a != 5 || b != -5 {
			t.Errorf("ValueSwap(&-5, &5): a=%d, b=%d, want a=5, b=-5", a, b)
		}
	})
}

// ============================================================
// Statistics tests (from statistics_test.go)
// ============================================================

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
			result := umt_math.Average(tt.args...)
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
			result := umt_math.Median(tt.args...)
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
			result := umt_math.Mode(tt.args...)
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
			result := umt_math.StandardDeviation(tt.args...)
			if stdmath.Abs(result-tt.expected) > tt.epsilon {
				t.Errorf("StandardDeviation(%v) = %v, want %v", tt.args, result, tt.expected)
			}
		})
	}
}

func TestDeviationValue(t *testing.T) {
	t.Run("basic", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		avg := umt_math.Average(scores...)
		sd := umt_math.StandardDeviation(scores...)

		if stdmath.Abs(avg-50) > 1e-10 {
			t.Errorf("Expected avg=50, got %v", avg)
		}

		result := umt_math.DeviationValue(scores, 100)
		expected := ((100 - avg) / sd) * 10 + 50
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("DeviationValue(scores, 100) = %v, want %v", result, expected)
		}
	})

	t.Run("target equals average", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := umt_math.DeviationValue(scores, 50)
		if stdmath.Abs(result-50) > 1e-10 {
			t.Errorf("DeviationValue(scores, 50) = %v, want 50", result)
		}
	})

	t.Run("target below average", func(t *testing.T) {
		scores := []float64{30, 40, 50, 60, 70}
		result := umt_math.DeviationValue(scores, 30)
		if result >= 50 {
			t.Errorf("DeviationValue(scores, 30) = %v, expected < 50", result)
		}
	})
}

func TestPercentile(t *testing.T) {
	t.Run("basic percentiles", func(t *testing.T) {
		data := []float64{1, 2, 3, 4, 5}
		if got := umt_math.Percentile(data, 50); got != 3 {
			t.Errorf("Percentile(data, 50) = %v, want 3", got)
		}
		if got := umt_math.Percentile(data, 25); got != 2 {
			t.Errorf("Percentile(data, 25) = %v, want 2", got)
		}
		if got := umt_math.Percentile(data, 75); got != 4 {
			t.Errorf("Percentile(data, 75) = %v, want 4", got)
		}
	})

	t.Run("0th and 100th percentile", func(t *testing.T) {
		data := []float64{1, 2, 3, 4, 5}
		if got := umt_math.Percentile(data, 0); got != 1 {
			t.Errorf("Percentile(data, 0) = %v, want 1", got)
		}
		if got := umt_math.Percentile(data, 100); got != 5 {
			t.Errorf("Percentile(data, 100) = %v, want 5", got)
		}
	})

	t.Run("single element", func(t *testing.T) {
		data := []float64{42}
		if got := umt_math.Percentile(data, 50); got != 42 {
			t.Errorf("Percentile([42], 50) = %v, want 42", got)
		}
	})

	t.Run("empty array returns NaN", func(t *testing.T) {
		if got := umt_math.Percentile([]float64{}, 50); !stdmath.IsNaN(got) {
			t.Errorf("Percentile([], 50) = %v, want NaN", got)
		}
	})

	t.Run("invalid percentile panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for percentile -1")
			}
		}()
		umt_math.Percentile([]float64{1, 2, 3}, -1)
	})

	t.Run("invalid percentile > 100 panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for percentile 101")
			}
		}()
		umt_math.Percentile([]float64{1, 2, 3}, 101)
	})

	t.Run("unsorted array", func(t *testing.T) {
		if got := umt_math.Percentile([]float64{5, 1, 3, 2, 4}, 50); got != 3 {
			t.Errorf("Percentile([5,1,3,2,4], 50) = %v, want 3", got)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		if got := umt_math.Percentile([]float64{-5, -3, -1, 1, 3}, 50); got != -1 {
			t.Errorf("Percentile([-5,-3,-1,1,3], 50) = %v, want -1", got)
		}
	})

	t.Run("duplicate values", func(t *testing.T) {
		if got := umt_math.Percentile([]float64{1, 1, 2, 2, 3, 3}, 50); got != 2 {
			t.Errorf("Percentile([1,1,2,2,3,3], 50) = %v, want 2", got)
		}
	})
}

func TestCorrelationCoefficient(t *testing.T) {
	t.Run("perfect positive correlation", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{2, 4, 6, 8, 10})
		if stdmath.Abs(result-1) > 1e-10 {
			t.Errorf("Got %v, want 1", result)
		}
	})

	t.Run("perfect negative correlation", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{5, 4, 3, 2, 1})
		if stdmath.Abs(result-(-1)) > 1e-10 {
			t.Errorf("Got %v, want -1", result)
		}
	})

	t.Run("no variance returns NaN", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{1, 1, 1, 1, 1})
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
		umt_math.CorrelationCoefficient([]float64{1, 2, 3}, []float64{1, 2})
	})

	t.Run("empty arrays", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{}, []float64{})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1}, []float64{2})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("partial correlation", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1, 2, 3, 4, 5}, []float64{2, 3, 5, 4, 6})
		if result <= 0 || result >= 1 {
			t.Errorf("Got %v, expected between 0 and 1", result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{-1, -2, -3}, []float64{-2, -4, -6})
		if stdmath.Abs(result-1) > 1e-10 {
			t.Errorf("Got %v, want 1", result)
		}
	})

	t.Run("decimal numbers", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1.5, 2.5, 3.5}, []float64{3.0, 5.0, 7.0})
		if stdmath.Abs(result-1) > 1e-10 {
			t.Errorf("Got %v, want 1", result)
		}
	})

	t.Run("x has no variance", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1, 1, 1}, []float64{1, 2, 3})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})

	t.Run("y has no variance", func(t *testing.T) {
		result := umt_math.CorrelationCoefficient([]float64{1, 2, 3}, []float64{2, 2, 2})
		if !stdmath.IsNaN(result) {
			t.Errorf("Got %v, want NaN", result)
		}
	})
}

// ============================================================
// Combinatorics tests (from combinatorics_test.go)
// ============================================================

func TestFactorial(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected int
	}{
		{"factorial of 0", 0, 1},
		{"factorial of 1", 1, 1},
		{"factorial of 2", 2, 2},
		{"factorial of 3", 3, 6},
		{"factorial of 4", 4, 24},
		{"factorial of 5", 5, 120},
		{"factorial of 6", 6, 720},
		{"factorial of 7", 7, 5040},
		{"factorial of 8", 8, 40320},
		{"factorial of 9", 9, 362880},
		{"factorial of 10", 10, 3628800},
		{"factorial of 11", 11, 39916800},
		{"factorial of 12", 12, 479001600},
		{"factorial of 13", 13, 6227020800},
		{"factorial of 14", 14, 87178291200},
		{"negative returns 1", -1, 1},
		{"negative -5 returns 1", -5, 1},
		{"negative -10 returns 1", -10, 1},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Factorial(tt.n)
			if result != tt.expected {
				t.Errorf("Factorial(%d) = %d, want %d", tt.n, result, tt.expected)
			}
		})
	}
}

func TestNCr(t *testing.T) {
	t.Run("basic combinations", func(t *testing.T) {
		if got := umt_math.NCr(5, 2); got != 10 {
			t.Errorf("NCr(5,2) = %d, want 10", got)
		}
		if got := umt_math.NCr(10, 4); got != 210 {
			t.Errorf("NCr(10,4) = %d, want 210", got)
		}
	})

	t.Run("choosing all items", func(t *testing.T) {
		if got := umt_math.NCr(3, 3); got != 1 {
			t.Errorf("NCr(3,3) = %d, want 1", got)
		}
		if got := umt_math.NCr(5, 5); got != 1 {
			t.Errorf("NCr(5,5) = %d, want 1", got)
		}
	})

	t.Run("choosing one item", func(t *testing.T) {
		if got := umt_math.NCr(5, 1); got != 5 {
			t.Errorf("NCr(5,1) = %d, want 5", got)
		}
		if got := umt_math.NCr(10, 1); got != 10 {
			t.Errorf("NCr(10,1) = %d, want 10", got)
		}
	})

	t.Run("larger combinations", func(t *testing.T) {
		if got := umt_math.NCr(20, 10); got != 184756 {
			t.Errorf("NCr(20,10) = %d, want 184756", got)
		}
		if got := umt_math.NCr(15, 7); got != 6435 {
			t.Errorf("NCr(15,7) = %d, want 6435", got)
		}
	})

	t.Run("choosing nothing", func(t *testing.T) {
		if got := umt_math.NCr(5, 0); got != 1 {
			t.Errorf("NCr(5,0) = %d, want 1", got)
		}
		if got := umt_math.NCr(10, 0); got != 1 {
			t.Errorf("NCr(10,0) = %d, want 1", got)
		}
	})

	t.Run("invalid inputs return -1", func(t *testing.T) {
		if got := umt_math.NCr(0, 5); got != -1 {
			t.Errorf("NCr(0,5) = %d, want -1", got)
		}
		if got := umt_math.NCr(2, 5); got != -1 {
			t.Errorf("NCr(2,5) = %d, want -1", got)
		}
		if got := umt_math.NCr(3, 4); got != -1 {
			t.Errorf("NCr(3,4) = %d, want -1", got)
		}
		if got := umt_math.NCr(-1, 2); got != -1 {
			t.Errorf("NCr(-1,2) = %d, want -1", got)
		}
		if got := umt_math.NCr(5, -1); got != -1 {
			t.Errorf("NCr(5,-1) = %d, want -1", got)
		}
	})

	t.Run("n=0, r=0", func(t *testing.T) {
		if got := umt_math.NCr(0, 0); got != 1 {
			t.Errorf("NCr(0,0) = %d, want 1", got)
		}
	})
}

func TestNPr(t *testing.T) {
	t.Run("basic permutations", func(t *testing.T) {
		if got := umt_math.NPr(5, 2); got != 20 {
			t.Errorf("NPr(5,2) = %d, want 20", got)
		}
		if got := umt_math.NPr(10, 4); got != 5040 {
			t.Errorf("NPr(10,4) = %d, want 5040", got)
		}
	})

	t.Run("choosing one item", func(t *testing.T) {
		if got := umt_math.NPr(3, 1); got != 3 {
			t.Errorf("NPr(3,1) = %d, want 3", got)
		}
		if got := umt_math.NPr(5, 1); got != 5 {
			t.Errorf("NPr(5,1) = %d, want 5", got)
		}
	})

	t.Run("choosing all items", func(t *testing.T) {
		if got := umt_math.NPr(3, 3); got != 6 {
			t.Errorf("NPr(3,3) = %d, want 6", got)
		}
		if got := umt_math.NPr(4, 4); got != 24 {
			t.Errorf("NPr(4,4) = %d, want 24", got)
		}
	})

	t.Run("larger permutations", func(t *testing.T) {
		if got := umt_math.NPr(8, 3); got != 336 {
			t.Errorf("NPr(8,3) = %d, want 336", got)
		}
		if got := umt_math.NPr(6, 4); got != 360 {
			t.Errorf("NPr(6,4) = %d, want 360", got)
		}
	})

	t.Run("arranging nothing", func(t *testing.T) {
		if got := umt_math.NPr(5, 0); got != 1 {
			t.Errorf("NPr(5,0) = %d, want 1", got)
		}
		if got := umt_math.NPr(10, 0); got != 1 {
			t.Errorf("NPr(10,0) = %d, want 1", got)
		}
		if got := umt_math.NPr(0, 0); got != 1 {
			t.Errorf("NPr(0,0) = %d, want 1", got)
		}
	})

	t.Run("invalid inputs", func(t *testing.T) {
		if got := umt_math.NPr(0, 5); got != -1 {
			t.Errorf("NPr(0,5) = %d, want -1", got)
		}
		if got := umt_math.NPr(2, 5); got != -1 {
			t.Errorf("NPr(2,5) = %d, want -1", got)
		}
		if got := umt_math.NPr(-1, 2); got != -1 {
			t.Errorf("NPr(-1,2) = %d, want -1", got)
		}
		if got := umt_math.NPr(5, -1); got != -1 {
			t.Errorf("NPr(5,-1) = %d, want -1", got)
		}
	})

	t.Run("n equals r", func(t *testing.T) {
		if got := umt_math.NPr(2, 2); got != 2 {
			t.Errorf("NPr(2,2) = %d, want 2", got)
		}
		if got := umt_math.NPr(5, 5); got != 120 {
			t.Errorf("NPr(5,5) = %d, want 120", got)
		}
	})
}

func TestNHr(t *testing.T) {
	t.Run("basic combinations with repetition", func(t *testing.T) {
		if got := umt_math.NHr(5, 2); got != 15 {
			t.Errorf("NHr(5,2) = %d, want 15", got)
		}
		if got := umt_math.NHr(3, 3); got != 10 {
			t.Errorf("NHr(3,3) = %d, want 10", got)
		}
	})

	t.Run("choosing one item", func(t *testing.T) {
		if got := umt_math.NHr(3, 1); got != 3 {
			t.Errorf("NHr(3,1) = %d, want 3", got)
		}
		if got := umt_math.NHr(5, 1); got != 5 {
			t.Errorf("NHr(5,1) = %d, want 5", got)
		}
	})

	t.Run("choosing same as total items", func(t *testing.T) {
		if got := umt_math.NHr(2, 2); got != 3 {
			t.Errorf("NHr(2,2) = %d, want 3", got)
		}
		if got := umt_math.NHr(4, 4); got != 35 {
			t.Errorf("NHr(4,4) = %d, want 35", got)
		}
	})

	t.Run("larger combinations", func(t *testing.T) {
		if got := umt_math.NHr(10, 3); got != 220 {
			t.Errorf("NHr(10,3) = %d, want 220", got)
		}
		if got := umt_math.NHr(6, 4); got != 126 {
			t.Errorf("NHr(6,4) = %d, want 126", got)
		}
	})

	t.Run("invalid when n or r is 0", func(t *testing.T) {
		if got := umt_math.NHr(0, 5); got != -1 {
			t.Errorf("NHr(0,5) = %d, want -1", got)
		}
		if got := umt_math.NHr(5, 0); got != -1 {
			t.Errorf("NHr(5,0) = %d, want -1", got)
		}
		if got := umt_math.NHr(0, 0); got != -1 {
			t.Errorf("NHr(0,0) = %d, want -1", got)
		}
	})

	t.Run("invalid when negative", func(t *testing.T) {
		if got := umt_math.NHr(-1, 5); got != -1 {
			t.Errorf("NHr(-1,5) = %d, want -1", got)
		}
		if got := umt_math.NHr(5, -1); got != -1 {
			t.Errorf("NHr(5,-1) = %d, want -1", got)
		}
	})
}

func TestRepeatedTrial(t *testing.T) {
	t.Run("basic binomial probability", func(t *testing.T) {
		result := umt_math.RepeatedTrial(5, 0.5, 2)
		expected := 0.3125
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(5, 0.5, 2) = %v, want %v", result, expected)
		}
	})

	t.Run("probability 1/3", func(t *testing.T) {
		result := umt_math.RepeatedTrial(4, 1.0/3.0, 2)
		expected := 6.0 * (1.0 / 9.0) * (4.0 / 9.0)
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(4, 1/3, 2) = %v, want %v", result, expected)
		}
	})

	t.Run("all successes", func(t *testing.T) {
		result := umt_math.RepeatedTrial(3, 2.0/3.0, 3)
		expected := 8.0 / 27.0
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(3, 2/3, 3) = %v, want %v", result, expected)
		}
	})

	t.Run("zero successes", func(t *testing.T) {
		result := umt_math.RepeatedTrial(3, 0.5, 0)
		expected := 0.125
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(3, 0.5, 0) = %v, want %v", result, expected)
		}
	})
}

// ============================================================
// Number theory tests (from number_theory_test.go)
// ============================================================

func TestIsPrimeNumber(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected bool
	}{
		{"2 is prime", 2, true},
		{"3 is prime", 3, true},
		{"5 is prime", 5, true},
		{"7 is prime", 7, true},
		{"11 is prime", 11, true},
		{"13 is prime", 13, true},
		{"17 is prime", 17, true},
		{"31 is prime", 31, true},
		{"997 is prime", 997, true},
		{"4 is not prime", 4, false},
		{"6 is not prime", 6, false},
		{"8 is not prime", 8, false},
		{"9 is not prime", 9, false},
		{"10 is not prime", 10, false},
		{"15 is not prime", 15, false},
		{"100 is not prime", 100, false},
		{"0 is not prime", 0, false},
		{"1 is not prime", 1, false},
		{"-1 is not prime", -1, false},
		{"-5 is not prime", -5, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.IsPrimeNumber(tt.n)
			if result != tt.expected {
				t.Errorf("IsPrimeNumber(%d) = %v, want %v", tt.n, result, tt.expected)
			}
		})
	}
}

func TestPrimeFactorization(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected []int
	}{
		{"factorize 12", 12, []int{2, 2, 3}},
		{"factorize 14", 14, []int{2, 7}},
		{"factorize 15", 15, []int{3, 5}},
		{"factorize 18", 18, []int{2, 3, 3}},
		{"factorize 20", 20, []int{2, 2, 5}},
		{"factorize 16 (powers)", 16, []int{2, 2, 2, 2}},
		{"factorize 27 (powers)", 27, []int{3, 3, 3}},
		{"factorize 32 (powers)", 32, []int{2, 2, 2, 2, 2}},
		{"factorize prime 17", 17, []int{17}},
		{"factorize prime 19", 19, []int{19}},
		{"factorize prime 23", 23, []int{23}},
		{"factorize 0", 0, []int{}},
		{"factorize 1", 1, []int{}},
		{"factorize -12", -12, []int{2, 2, 3}},
		{"factorize -15", -15, []int{3, 5}},
		{"factorize large prime 997", 997, []int{997}},
		{"factorize 1000", 1000, []int{2, 2, 2, 5, 5, 5}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.PrimeFactorization(tt.n)
			if len(result) != len(tt.expected) {
				t.Errorf("PrimeFactorization(%d) = %v, want %v", tt.n, result, tt.expected)
				return
			}
			for i := range result {
				if result[i] != tt.expected[i] {
					t.Errorf("PrimeFactorization(%d) = %v, want %v", tt.n, result, tt.expected)
					return
				}
			}
		})
	}
}

func TestFactorize(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected []int
	}{
		{"factorize 12", 12, []int{1, 2, 3, 4, 6, 12}},
		{"factorize 7 (prime)", 7, []int{1, 7}},
		{"factorize 1", 1, []int{1}},
		{"factorize 0", 0, []int{}},
		{"factorize 25", 25, []int{1, 5, 25}},
		{"factorize 36", 36, []int{1, 2, 3, 4, 6, 9, 12, 18, 36}},
		{"factorize 100", 100, []int{1, 2, 4, 5, 10, 20, 25, 50, 100}},
		{"factorize negative", -12, []int{1, 2, 3, 4, 6, 12}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Factorize(tt.n)
			if len(result) != len(tt.expected) {
				t.Errorf("Factorize(%d) = %v, want %v", tt.n, result, tt.expected)
				return
			}
			for i := range result {
				if result[i] != tt.expected[i] {
					t.Errorf("Factorize(%d) = %v, want %v", tt.n, result, tt.expected)
					return
				}
			}
		})
	}
}

func TestGCD(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"56 and 48", 56, 48, 8},
		{"12 and 18", 12, 18, 6},
		{"48 and 18", 48, 18, 6},
		{"56 and 0", 56, 0, 56},
		{"0 and 56", 0, 56, 56},
		{"0 and 0", 0, 0, 0},
		{"56 and 1", 56, 1, 1},
		{"1 and 56", 1, 56, 1},
		{"1 and 1", 1, 1, 1},
		{"-56 and 48", -56, 48, 8},
		{"56 and -48", 56, -48, 8},
		{"-56 and -48", -56, -48, 8},
		{"1000000 and 500000", 1000000, 500000, 500000},
		{"1 and 2", 1, 2, 1},
		{"2 and 3", 2, 3, 1},
		{"42 and 42", 42, 42, 42},
		{"2 and 4", 2, 4, 2},
		{"3 and 9", 3, 9, 3},
		{"5 and 25", 5, 25, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.GCD(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("GCD(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestLCM(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"4 and 5", 4, 5, 20},
		{"6 and 8", 6, 8, 24},
		{"10 and 15", 10, 15, 30},
		{"7 and 11", 7, 11, 77},
		{"2 and 3", 2, 3, 6},
		{"12 and 18", 12, 18, 36},
		{"0 and 5", 0, 5, 0},
		{"4 and 0", 4, 0, 0},
		{"0 and 0", 0, 0, 0},
		{"1 and 5", 1, 5, 5},
		{"4 and 1", 4, 1, 4},
		{"1 and 1", 1, 1, 1},
		{"1000 and 2000", 1000, 2000, 2000},
		{"999 and 1001", 999, 1001, 999999},
		{"-4 and 6", -4, 6, 12},
		{"4 and -6", 4, -6, 12},
		{"-4 and -6", -4, -6, 12},
		{"5 and 5", 5, 5, 5},
		{"42 and 42", 42, 42, 42},
		{"3 and 5", 3, 5, 15},
		{"13 and 17", 13, 17, 221},
		{"6 and 12", 6, 12, 12},
		{"5 and 25", 5, 25, 25},
		{"4 and 20", 4, 20, 20},
		{"2 and 2", 2, 2, 2},
		{"100 and 200", 100, 200, 200},
		{"17 and 1", 17, 1, 17},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.LCM(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("LCM(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

// ============================================================
// Conversion tests (from conversion_test.go)
// ============================================================

func TestDegToRad(t *testing.T) {
	tests := []struct {
		name     string
		deg      float64
		expected float64
	}{
		{"0 degrees", 0, 0},
		{"90 degrees", 90, stdmath.Pi / 2},
		{"180 degrees", 180, stdmath.Pi},
		{"270 degrees", 270, 3 * stdmath.Pi / 2},
		{"360 degrees", 360, 2 * stdmath.Pi},
		{"-90 degrees", -90, -stdmath.Pi / 2},
		{"-180 degrees", -180, -stdmath.Pi},
		{"-270 degrees", -270, -3 * stdmath.Pi / 2},
		{"-360 degrees", -360, -2 * stdmath.Pi},
		{"45.5 degrees", 45.5, stdmath.Pi/4 + stdmath.Pi/360},
		{"450 degrees", 450, 5 * stdmath.Pi / 2},
		{"720 degrees", 720, 4 * stdmath.Pi},
		{"1080 degrees", 1080, 6 * stdmath.Pi},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.DegToRad(tt.deg)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("DegToRad(%v) = %v, want close to %v", tt.deg, result, tt.expected)
			}
		})
	}

	t.Run("NaN", func(t *testing.T) {
		if !stdmath.IsNaN(umt_math.DegToRad(stdmath.NaN())) {
			t.Error("DegToRad(NaN) should return NaN")
		}
	})

	t.Run("positive infinity", func(t *testing.T) {
		if !stdmath.IsInf(umt_math.DegToRad(stdmath.Inf(1)), 1) {
			t.Error("DegToRad(+Inf) should return +Inf")
		}
	})

	t.Run("negative infinity", func(t *testing.T) {
		if !stdmath.IsInf(umt_math.DegToRad(stdmath.Inf(-1)), -1) {
			t.Error("DegToRad(-Inf) should return -Inf")
		}
	})
}

func TestRadToDeg(t *testing.T) {
	tests := []struct {
		name     string
		rad      float64
		expected float64
	}{
		{"0 radians", 0, 0},
		{"Pi/2", stdmath.Pi / 2, 90},
		{"Pi", stdmath.Pi, 180},
		{"3Pi/2", 3 * stdmath.Pi / 2, 270},
		{"2Pi", 2 * stdmath.Pi, 360},
		{"-Pi/2", -stdmath.Pi / 2, -90},
		{"-Pi", -stdmath.Pi, -180},
		{"-3Pi/2", -3 * stdmath.Pi / 2, -270},
		{"-2Pi", -2 * stdmath.Pi, -360},
		{"Pi/4", stdmath.Pi / 4, 45},
		{"5Pi/2", 5 * stdmath.Pi / 2, 450},
		{"4Pi", 4 * stdmath.Pi, 720},
		{"6Pi", 6 * stdmath.Pi, 1080},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.RadToDeg(tt.rad)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("RadToDeg(%v) = %v, want close to %v", tt.rad, result, tt.expected)
			}
		})
	}

	t.Run("NaN", func(t *testing.T) {
		if !stdmath.IsNaN(umt_math.RadToDeg(stdmath.NaN())) {
			t.Error("RadToDeg(NaN) should return NaN")
		}
	})

	t.Run("positive infinity", func(t *testing.T) {
		if !stdmath.IsInf(umt_math.RadToDeg(stdmath.Inf(1)), 1) {
			t.Error("RadToDeg(+Inf) should return +Inf")
		}
	})

	t.Run("negative infinity", func(t *testing.T) {
		if !stdmath.IsInf(umt_math.RadToDeg(stdmath.Inf(-1)), -1) {
			t.Error("RadToDeg(-Inf) should return -Inf")
		}
	})
}

func TestToBaseN(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		base     int
		expected string
	}{
		{"1 to binary", 1, 2, "1"},
		{"2 to binary", 2, 2, "10"},
		{"3 to binary", 3, 2, "11"},
		{"4 to binary", 4, 2, "100"},
		{"5 to binary", 5, 2, "101"},
		{"6 to binary", 6, 2, "110"},
		{"112 to binary", 112, 2, "1110000"},
		{"7 to base 4", 7, 4, "13"},
		{"8 to base 4", 8, 4, "20"},
		{"9 to base 4", 9, 4, "21"},
		{"112 to base 4", 112, 4, "1300"},
		{"7 to octal", 7, 8, "7"},
		{"8 to octal", 8, 8, "10"},
		{"9 to octal", 9, 8, "11"},
		{"112 to octal", 112, 8, "160"},
		{"10 to hex", 10, 16, "a"},
		{"15 to hex", 15, 16, "f"},
		{"16 to hex", 16, 16, "10"},
		{"255 to hex", 255, 16, "ff"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.ToBaseN(tt.n, tt.base)
			if result != tt.expected {
				t.Errorf("ToBaseN(%d, %d) = %q, want %q", tt.n, tt.base, result, tt.expected)
			}
		})
	}
}

func TestFlexibleNumberConversion(t *testing.T) {
	tests := []struct {
		name     string
		value    string
		fromBase int
		toBase   int
		expected string
		hasError bool
	}{
		{"hex to decimal", "ff", 16, 10, "255", false},
		{"binary to decimal", "1010", 2, 10, "10", false},
		{"decimal to hex", "255", 10, 16, "ff", false},
		{"decimal to binary", "10", 10, 2, "1010", false},
		{"octal to decimal", "77", 8, 10, "63", false},
		{"decimal to octal", "63", 10, 8, "77", false},
		{"hex to binary", "ff", 16, 2, "11111111", false},
		{"binary to hex", "11111111", 2, 16, "ff", false},
		{"invalid input", "xyz", 10, 16, "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := umt_math.FlexibleNumberConversion(tt.value, tt.fromBase, tt.toBase)
			if tt.hasError {
				if err == nil {
					t.Errorf("FlexibleNumberConversion(%q, %d, %d) expected error, got %q", tt.value, tt.fromBase, tt.toBase, result)
				}
				return
			}
			if err != nil {
				t.Errorf("FlexibleNumberConversion(%q, %d, %d) unexpected error: %v", tt.value, tt.fromBase, tt.toBase, err)
				return
			}
			if result != tt.expected {
				t.Errorf("FlexibleNumberConversion(%q, %d, %d) = %q, want %q", tt.value, tt.fromBase, tt.toBase, result, tt.expected)
			}
		})
	}
}

func TestGetDecimalLength(t *testing.T) {
	tests := []struct {
		name     string
		n        float64
		expected int
	}{
		{"integer 1", 1, 0},
		{"integer 100", 100, 0},
		{"negative integer", -42, 0},
		{"1.0 (no decimals)", 1.0, 0},
		{"zero", 0, 0},
		{"-0", -0.0, 0},
		{"1.1", 1.1, 1},
		{"1.11", 1.11, 2},
		{"1.123", 1.123, 3},
		{"1.01", 1.01, 2},
		{"1.001", 1.001, 3},
		{"1.000001", 1.000001, 6},
		{"-1.1", -1.1, 1},
		{"-0.01", -0.01, 2},
		{"123456.789", 123456.789, 3},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.GetDecimalLength(tt.n)
			if result != tt.expected {
				t.Errorf("GetDecimalLength(%v) = %d, want %d", tt.n, result, tt.expected)
			}
		})
	}
}

func TestRoundOf(t *testing.T) {
	tests := []struct {
		name      string
		n         float64
		precision int
		expected  float64
	}{
		{"round to 0 places (down)", 1.111111111111, 0, 1},
		{"round to 0 places (up)", 1.555555555555, 0, 2},
		{"round to 0 places (boundary)", 1.499999999999, 0, 1},
		{"round to 2 places (down)", 1.111111111111, 2, 1.11},
		{"round to 2 places (up)", 1.555555555555, 2, 1.56},
		{"round to 2 places (boundary)", 1.499999999999, 2, 1.5},
		{"round to 1 place", 1.111111111111, 1, 1.1},
		{"round to 3 places", 1.111111111111, 3, 1.111},
		{"round to 4 places", 1.111111111111, 4, 1.1111},
		{"round to 5 places", 1.111111111111, 5, 1.11111},
		{"negative number down", -1.234, 2, -1.23},
		{"negative number up", -1.235, 2, -1.24},
		{"zero precision 0", 0, 0, 0},
		{"zero precision 2", 0, 2, 0},
		{"negative precision -1", 1234.5678, -1, 1230},
		{"negative precision -2", 1234.5678, -2, 1200},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.RoundOf(tt.n, tt.precision)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("RoundOf(%v, %d) = %v, want %v", tt.n, tt.precision, result, tt.expected)
			}
		})
	}
}

func TestMathSeparator(t *testing.T) {
	tests := []struct {
		name     string
		n        float64
		sep      string
		expected string
	}{
		{"thousands", 1234567.89, ",", "1,234,567.89"},
		{"simple thousands", 1000, ",", "1,000"},
		{"no separator needed", 100, ",", "100"},
		{"zero", 0, ",", "0"},
		{"negative", -1234567.89, ",", "-1,234,567.89"},
		{"single digit", 5, ",", "5"},
		{"with different separator", 1234567, ".", "1.234.567"},
		{"millions", 1000000, ",", "1,000,000"},
		{"just decimal", 0.123, ",", "0.123"},
		{"ten thousand", 10000, ",", "10,000"},
		{"hundred thousand", 100000, ",", "100,000"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.MathSeparator(tt.n, tt.sep)
			if result != tt.expected {
				t.Errorf("MathSeparator(%v, %q) = %q, want %q", tt.n, tt.sep, result, tt.expected)
			}
		})
	}
}

// ============================================================
// Bitwise tests (from bitwise_test.go)
// ============================================================

func TestBitwiseAnd(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"5 AND 3", 5, 3, 1},
		{"0xFF AND 0x0F", 0xFF, 0x0F, 0x0F},
		{"12 AND 10", 12, 10, 8},
		{"0 AND 0", 0, 0, 0},
		{"0 AND 5", 0, 5, 0},
		{"-1 AND 5", -1, 5, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.BitwiseAnd(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseAnd(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseOr(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"5 OR 3", 5, 3, 7},
		{"0xF0 OR 0x0F", 0xF0, 0x0F, 0xFF},
		{"12 OR 10", 12, 10, 14},
		{"0 OR 0", 0, 0, 0},
		{"0 OR 5", 0, 5, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.BitwiseOr(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseOr(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseXor(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"5 XOR 3", 5, 3, 6},
		{"0xFF XOR 0x0F", 0xFF, 0x0F, 0xF0},
		{"12 XOR 10", 12, 10, 6},
		{"0 XOR 0", 0, 0, 0},
		{"5 XOR 0", 5, 0, 5},
		{"same values", 42, 42, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.BitwiseXor(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseXor(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseNot(t *testing.T) {
	tests := []struct {
		name     string
		a        int
		expected int
	}{
		{"NOT 0", 0, -1},
		{"NOT 5", 5, -6},
		{"NOT -1", -1, 0},
		{"NOT 1", 1, -2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.BitwiseNot(tt.a)
			if result != tt.expected {
				t.Errorf("BitwiseNot(%d) = %d, want %d", tt.a, result, tt.expected)
			}
		})
	}
}

func TestBitwiseLeftShift(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"1 << 0", 1, 0, 1},
		{"1 << 1", 1, 1, 2},
		{"1 << 3", 1, 3, 8},
		{"5 << 2", 5, 2, 20},
		{"0 << 5", 0, 5, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.BitwiseLeftShift(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseLeftShift(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseRightShift(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"8 >> 0", 8, 0, 8},
		{"8 >> 1", 8, 1, 4},
		{"8 >> 3", 8, 3, 1},
		{"20 >> 2", 20, 2, 5},
		{"0 >> 5", 0, 5, 0},
		{"1 >> 1", 1, 1, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.BitwiseRightShift(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseRightShift(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

// ============================================================
// Equation tests (from equation_test.go)
// ============================================================

func TestSolveEquation(t *testing.T) {
	t.Run("two distinct real roots", func(t *testing.T) {
		result := umt_math.SolveEquation(1, -3, 2)
		if len(result) != 2 {
			t.Fatalf("Expected 2 roots, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-2) > 1e-10 {
			t.Errorf("First root = %v, want 2", result[0])
		}
		if stdmath.Abs(result[1]-1) > 1e-10 {
			t.Errorf("Second root = %v, want 1", result[1])
		}
	})

	t.Run("two distinct roots negative", func(t *testing.T) {
		result := umt_math.SolveEquation(1, 1, -6)
		if len(result) != 2 {
			t.Fatalf("Expected 2 roots, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-2) > 1e-10 {
			t.Errorf("First root = %v, want 2", result[0])
		}
		if stdmath.Abs(result[1]-(-3)) > 1e-10 {
			t.Errorf("Second root = %v, want -3", result[1])
		}
	})

	t.Run("one repeated root", func(t *testing.T) {
		result := umt_math.SolveEquation(1, -2, 1)
		if len(result) != 1 {
			t.Fatalf("Expected 1 root, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-1) > 1e-10 {
			t.Errorf("Root = %v, want 1", result[0])
		}
	})

	t.Run("no real roots", func(t *testing.T) {
		result := umt_math.SolveEquation(1, 0, 1)
		if len(result) != 0 {
			t.Errorf("Expected no roots, got %v", result)
		}
	})

	t.Run("linear equation (a=0)", func(t *testing.T) {
		result := umt_math.SolveEquation(0, 2, -4)
		if len(result) != 1 {
			t.Fatalf("Expected 1 root, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-2) > 1e-10 {
			t.Errorf("Root = %v, want 2", result[0])
		}
	})

	t.Run("constant equation (a=0, b=0)", func(t *testing.T) {
		result := umt_math.SolveEquation(0, 0, 5)
		if len(result) != 0 {
			t.Errorf("Expected no roots, got %v", result)
		}
	})

	t.Run("x^2 - 4 = 0", func(t *testing.T) {
		result := umt_math.SolveEquation(1, 0, -4)
		if len(result) != 2 {
			t.Fatalf("Expected 2 roots, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-2) > 1e-10 {
			t.Errorf("First root = %v, want 2", result[0])
		}
		if stdmath.Abs(result[1]-(-2)) > 1e-10 {
			t.Errorf("Second root = %v, want -2", result[1])
		}
	})

	t.Run("2x^2 + 5x - 3 = 0", func(t *testing.T) {
		result := umt_math.SolveEquation(2, 5, -3)
		if len(result) != 2 {
			t.Fatalf("Expected 2 roots, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-0.5) > 1e-10 {
			t.Errorf("First root = %v, want 0.5", result[0])
		}
		if stdmath.Abs(result[1]-(-3)) > 1e-10 {
			t.Errorf("Second root = %v, want -3", result[1])
		}
	})
}

// ============================================================
// Random tests (from random_test.go)
// ============================================================

func TestRandom(t *testing.T) {
	tests := []struct {
		name    string
		max     int
		min     []int
		wantMin int
		wantMax int
	}{
		{
			name:    "basic range test (0-10)",
			max:     10,
			min:     nil,
			wantMin: 0,
			wantMax: 10,
		},
		{
			name:    "min and max specified (5-10)",
			max:     10,
			min:     []int{5},
			wantMin: 5,
			wantMax: 10,
		},
		{
			name:    "min greater than max (10-5 becomes 5-10)",
			max:     5,
			min:     []int{10},
			wantMin: 5,
			wantMax: 10,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			for i := 0; i < 100; i++ {
				got, err := umt_math.Random(tt.max, tt.min...)
				if err != nil {
					t.Errorf("Random() error = %v", err)
					return
				}
				if got < tt.wantMin || got > tt.wantMax {
					t.Errorf("Random() = %v, want range [%v, %v]", got, tt.wantMin, tt.wantMax)
				}
			}
		})
	}
}

func TestMustRandom(t *testing.T) {
	t.Run("normal case", func(t *testing.T) {
		for i := 0; i < 100; i++ {
			got := umt_math.MustRandom(10, 5)
			if got < 5 || got > 10 {
				t.Errorf("MustRandom() = %v, want range [5, 10]", got)
			}
		}
	})
}

func TestRandomDistribution(t *testing.T) {
	counts := make([]int, 10)
	trials := 1000

	for i := 0; i < trials; i++ {
		n, err := umt_math.Random(9)
		if err != nil {
			t.Fatalf("Random() error = %v", err)
		}
		counts[n]++
	}

	expectedMin := trials / 10 * 8 / 10  // 8%
	expectedMax := trials / 10 * 12 / 10 // 12%

	for i, count := range counts {
		if count < expectedMin || count > expectedMax {
			t.Logf("Warning: number %d appeared %d times (expected range: %d-%d)", i, count, expectedMin, expectedMax)
		}
	}
}

// ============================================================
// LCG tests (from lcg_test.go)
// ============================================================

func TestLinearCongruentialGenerator(t *testing.T) {
	t.Run("generates values in [0, 1)", func(t *testing.T) {
		rng := umt_math.LinearCongruentialGenerator(12345)
		for i := 0; i < 100; i++ {
			val := rng()
			if val < 0 || val >= 1 {
				t.Errorf("Value %v out of range [0, 1)", val)
			}
		}
	})

	t.Run("same seed produces same sequence", func(t *testing.T) {
		rng1 := umt_math.LinearCongruentialGenerator(12345)
		rng2 := umt_math.LinearCongruentialGenerator(12345)
		for i := 0; i < 10; i++ {
			v1 := rng1()
			v2 := rng2()
			if v1 != v2 {
				t.Errorf("Different values at step %d: %v != %v", i, v1, v2)
			}
		}
	})

	t.Run("different seeds produce different sequences", func(t *testing.T) {
		rng1 := umt_math.LinearCongruentialGenerator(12345)
		rng2 := umt_math.LinearCongruentialGenerator(54321)
		v1 := rng1()
		v2 := rng2()
		if v1 == v2 {
			t.Errorf("Expected different values, both got %v", v1)
		}
	})

	t.Run("produces deterministic first value", func(t *testing.T) {
		rng := umt_math.LinearCongruentialGenerator(12345)
		val := rng()
		// (1664525 * 12345 + 1013904223) % 4294967296 = 87628868
		expected := 87628868.0 / 4294967296.0
		if val != expected {
			t.Errorf("First value = %v, want %v", val, expected)
		}
	})

	t.Run("subsequent calls advance state", func(t *testing.T) {
		rng := umt_math.LinearCongruentialGenerator(42)
		v1 := rng()
		v2 := rng()
		v3 := rng()
		if v1 == v2 || v2 == v3 || v1 == v3 {
			t.Errorf("Values should differ: %v, %v, %v", v1, v2, v3)
		}
	})
}

// ============================================================
// Xoshiro tests (from xoshiro_test.go)
// ============================================================

func TestXoshiro256(t *testing.T) {
	t.Run("generates values in [0, 1)", func(t *testing.T) {
		rng := umt_math.Xoshiro256(12345)
		for i := 0; i < 1000; i++ {
			val := rng()
			if val < 0 || val >= 1 {
				t.Errorf("Value %v out of range [0, 1) at step %d", val, i)
			}
		}
	})

	t.Run("same seed produces same sequence", func(t *testing.T) {
		rng1 := umt_math.Xoshiro256(12345)
		rng2 := umt_math.Xoshiro256(12345)
		for i := 0; i < 10; i++ {
			v1 := rng1()
			v2 := rng2()
			if v1 != v2 {
				t.Errorf("Different values at step %d: %v != %v", i, v1, v2)
			}
		}
	})

	t.Run("different seeds produce different sequences", func(t *testing.T) {
		rng1 := umt_math.Xoshiro256(12345)
		rng2 := umt_math.Xoshiro256(54321)
		v1 := rng1()
		v2 := rng2()
		if v1 == v2 {
			t.Errorf("Expected different values, both got %v", v1)
		}
	})

	t.Run("subsequent calls advance state", func(t *testing.T) {
		rng := umt_math.Xoshiro256(42)
		v1 := rng()
		v2 := rng()
		v3 := rng()
		if v1 == v2 || v2 == v3 {
			t.Errorf("Values should differ: %v, %v, %v", v1, v2, v3)
		}
	})

	t.Run("produces reasonable distribution", func(t *testing.T) {
		rng := umt_math.Xoshiro256(99)
		bins := make([]int, 10)
		n := 10000
		for i := 0; i < n; i++ {
			val := rng()
			bin := int(val * 10)
			if bin >= 10 {
				bin = 9
			}
			bins[bin]++
		}
		for i, count := range bins {
			if count < 500 || count > 1500 {
				t.Errorf("Bin %d has %d entries, expected ~1000", i, count)
			}
		}
	})
}

// ============================================================
// NEW TESTS: Multiples
// ============================================================

func TestMultiples(t *testing.T) {
	tests := []struct {
		name     string
		x        float64
		n        int
		expected []float64
	}{
		{"multiples of 2 up to 5", 2, 5, []float64{2, 4, 6, 8, 10}},
		{"multiples of 1 up to 3", 1, 3, []float64{1, 2, 3}},
		{"multiples of -2 up to 3", -2, 3, []float64{-2, -4, -6}},
		{"multiples of 0.5 up to 3", 0.5, 3, []float64{0.5, 1.0, 1.5}},
		{"multiples of 10 up to 4", 10, 4, []float64{10, 20, 30, 40}},
		{"multiples of 3 up to 1", 3, 1, []float64{3}},
		{"multiples with n=0", 5, 0, []float64{}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.Multiples(tt.x, tt.n)
			if len(result) != len(tt.expected) {
				t.Errorf("Multiples(%v, %d) = %v, want %v", tt.x, tt.n, result, tt.expected)
				return
			}
			for i := range result {
				if stdmath.Abs(result[i]-tt.expected[i]) > 1e-10 {
					t.Errorf("Multiples(%v, %d)[%d] = %v, want %v", tt.x, tt.n, i, result[i], tt.expected[i])
					return
				}
			}
		})
	}
}

// ============================================================
// NEW TESTS: Reduce
// ============================================================

func TestReduce(t *testing.T) {
	t.Run("basic reduction", func(t *testing.T) {
		result, err := umt_math.Reduce(2, 4)
		if err != nil {
			t.Fatalf("Reduce(2, 4) unexpected error: %v", err)
		}
		if result.X != 1 || result.Y != 2 || result.GCD != 2 {
			t.Errorf("Reduce(2, 4) = {X:%d, Y:%d, GCD:%d}, want {X:1, Y:2, GCD:2}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("reduce 3/6", func(t *testing.T) {
		result, err := umt_math.Reduce(3, 6)
		if err != nil {
			t.Fatalf("Reduce(3, 6) unexpected error: %v", err)
		}
		if result.X != 1 || result.Y != 2 || result.GCD != 3 {
			t.Errorf("Reduce(3, 6) = {X:%d, Y:%d, GCD:%d}, want {X:1, Y:2, GCD:3}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("zero numerator", func(t *testing.T) {
		result, err := umt_math.Reduce(0, 5)
		if err != nil {
			t.Fatalf("Reduce(0, 5) unexpected error: %v", err)
		}
		if result.X != 0 || result.Y != 1 || result.GCD != 5 {
			t.Errorf("Reduce(0, 5) = {X:%d, Y:%d, GCD:%d}, want {X:0, Y:1, GCD:5}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("zero denominator returns error", func(t *testing.T) {
		_, err := umt_math.Reduce(5, 0)
		if err == nil {
			t.Error("Reduce(5, 0) expected error, got nil")
		}
	})

	t.Run("negative numerator", func(t *testing.T) {
		result, err := umt_math.Reduce(-6, 8)
		if err != nil {
			t.Fatalf("Reduce(-6, 8) unexpected error: %v", err)
		}
		if result.X != -3 || result.Y != 4 || result.GCD != 2 {
			t.Errorf("Reduce(-6, 8) = {X:%d, Y:%d, GCD:%d}, want {X:-3, Y:4, GCD:2}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("negative denominator", func(t *testing.T) {
		result, err := umt_math.Reduce(6, -8)
		if err != nil {
			t.Fatalf("Reduce(6, -8) unexpected error: %v", err)
		}
		if result.X != -3 || result.Y != 4 || result.GCD != 2 {
			t.Errorf("Reduce(6, -8) = {X:%d, Y:%d, GCD:%d}, want {X:-3, Y:4, GCD:2}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("both negative", func(t *testing.T) {
		result, err := umt_math.Reduce(-6, -8)
		if err != nil {
			t.Fatalf("Reduce(-6, -8) unexpected error: %v", err)
		}
		if result.X != 3 || result.Y != 4 || result.GCD != 2 {
			t.Errorf("Reduce(-6, -8) = {X:%d, Y:%d, GCD:%d}, want {X:3, Y:4, GCD:2}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("already reduced", func(t *testing.T) {
		result, err := umt_math.Reduce(3, 7)
		if err != nil {
			t.Fatalf("Reduce(3, 7) unexpected error: %v", err)
		}
		if result.X != 3 || result.Y != 7 || result.GCD != 1 {
			t.Errorf("Reduce(3, 7) = {X:%d, Y:%d, GCD:%d}, want {X:3, Y:7, GCD:1}", result.X, result.Y, result.GCD)
		}
	})

	t.Run("equal values", func(t *testing.T) {
		result, err := umt_math.Reduce(5, 5)
		if err != nil {
			t.Fatalf("Reduce(5, 5) unexpected error: %v", err)
		}
		if result.X != 1 || result.Y != 1 || result.GCD != 5 {
			t.Errorf("Reduce(5, 5) = {X:%d, Y:%d, GCD:%d}, want {X:1, Y:1, GCD:5}", result.X, result.Y, result.GCD)
		}
	})
}

// ============================================================
// NEW TESTS: ToCelsius and ToKelvin
// ============================================================

func TestToCelsius(t *testing.T) {
	tests := []struct {
		name     string
		kelvin   float64
		expected float64
	}{
		{"absolute zero", 0, -273.15},
		{"water freezing point", 273.15, 0},
		{"water boiling point", 373.15, 100},
		{"room temperature", 300, 26.85},
		{"body temperature", 310.15, 37},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.ToCelsius(tt.kelvin)
			if stdmath.Abs(result-tt.expected) > 1e-9 {
				t.Errorf("ToCelsius(%v) = %v, want %v", tt.kelvin, result, tt.expected)
			}
		})
	}
}

func TestToKelvin(t *testing.T) {
	tests := []struct {
		name     string
		celsius  float64
		expected float64
	}{
		{"absolute zero", -273.15, 0},
		{"water freezing point", 0, 273.15},
		{"water boiling point", 100, 373.15},
		{"room temperature", 26.85, 300},
		{"body temperature", 37, 310.15},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := umt_math.ToKelvin(tt.celsius)
			if stdmath.Abs(result-tt.expected) > 1e-9 {
				t.Errorf("ToKelvin(%v) = %v, want %v", tt.celsius, result, tt.expected)
			}
		})
	}
}

// ============================================================
// NEW TESTS: UuidV7
// ============================================================

func TestUuidV7(t *testing.T) {
	t.Run("correct length", func(t *testing.T) {
		uuid := umt_math.UuidV7()
		if len(uuid) != 36 {
			t.Errorf("UuidV7() length = %d, want 36", len(uuid))
		}
	})

	t.Run("correct dash positions", func(t *testing.T) {
		uuid := umt_math.UuidV7()
		dashPositions := []int{8, 13, 18, 23}
		for _, pos := range dashPositions {
			if uuid[pos] != '-' {
				t.Errorf("UuidV7()[%d] = %c, want '-'", pos, uuid[pos])
			}
		}
	})

	t.Run("version 7", func(t *testing.T) {
		uuid := umt_math.UuidV7()
		// Position 14 should be '7' (version)
		if uuid[14] != '7' {
			t.Errorf("UuidV7() version char = %c, want '7'", uuid[14])
		}
	})

	t.Run("variant 2", func(t *testing.T) {
		uuid := umt_math.UuidV7()
		// Position 19 should be '8', '9', 'a', or 'b' (variant 2)
		variant := uuid[19]
		if variant != '8' && variant != '9' && variant != 'a' && variant != 'b' {
			t.Errorf("UuidV7() variant char = %c, want one of '8', '9', 'a', 'b'", variant)
		}
	})

	t.Run("uniqueness", func(t *testing.T) {
		uuid1 := umt_math.UuidV7()
		uuid2 := umt_math.UuidV7()
		if uuid1 == uuid2 {
			t.Errorf("Two UuidV7() calls returned same value: %s", uuid1)
		}
	})

	t.Run("valid hex characters", func(t *testing.T) {
		uuid := umt_math.UuidV7()
		for i, ch := range uuid {
			if i == 8 || i == 13 || i == 18 || i == 23 {
				continue // skip dashes
			}
			if !((ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f')) {
				t.Errorf("UuidV7()[%d] = %c, want hex character", i, ch)
			}
		}
	})
}

// ============================================================
// NEW TESTS: MathConverter
// ============================================================

func TestMathConverter(t *testing.T) {
	t.Run("1250*1250 contains plus signs", func(t *testing.T) {
		result := umt_math.MathConverter("1250*1250")
		if !strings.Contains(result, "+") {
			t.Errorf("MathConverter(\"1250*1250\") = %q, expected result to contain '+'", result)
		}
	})

	t.Run("1250*1250 exact result", func(t *testing.T) {
		result := umt_math.MathConverter("1250*1250")
		expected := "1500*1000+400*100+200*100+50*50"
		if result != expected {
			t.Errorf("MathConverter(\"1250*1250\") = %q, want %q", result, expected)
		}
	})

	t.Run("1250^2 exponent notation", func(t *testing.T) {
		result := umt_math.MathConverter("1250^2")
		if !strings.Contains(result, "+") {
			t.Errorf("MathConverter(\"1250^2\") = %q, expected result to contain '+'", result)
		}
	})

	t.Run("no conversion needed", func(t *testing.T) {
		result := umt_math.MathConverter("1250")
		if result != "1250" {
			t.Errorf("MathConverter(\"1250\") = %q, want \"1250\"", result)
		}
	})

	t.Run("invalid input returned as-is", func(t *testing.T) {
		result := umt_math.MathConverter("abc")
		if result != "abc" {
			t.Errorf("MathConverter(\"abc\") = %q, want \"abc\"", result)
		}
	})

	t.Run("different operands not converted", func(t *testing.T) {
		result := umt_math.MathConverter("100*200")
		if result != "100*200" {
			t.Errorf("MathConverter(\"100*200\") = %q, want \"100*200\"", result)
		}
	})
}

// ============================================================
// NEW TESTS: BitwiseRotate
// ============================================================

func TestBitwiseRotate(t *testing.T) {
	t.Run("left rotate 0x12345678 by 8", func(t *testing.T) {
		result := umt_math.BitwiseRotate(0x12345678, 8, "left")
		expected := int32(0x34567812)
		if result != expected {
			t.Errorf("BitwiseRotate(0x12345678, 8, \"left\") = 0x%x, want 0x%x", result, expected)
		}
	})

	t.Run("right rotate 0x12345678 by 8", func(t *testing.T) {
		result := umt_math.BitwiseRotate(0x12345678, 8, "right")
		expected := int32(0x78123456)
		if result != expected {
			t.Errorf("BitwiseRotate(0x12345678, 8, \"right\") = 0x%x, want 0x%x", result, expected)
		}
	})

	t.Run("default direction is left", func(t *testing.T) {
		result := umt_math.BitwiseRotate(0x12345678, 8, "")
		expected := int32(0x34567812)
		if result != expected {
			t.Errorf("BitwiseRotate(0x12345678, 8, \"\") = 0x%x, want 0x%x", result, expected)
		}
	})

	t.Run("rotate by 0", func(t *testing.T) {
		result := umt_math.BitwiseRotate(0x12345678, 0, "left")
		expected := int32(0x12345678)
		if result != expected {
			t.Errorf("BitwiseRotate(0x12345678, 0, \"left\") = 0x%x, want 0x%x", result, expected)
		}
	})

	t.Run("rotate by 32 is identity", func(t *testing.T) {
		result := umt_math.BitwiseRotate(0x12345678, 32, "left")
		expected := int32(0x12345678)
		if result != expected {
			t.Errorf("BitwiseRotate(0x12345678, 32, \"left\") = 0x%x, want 0x%x", result, expected)
		}
	})

	t.Run("invalid direction panics", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("BitwiseRotate with invalid direction should panic")
			}
		}()
		umt_math.BitwiseRotate(0x12345678, 8, "up")
	})
}
