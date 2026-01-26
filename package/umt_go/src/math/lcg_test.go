package math

import (
	"testing"
)

func TestLinearCongruentialGenerator(t *testing.T) {
	t.Run("generates values in [0, 1)", func(t *testing.T) {
		rng := LinearCongruentialGenerator(12345)
		for i := 0; i < 100; i++ {
			val := rng()
			if val < 0 || val >= 1 {
				t.Errorf("Value %v out of range [0, 1)", val)
			}
		}
	})

	t.Run("same seed produces same sequence", func(t *testing.T) {
		rng1 := LinearCongruentialGenerator(12345)
		rng2 := LinearCongruentialGenerator(12345)
		for i := 0; i < 10; i++ {
			v1 := rng1()
			v2 := rng2()
			if v1 != v2 {
				t.Errorf("Different values at step %d: %v != %v", i, v1, v2)
			}
		}
	})

	t.Run("different seeds produce different sequences", func(t *testing.T) {
		rng1 := LinearCongruentialGenerator(12345)
		rng2 := LinearCongruentialGenerator(54321)
		v1 := rng1()
		v2 := rng2()
		if v1 == v2 {
			t.Errorf("Expected different values, both got %v", v1)
		}
	})

	t.Run("produces deterministic first value", func(t *testing.T) {
		rng := LinearCongruentialGenerator(12345)
		val := rng()
		// (1664525 * 12345 + 1013904223) % 4294967296 = 87628868
		expected := 87628868.0 / 4294967296.0
		if val != expected {
			t.Errorf("First value = %v, want %v", val, expected)
		}
	})

	t.Run("subsequent calls advance state", func(t *testing.T) {
		rng := LinearCongruentialGenerator(42)
		v1 := rng()
		v2 := rng()
		v3 := rng()
		// All values should be different
		if v1 == v2 || v2 == v3 || v1 == v3 {
			t.Errorf("Values should differ: %v, %v, %v", v1, v2, v3)
		}
	})
}
