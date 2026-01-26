package math

import (
	stdmath "math"
	"testing"
)

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
			result := Addition(tt.args...)
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
			result := Subtraction(tt.args...)
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
			result := Multiplication(tt.args...)
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
			result := Division(tt.args...)
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
			result := Quotient(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("Quotient(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestMax(t *testing.T) {
	t.Run("basic max", func(t *testing.T) {
		if got := Max(1, 2, 3); got != 3 {
			t.Errorf("Max(1,2,3) = %v, want 3", got)
		}
		if got := Max(3, 2, 1); got != 3 {
			t.Errorf("Max(3,2,1) = %v, want 3", got)
		}
		if got := Max(-1, -2, -3); got != -1 {
			t.Errorf("Max(-1,-2,-3) = %v, want -1", got)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		if got := Max(1, 1, 1); got != 1 {
			t.Errorf("Max(1,1,1) = %v, want 1", got)
		}
		if got := Max(1, 2, 2, 3, 3); got != 3 {
			t.Errorf("Max(1,2,2,3,3) = %v, want 3", got)
		}
	})

	t.Run("single value", func(t *testing.T) {
		if got := Max(5); got != 5 {
			t.Errorf("Max(5) = %v, want 5", got)
		}
		if got := Max(-5); got != -5 {
			t.Errorf("Max(-5) = %v, want -5", got)
		}
	})

	t.Run("float64 max", func(t *testing.T) {
		if got := Max(1.5, 2.5, 1.1); got != 2.5 {
			t.Errorf("Max(1.5,2.5,1.1) = %v, want 2.5", got)
		}
		if got := Max(-1.5, -2.5, -1.1); got != -1.1 {
			t.Errorf("Max(-1.5,-2.5,-1.1) = %v, want -1.1", got)
		}
	})
}

func TestMin(t *testing.T) {
	t.Run("basic min", func(t *testing.T) {
		if got := Min(1, 2, 3); got != 1 {
			t.Errorf("Min(1,2,3) = %v, want 1", got)
		}
		if got := Min(3, 2, 1); got != 1 {
			t.Errorf("Min(3,2,1) = %v, want 1", got)
		}
		if got := Min(-1, -2, -3); got != -3 {
			t.Errorf("Min(-1,-2,-3) = %v, want -3", got)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		if got := Min(1, 1, 1); got != 1 {
			t.Errorf("Min(1,1,1) = %v, want 1", got)
		}
		if got := Min(1, 2, 2, 3, 3); got != 1 {
			t.Errorf("Min(1,2,2,3,3) = %v, want 1", got)
		}
	})

	t.Run("single value", func(t *testing.T) {
		if got := Min(5); got != 5 {
			t.Errorf("Min(5) = %v, want 5", got)
		}
		if got := Min(-5); got != -5 {
			t.Errorf("Min(-5) = %v, want -5", got)
		}
	})

	t.Run("float64 min", func(t *testing.T) {
		if got := Min(1.5, 2.5, 1.1); got != 1.1 {
			t.Errorf("Min(1.5,2.5,1.1) = %v, want 1.1", got)
		}
		if got := Min(-1.5, -2.5, -1.1); got != -2.5 {
			t.Errorf("Min(-1.5,-2.5,-1.1) = %v, want -2.5", got)
		}
	})
}

func TestValueSwap(t *testing.T) {
	t.Run("swap integers", func(t *testing.T) {
		a, b := 1, 2
		ValueSwap(&a, &b)
		if a != 2 || b != 1 {
			t.Errorf("ValueSwap(&1, &2): a=%d, b=%d, want a=2, b=1", a, b)
		}
	})

	t.Run("swap floats", func(t *testing.T) {
		a, b := 1.5, 2.5
		ValueSwap(&a, &b)
		if a != 2.5 || b != 1.5 {
			t.Errorf("ValueSwap(&1.5, &2.5): a=%f, b=%f, want a=2.5, b=1.5", a, b)
		}
	})

	t.Run("swap strings", func(t *testing.T) {
		a, b := "hello", "world"
		ValueSwap(&a, &b)
		if a != "world" || b != "hello" {
			t.Errorf("ValueSwap: a=%s, b=%s, want a=world, b=hello", a, b)
		}
	})

	t.Run("swap equal values", func(t *testing.T) {
		a, b := 5, 5
		ValueSwap(&a, &b)
		if a != 5 || b != 5 {
			t.Errorf("ValueSwap(&5, &5): a=%d, b=%d, want a=5, b=5", a, b)
		}
	})

	t.Run("swap negative values", func(t *testing.T) {
		a, b := -5, 5
		ValueSwap(&a, &b)
		if a != 5 || b != -5 {
			t.Errorf("ValueSwap(&-5, &5): a=%d, b=%d, want a=5, b=-5", a, b)
		}
	})
}
