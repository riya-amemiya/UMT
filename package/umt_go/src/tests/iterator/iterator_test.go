package iterator_test

import (
	"fmt"
	"iter"
	"slices"
	"strings"
	"testing"

	"github.com/riya-amemiya/umt-go/src/iterator"
)

// --- LazyFilter tests ---

func TestLazyFilter(t *testing.T) {
	t.Run("filters values lazily", func(t *testing.T) {
		result := slices.Collect(iterator.LazyFilter(
			slices.Values([]int{1, 2, 3, 4, 5}),
			func(n, _ int) bool { return n%2 == 0 },
		))
		expected := []int{2, 4}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyFilter result = %v, want %v", result, expected)
		}
	})

	t.Run("provides index to the predicate", func(t *testing.T) {
		result := slices.Collect(iterator.LazyFilter(
			slices.Values([]int{10, 20, 30, 40}),
			func(_, i int) bool { return i >= 2 },
		))
		expected := []int{30, 40}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyFilter result = %v, want %v", result, expected)
		}
	})

	t.Run("handles empty iterable", func(t *testing.T) {
		result := slices.Collect(iterator.LazyFilter(
			slices.Values([]int{}),
			func(n, _ int) bool { return n > 0 },
		))
		if len(result) != 0 {
			t.Errorf("LazyFilter result = %v, want empty", result)
		}
	})

	t.Run("handles no matching elements", func(t *testing.T) {
		result := slices.Collect(iterator.LazyFilter(
			slices.Values([]int{1, 2, 3}),
			func(_, _ int) bool { return false },
		))
		if len(result) != 0 {
			t.Errorf("LazyFilter result = %v, want empty", result)
		}
	})

	t.Run("handles all matching elements", func(t *testing.T) {
		result := slices.Collect(iterator.LazyFilter(
			slices.Values([]int{1, 2, 3}),
			func(_, _ int) bool { return true },
		))
		expected := []int{1, 2, 3}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyFilter result = %v, want %v", result, expected)
		}
	})

	t.Run("evaluates lazily", func(t *testing.T) {
		callCount := 0
		seq := iterator.LazyFilter(
			slices.Values([]int{1, 2, 3, 4, 5}),
			func(n, _ int) bool {
				callCount++
				return n > 3
			},
		)

		if callCount != 0 {
			t.Errorf("callCount before pulling = %d, want 0", callCount)
		}

		next, stop := iter.Pull(seq)
		defer stop()
		next()
		if callCount <= 0 {
			t.Errorf("callCount after first pull = %d, want > 0", callCount)
		}
	})

	t.Run("works with a set-like source", func(t *testing.T) {
		result := slices.Collect(iterator.LazyFilter(
			slices.Values([]int{1, 2, 3, 4, 5}),
			func(n, _ int) bool { return n > 3 },
		))
		expected := []int{4, 5}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyFilter result = %v, want %v", result, expected)
		}
	})
}

// --- LazyMap tests ---

func TestLazyMap(t *testing.T) {
	t.Run("maps values lazily", func(t *testing.T) {
		result := slices.Collect(iterator.LazyMap(
			slices.Values([]int{1, 2, 3}),
			func(n, _ int) int { return n * 2 },
		))
		expected := []int{2, 4, 6}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyMap result = %v, want %v", result, expected)
		}
	})

	t.Run("provides index to the mapping function", func(t *testing.T) {
		result := slices.Collect(iterator.LazyMap(
			slices.Values([]string{"a", "b", "c"}),
			func(v string, i int) string { return fmt.Sprintf("%s-%d", v, i) },
		))
		expected := []string{"a-0", "b-1", "c-2"}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyMap result = %v, want %v", result, expected)
		}
	})

	t.Run("handles empty iterable", func(t *testing.T) {
		result := slices.Collect(iterator.LazyMap(
			slices.Values([]int{}),
			func(n, _ int) int { return n },
		))
		if len(result) != 0 {
			t.Errorf("LazyMap result = %v, want empty", result)
		}
	})

	t.Run("evaluates lazily", func(t *testing.T) {
		callCount := 0
		seq := iterator.LazyMap(
			slices.Values([]int{1, 2, 3, 4, 5}),
			func(n, _ int) int {
				callCount++
				return n * 2
			},
		)

		if callCount != 0 {
			t.Errorf("callCount before pulling = %d, want 0", callCount)
		}

		next, stop := iter.Pull(seq)
		defer stop()

		next()
		if callCount != 1 {
			t.Errorf("callCount after first pull = %d, want 1", callCount)
		}

		next()
		if callCount != 2 {
			t.Errorf("callCount after second pull = %d, want 2", callCount)
		}
	})

	t.Run("works with a set-like source", func(t *testing.T) {
		result := slices.Collect(iterator.LazyMap(
			slices.Values([]int{10, 20, 30}),
			func(n, _ int) int { return n + 1 },
		))
		expected := []int{11, 21, 31}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyMap result = %v, want %v", result, expected)
		}
	})

	t.Run("works with a string source", func(t *testing.T) {
		result := slices.Collect(iterator.LazyMap(
			slices.Values([]rune("abc")),
			func(c rune, _ int) string { return strings.ToUpper(string(c)) },
		))
		expected := []string{"A", "B", "C"}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyMap result = %v, want %v", result, expected)
		}
	})

	t.Run("works with a generator-style source", func(t *testing.T) {
		source := func(yield func(int) bool) {
			for _, v := range []int{1, 2, 3} {
				if !yield(v) {
					return
				}
			}
		}
		result := slices.Collect(iterator.LazyMap(
			iter.Seq[int](source),
			func(n, _ int) int { return n * 10 },
		))
		expected := []int{10, 20, 30}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyMap result = %v, want %v", result, expected)
		}
	})
}

// --- LazyTake tests ---

func TestLazyTake(t *testing.T) {
	t.Run("takes the first n elements", func(t *testing.T) {
		result := slices.Collect(iterator.LazyTake(
			slices.Values([]int{1, 2, 3, 4, 5}), 3,
		))
		expected := []int{1, 2, 3}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyTake result = %v, want %v", result, expected)
		}
	})

	t.Run("takes all when n exceeds length", func(t *testing.T) {
		result := slices.Collect(iterator.LazyTake(
			slices.Values([]int{1, 2}), 5,
		))
		expected := []int{1, 2}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyTake result = %v, want %v", result, expected)
		}
	})

	t.Run("takes nothing when n is 0", func(t *testing.T) {
		result := slices.Collect(iterator.LazyTake(
			slices.Values([]int{1, 2, 3}), 0,
		))
		if len(result) != 0 {
			t.Errorf("LazyTake result = %v, want empty", result)
		}
	})

	t.Run("handles empty iterable", func(t *testing.T) {
		result := slices.Collect(iterator.LazyTake(
			slices.Values([]int{}), 5,
		))
		if len(result) != 0 {
			t.Errorf("LazyTake result = %v, want empty", result)
		}
	})

	t.Run("stops iterating after n elements (early return)", func(t *testing.T) {
		yieldCount := 0
		source := func(yield func(int) bool) {
			for i := 0; i < 100; i++ {
				yieldCount++
				if !yield(i) {
					return
				}
			}
		}
		result := slices.Collect(iterator.LazyTake(iter.Seq[int](source), 3))
		expected := []int{0, 1, 2}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyTake result = %v, want %v", result, expected)
		}
		if yieldCount > 4 {
			t.Errorf("yieldCount = %d, want <= 4", yieldCount)
		}
	})

	t.Run("works with a set-like source", func(t *testing.T) {
		result := slices.Collect(iterator.LazyTake(
			slices.Values([]int{10, 20, 30, 40, 50}), 2,
		))
		expected := []int{10, 20}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyTake result = %v, want %v", result, expected)
		}
	})

	t.Run("chains with an infinite source", func(t *testing.T) {
		naturals := func(yield func(int) bool) {
			n := 1
			for {
				if !yield(n) {
					return
				}
				n++
			}
		}
		result := slices.Collect(iterator.LazyTake(iter.Seq[int](naturals), 5))
		expected := []int{1, 2, 3, 4, 5}
		if !slices.Equal(result, expected) {
			t.Errorf("LazyTake result = %v, want %v", result, expected)
		}
	})
}
