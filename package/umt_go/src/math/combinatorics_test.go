package math

import (
	stdmath "math"
	"testing"
)

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
			result := Factorial(tt.n)
			if result != tt.expected {
				t.Errorf("Factorial(%d) = %d, want %d", tt.n, result, tt.expected)
			}
		})
	}
}

func TestNCr(t *testing.T) {
	t.Run("basic combinations", func(t *testing.T) {
		if got := NCr(5, 2); got != 10 {
			t.Errorf("NCr(5,2) = %d, want 10", got)
		}
		if got := NCr(10, 4); got != 210 {
			t.Errorf("NCr(10,4) = %d, want 210", got)
		}
	})

	t.Run("choosing all items", func(t *testing.T) {
		if got := NCr(3, 3); got != 1 {
			t.Errorf("NCr(3,3) = %d, want 1", got)
		}
		if got := NCr(5, 5); got != 1 {
			t.Errorf("NCr(5,5) = %d, want 1", got)
		}
	})

	t.Run("choosing one item", func(t *testing.T) {
		if got := NCr(5, 1); got != 5 {
			t.Errorf("NCr(5,1) = %d, want 5", got)
		}
		if got := NCr(10, 1); got != 10 {
			t.Errorf("NCr(10,1) = %d, want 10", got)
		}
	})

	t.Run("larger combinations", func(t *testing.T) {
		if got := NCr(20, 10); got != 184756 {
			t.Errorf("NCr(20,10) = %d, want 184756", got)
		}
		if got := NCr(15, 7); got != 6435 {
			t.Errorf("NCr(15,7) = %d, want 6435", got)
		}
	})

	t.Run("choosing nothing", func(t *testing.T) {
		if got := NCr(5, 0); got != 1 {
			t.Errorf("NCr(5,0) = %d, want 1", got)
		}
		if got := NCr(10, 0); got != 1 {
			t.Errorf("NCr(10,0) = %d, want 1", got)
		}
	})

	t.Run("invalid inputs return -1", func(t *testing.T) {
		if got := NCr(0, 5); got != -1 {
			t.Errorf("NCr(0,5) = %d, want -1", got)
		}
		if got := NCr(2, 5); got != -1 {
			t.Errorf("NCr(2,5) = %d, want -1", got)
		}
		if got := NCr(3, 4); got != -1 {
			t.Errorf("NCr(3,4) = %d, want -1", got)
		}
		if got := NCr(-1, 2); got != -1 {
			t.Errorf("NCr(-1,2) = %d, want -1", got)
		}
		if got := NCr(5, -1); got != -1 {
			t.Errorf("NCr(5,-1) = %d, want -1", got)
		}
	})

	t.Run("n=0, r=0", func(t *testing.T) {
		if got := NCr(0, 0); got != 1 {
			t.Errorf("NCr(0,0) = %d, want 1", got)
		}
	})
}

func TestNPr(t *testing.T) {
	t.Run("basic permutations", func(t *testing.T) {
		if got := NPr(5, 2); got != 20 {
			t.Errorf("NPr(5,2) = %d, want 20", got)
		}
		if got := NPr(10, 4); got != 5040 {
			t.Errorf("NPr(10,4) = %d, want 5040", got)
		}
	})

	t.Run("choosing one item", func(t *testing.T) {
		if got := NPr(3, 1); got != 3 {
			t.Errorf("NPr(3,1) = %d, want 3", got)
		}
		if got := NPr(5, 1); got != 5 {
			t.Errorf("NPr(5,1) = %d, want 5", got)
		}
	})

	t.Run("choosing all items", func(t *testing.T) {
		if got := NPr(3, 3); got != 6 {
			t.Errorf("NPr(3,3) = %d, want 6", got)
		}
		if got := NPr(4, 4); got != 24 {
			t.Errorf("NPr(4,4) = %d, want 24", got)
		}
	})

	t.Run("larger permutations", func(t *testing.T) {
		if got := NPr(8, 3); got != 336 {
			t.Errorf("NPr(8,3) = %d, want 336", got)
		}
		if got := NPr(6, 4); got != 360 {
			t.Errorf("NPr(6,4) = %d, want 360", got)
		}
	})

	t.Run("arranging nothing", func(t *testing.T) {
		if got := NPr(5, 0); got != 1 {
			t.Errorf("NPr(5,0) = %d, want 1", got)
		}
		if got := NPr(10, 0); got != 1 {
			t.Errorf("NPr(10,0) = %d, want 1", got)
		}
		if got := NPr(0, 0); got != 1 {
			t.Errorf("NPr(0,0) = %d, want 1", got)
		}
	})

	t.Run("invalid inputs", func(t *testing.T) {
		if got := NPr(0, 5); got != -1 {
			t.Errorf("NPr(0,5) = %d, want -1", got)
		}
		if got := NPr(2, 5); got != -1 {
			t.Errorf("NPr(2,5) = %d, want -1", got)
		}
		if got := NPr(-1, 2); got != -1 {
			t.Errorf("NPr(-1,2) = %d, want -1", got)
		}
		if got := NPr(5, -1); got != -1 {
			t.Errorf("NPr(5,-1) = %d, want -1", got)
		}
	})

	t.Run("n equals r", func(t *testing.T) {
		if got := NPr(2, 2); got != 2 {
			t.Errorf("NPr(2,2) = %d, want 2", got)
		}
		if got := NPr(5, 5); got != 120 {
			t.Errorf("NPr(5,5) = %d, want 120", got)
		}
	})
}

func TestNHr(t *testing.T) {
	t.Run("basic combinations with repetition", func(t *testing.T) {
		if got := NHr(5, 2); got != 15 {
			t.Errorf("NHr(5,2) = %d, want 15", got)
		}
		if got := NHr(3, 3); got != 10 {
			t.Errorf("NHr(3,3) = %d, want 10", got)
		}
	})

	t.Run("choosing one item", func(t *testing.T) {
		if got := NHr(3, 1); got != 3 {
			t.Errorf("NHr(3,1) = %d, want 3", got)
		}
		if got := NHr(5, 1); got != 5 {
			t.Errorf("NHr(5,1) = %d, want 5", got)
		}
	})

	t.Run("choosing same as total items", func(t *testing.T) {
		if got := NHr(2, 2); got != 3 {
			t.Errorf("NHr(2,2) = %d, want 3", got)
		}
		if got := NHr(4, 4); got != 35 {
			t.Errorf("NHr(4,4) = %d, want 35", got)
		}
	})

	t.Run("larger combinations", func(t *testing.T) {
		if got := NHr(10, 3); got != 220 {
			t.Errorf("NHr(10,3) = %d, want 220", got)
		}
		if got := NHr(6, 4); got != 126 {
			t.Errorf("NHr(6,4) = %d, want 126", got)
		}
	})

	t.Run("invalid when n or r is 0", func(t *testing.T) {
		if got := NHr(0, 5); got != -1 {
			t.Errorf("NHr(0,5) = %d, want -1", got)
		}
		if got := NHr(5, 0); got != -1 {
			t.Errorf("NHr(5,0) = %d, want -1", got)
		}
		if got := NHr(0, 0); got != -1 {
			t.Errorf("NHr(0,0) = %d, want -1", got)
		}
	})

	t.Run("invalid when negative", func(t *testing.T) {
		if got := NHr(-1, 5); got != -1 {
			t.Errorf("NHr(-1,5) = %d, want -1", got)
		}
		if got := NHr(5, -1); got != -1 {
			t.Errorf("NHr(5,-1) = %d, want -1", got)
		}
	})
}

func TestRepeatedTrial(t *testing.T) {
	t.Run("basic binomial probability", func(t *testing.T) {
		// P(X=2) in 5 trials with p=0.5: C(5,2) * 0.5^2 * 0.5^3 = 10 * 0.03125 = 0.3125
		result := RepeatedTrial(5, 0.5, 2)
		expected := 0.3125
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(5, 0.5, 2) = %v, want %v", result, expected)
		}
	})

	t.Run("probability 1/3", func(t *testing.T) {
		// P(X=2) in 4 trials with p=1/3: C(4,2) * (1/3)^2 * (2/3)^2
		result := RepeatedTrial(4, 1.0/3.0, 2)
		expected := 6.0 * (1.0 / 9.0) * (4.0 / 9.0)
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(4, 1/3, 2) = %v, want %v", result, expected)
		}
	})

	t.Run("all successes", func(t *testing.T) {
		// P(X=3) in 3 trials with p=2/3: C(3,3) * (2/3)^3 * (1/3)^0
		result := RepeatedTrial(3, 2.0/3.0, 3)
		expected := 8.0 / 27.0
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(3, 2/3, 3) = %v, want %v", result, expected)
		}
	})

	t.Run("zero successes", func(t *testing.T) {
		// P(X=0) in 3 trials with p=0.5: C(3,0) * 0.5^0 * 0.5^3 = 0.125
		result := RepeatedTrial(3, 0.5, 0)
		expected := 0.125
		if stdmath.Abs(result-expected) > 1e-10 {
			t.Errorf("RepeatedTrial(3, 0.5, 0) = %v, want %v", result, expected)
		}
	})
}
