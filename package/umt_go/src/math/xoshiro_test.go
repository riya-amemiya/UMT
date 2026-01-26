package math

import (
	"testing"
)

func TestXoshiro256(t *testing.T) {
	t.Run("generates values in [0, 1)", func(t *testing.T) {
		rng := Xoshiro256(12345)
		for i := 0; i < 1000; i++ {
			val := rng()
			if val < 0 || val >= 1 {
				t.Errorf("Value %v out of range [0, 1) at step %d", val, i)
			}
		}
	})

	t.Run("same seed produces same sequence", func(t *testing.T) {
		rng1 := Xoshiro256(12345)
		rng2 := Xoshiro256(12345)
		for i := 0; i < 10; i++ {
			v1 := rng1()
			v2 := rng2()
			if v1 != v2 {
				t.Errorf("Different values at step %d: %v != %v", i, v1, v2)
			}
		}
	})

	t.Run("different seeds produce different sequences", func(t *testing.T) {
		rng1 := Xoshiro256(12345)
		rng2 := Xoshiro256(54321)
		v1 := rng1()
		v2 := rng2()
		if v1 == v2 {
			t.Errorf("Expected different values, both got %v", v1)
		}
	})

	t.Run("subsequent calls advance state", func(t *testing.T) {
		rng := Xoshiro256(42)
		v1 := rng()
		v2 := rng()
		v3 := rng()
		// All values should be different (extremely unlikely to be same)
		if v1 == v2 || v2 == v3 {
			t.Errorf("Values should differ: %v, %v, %v", v1, v2, v3)
		}
	})

	t.Run("produces reasonable distribution", func(t *testing.T) {
		rng := Xoshiro256(99)
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
		// Each bin should have roughly n/10 = 1000 entries
		// Allow 50% deviation
		for i, count := range bins {
			if count < 500 || count > 1500 {
				t.Errorf("Bin %d has %d entries, expected ~1000", i, count)
			}
		}
	})
}
