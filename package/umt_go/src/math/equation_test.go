package math

import (
	stdmath "math"
	"testing"
)

func TestSolveEquation(t *testing.T) {
	t.Run("two distinct real roots", func(t *testing.T) {
		// x^2 - 3x + 2 = 0 => x=2, x=1
		result := SolveEquation(1, -3, 2)
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
		// x^2 + x - 6 = 0 => x=2, x=-3
		result := SolveEquation(1, 1, -6)
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
		// x^2 - 2x + 1 = 0 => x=1 (double root)
		result := SolveEquation(1, -2, 1)
		if len(result) != 1 {
			t.Fatalf("Expected 1 root, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-1) > 1e-10 {
			t.Errorf("Root = %v, want 1", result[0])
		}
	})

	t.Run("no real roots", func(t *testing.T) {
		// x^2 + 1 = 0 => no real roots
		result := SolveEquation(1, 0, 1)
		if len(result) != 0 {
			t.Errorf("Expected no roots, got %v", result)
		}
	})

	t.Run("linear equation (a=0)", func(t *testing.T) {
		// 2x - 4 = 0 => x=2
		result := SolveEquation(0, 2, -4)
		if len(result) != 1 {
			t.Fatalf("Expected 1 root, got %d: %v", len(result), result)
		}
		if stdmath.Abs(result[0]-2) > 1e-10 {
			t.Errorf("Root = %v, want 2", result[0])
		}
	})

	t.Run("constant equation (a=0, b=0)", func(t *testing.T) {
		// 5 = 0 => no solution
		result := SolveEquation(0, 0, 5)
		if len(result) != 0 {
			t.Errorf("Expected no roots, got %v", result)
		}
	})

	t.Run("x^2 - 4 = 0", func(t *testing.T) {
		// x^2 - 4 = 0 => x=2, x=-2
		result := SolveEquation(1, 0, -4)
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
		// 2x^2 + 5x - 3 = 0 => x=0.5, x=-3
		result := SolveEquation(2, 5, -3)
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
