package predicate_test

import (
	"testing"

	"github.com/riya-amemiya/umt-go/src/predicate"
)

// --- Every tests ---

func TestEvery(t *testing.T) {
	isPositiveEven := predicate.Every(
		func(a ...any) bool { return a[0].(int) > 0 },
		func(a ...any) bool { return a[0].(int)%2 == 0 },
	)

	t.Run("returns true when all predicates pass", func(t *testing.T) {
		if !isPositiveEven(4) {
			t.Errorf("isPositiveEven(4) = false, want true")
		}
		if !isPositiveEven(8) {
			t.Errorf("isPositiveEven(8) = false, want true")
		}
	})

	t.Run("returns false when any predicate fails", func(t *testing.T) {
		if isPositiveEven(-2) {
			t.Errorf("isPositiveEven(-2) = true, want false")
		}
		if isPositiveEven(3) {
			t.Errorf("isPositiveEven(3) = true, want false")
		}
	})

	t.Run("short-circuits on first failure", func(t *testing.T) {
		secondCalled := false
		combined := predicate.Every(
			func(_ ...any) bool { return false },
			func(_ ...any) bool {
				secondCalled = true
				return true
			},
		)
		combined()
		if secondCalled {
			t.Errorf("secondCalled = true, want false")
		}
	})

	t.Run("handles single predicate", func(t *testing.T) {
		single := predicate.Every(func(a ...any) bool { return a[0].(int) > 0 })
		if !single(1) {
			t.Errorf("single(1) = false, want true")
		}
		if single(-1) {
			t.Errorf("single(-1) = true, want false")
		}
	})

	t.Run("returns true for no predicates", func(t *testing.T) {
		always := predicate.Every()
		if !always() {
			t.Errorf("always() = false, want true")
		}
	})

	t.Run("handles multiple arguments", func(t *testing.T) {
		bothPositive := predicate.Every(
			func(a ...any) bool { return a[0].(int) > 0 },
			func(a ...any) bool { return a[1].(int) > 0 },
		)
		if !bothPositive(1, 2) {
			t.Errorf("bothPositive(1, 2) = false, want true")
		}
		if bothPositive(1, -1) {
			t.Errorf("bothPositive(1, -1) = true, want false")
		}
		if bothPositive(-1, 1) {
			t.Errorf("bothPositive(-1, 1) = true, want false")
		}
	})
}

// --- Some tests ---

func TestSome(t *testing.T) {
	isZeroOrNegative := predicate.Some(
		func(a ...any) bool { return a[0].(int) == 0 },
		func(a ...any) bool { return a[0].(int) < 0 },
	)

	t.Run("returns true when at least one predicate passes", func(t *testing.T) {
		if !isZeroOrNegative(0) {
			t.Errorf("isZeroOrNegative(0) = false, want true")
		}
		if !isZeroOrNegative(-5) {
			t.Errorf("isZeroOrNegative(-5) = false, want true")
		}
	})

	t.Run("returns false when no predicate passes", func(t *testing.T) {
		if isZeroOrNegative(5) {
			t.Errorf("isZeroOrNegative(5) = true, want false")
		}
	})

	t.Run("short-circuits on first success", func(t *testing.T) {
		secondCalled := false
		combined := predicate.Some(
			func(_ ...any) bool { return true },
			func(_ ...any) bool {
				secondCalled = true
				return false
			},
		)
		combined()
		if secondCalled {
			t.Errorf("secondCalled = true, want false")
		}
	})

	t.Run("handles single predicate", func(t *testing.T) {
		single := predicate.Some(func(a ...any) bool { return a[0].(int) > 0 })
		if !single(1) {
			t.Errorf("single(1) = false, want true")
		}
		if single(-1) {
			t.Errorf("single(-1) = true, want false")
		}
	})

	t.Run("returns false for no predicates", func(t *testing.T) {
		never := predicate.Some()
		if never() {
			t.Errorf("never() = true, want false")
		}
	})

	t.Run("handles multiple arguments", func(t *testing.T) {
		eitherPositive := predicate.Some(
			func(a ...any) bool { return a[0].(int) > 0 },
			func(a ...any) bool { return a[1].(int) > 0 },
		)
		if !eitherPositive(1, -1) {
			t.Errorf("eitherPositive(1, -1) = false, want true")
		}
		if !eitherPositive(-1, 1) {
			t.Errorf("eitherPositive(-1, 1) = false, want true")
		}
		if eitherPositive(-1, -1) {
			t.Errorf("eitherPositive(-1, -1) = true, want false")
		}
	})
}

// --- Not tests ---

func TestNot(t *testing.T) {
	t.Run("negates a truthy predicate", func(t *testing.T) {
		isEven := func(a ...any) bool { return a[0].(int)%2 == 0 }
		isOdd := predicate.Not(isEven)

		if !isOdd(1) {
			t.Errorf("isOdd(1) = false, want true")
		}
		if isOdd(2) {
			t.Errorf("isOdd(2) = true, want false")
		}
		if !isOdd(3) {
			t.Errorf("isOdd(3) = false, want true")
		}
		if isOdd(4) {
			t.Errorf("isOdd(4) = true, want false")
		}
	})

	t.Run("negates a string predicate", func(t *testing.T) {
		isEmpty := func(a ...any) bool { return len(a[0].(string)) == 0 }
		isNotEmpty := predicate.Not(isEmpty)

		if isNotEmpty("") {
			t.Errorf("isNotEmpty(\"\") = true, want false")
		}
		if !isNotEmpty("hello") {
			t.Errorf("isNotEmpty(\"hello\") = false, want true")
		}
	})

	t.Run("handles predicates with multiple arguments", func(t *testing.T) {
		greaterThan := func(a ...any) bool { return a[0].(int) > a[1].(int) }
		notGreaterThan := predicate.Not(greaterThan)

		if notGreaterThan(5, 3) {
			t.Errorf("notGreaterThan(5, 3) = true, want false")
		}
		if !notGreaterThan(3, 5) {
			t.Errorf("notGreaterThan(3, 5) = false, want true")
		}
		if !notGreaterThan(3, 3) {
			t.Errorf("notGreaterThan(3, 3) = false, want true")
		}
	})

	t.Run("passes through arguments correctly", func(t *testing.T) {
		var receivedArgs []any
		spy := func(a ...any) bool {
			receivedArgs = append(receivedArgs, a...)
			return true
		}
		negated := predicate.Not(spy)
		negated("a", "b", "c")
		want := []any{"a", "b", "c"}
		if len(receivedArgs) != len(want) {
			t.Fatalf("receivedArgs = %v, want %v", receivedArgs, want)
		}
		for i := range want {
			if receivedArgs[i] != want[i] {
				t.Errorf("receivedArgs[%d] = %v, want %v", i, receivedArgs[i], want[i])
			}
		}
	})

	t.Run("double negation returns original result", func(t *testing.T) {
		isPositive := func(a ...any) bool { return a[0].(int) > 0 }
		doubleNot := predicate.Not(predicate.Not(isPositive))

		if !doubleNot(5) {
			t.Errorf("doubleNot(5) = false, want true")
		}
		if doubleNot(-1) {
			t.Errorf("doubleNot(-1) = true, want false")
		}
	})
}

// --- IsNotNullish tests ---

func TestIsNotNullish(t *testing.T) {
	t.Run("returns false for nil", func(t *testing.T) {
		if predicate.IsNotNullish(nil) {
			t.Errorf("IsNotNullish(nil) = true, want false")
		}
	})

	t.Run("returns false for a typed nil pointer", func(t *testing.T) {
		var p *int
		if predicate.IsNotNullish(p) {
			t.Errorf("IsNotNullish((*int)(nil)) = true, want false")
		}
	})

	t.Run("returns true for 0", func(t *testing.T) {
		if !predicate.IsNotNullish(0) {
			t.Errorf("IsNotNullish(0) = false, want true")
		}
	})

	t.Run("returns true for empty string", func(t *testing.T) {
		if !predicate.IsNotNullish("") {
			t.Errorf("IsNotNullish(\"\") = false, want true")
		}
	})

	t.Run("returns true for false", func(t *testing.T) {
		if !predicate.IsNotNullish(false) {
			t.Errorf("IsNotNullish(false) = false, want true")
		}
	})

	t.Run("returns true for objects and slices", func(t *testing.T) {
		if !predicate.IsNotNullish(struct{}{}) {
			t.Errorf("IsNotNullish(struct{}{}) = false, want true")
		}
		if !predicate.IsNotNullish([]int{}) {
			t.Errorf("IsNotNullish([]int{}) = false, want true")
		}
	})

	t.Run("returns true for functions", func(t *testing.T) {
		if !predicate.IsNotNullish(func() {}) {
			t.Errorf("IsNotNullish(func(){}) = false, want true")
		}
	})
}

// --- IsNullish tests ---

func TestIsNullish(t *testing.T) {
	t.Run("returns true for nil", func(t *testing.T) {
		if !predicate.IsNullish(nil) {
			t.Errorf("IsNullish(nil) = false, want true")
		}
	})

	t.Run("returns true for a typed nil pointer", func(t *testing.T) {
		var p *int
		if !predicate.IsNullish(p) {
			t.Errorf("IsNullish((*int)(nil)) = false, want true")
		}
	})

	t.Run("returns false for 0", func(t *testing.T) {
		if predicate.IsNullish(0) {
			t.Errorf("IsNullish(0) = true, want false")
		}
	})

	t.Run("returns false for empty string", func(t *testing.T) {
		if predicate.IsNullish("") {
			t.Errorf("IsNullish(\"\") = true, want false")
		}
	})

	t.Run("returns false for false", func(t *testing.T) {
		if predicate.IsNullish(false) {
			t.Errorf("IsNullish(false) = true, want false")
		}
	})

	t.Run("returns false for objects and slices", func(t *testing.T) {
		if predicate.IsNullish(struct{}{}) {
			t.Errorf("IsNullish(struct{}{}) = true, want false")
		}
		if predicate.IsNullish([]int{}) {
			t.Errorf("IsNullish([]int{}) = true, want false")
		}
	})

	t.Run("returns false for functions", func(t *testing.T) {
		if predicate.IsNullish(func() {}) {
			t.Errorf("IsNullish(func(){}) = true, want false")
		}
	})
}

// --- Matches tests ---

func TestMatches(t *testing.T) {
	t.Run("returns true when object matches the pattern", func(t *testing.T) {
		isAdmin := predicate.Matches(map[string]any{"role": "admin"})
		if !isAdmin(map[string]any{"name": "Alice", "role": "admin"}) {
			t.Errorf("isAdmin matched object = false, want true")
		}
	})

	t.Run("returns false when object does not match", func(t *testing.T) {
		isAdmin := predicate.Matches(map[string]any{"role": "admin"})
		if isAdmin(map[string]any{"name": "Bob", "role": "user"}) {
			t.Errorf("isAdmin non-matching object = true, want false")
		}
	})

	t.Run("matches multiple properties", func(t *testing.T) {
		matcher := predicate.Matches(map[string]any{"a": 1, "b": 2})
		if !matcher(map[string]any{"a": 1, "b": 2, "c": 3}) {
			t.Errorf("matcher({a:1,b:2,c:3}) = false, want true")
		}
		if matcher(map[string]any{"a": 1, "b": 3}) {
			t.Errorf("matcher({a:1,b:3}) = true, want false")
		}
		if matcher(map[string]any{"a": 2, "b": 2}) {
			t.Errorf("matcher({a:2,b:2}) = true, want false")
		}
	})

	t.Run("uses strict value equality", func(t *testing.T) {
		matcher := predicate.Matches(map[string]any{"value": 0})
		if !matcher(map[string]any{"value": 0}) {
			t.Errorf("matcher({value:0}) = false, want true")
		}
		if matcher(map[string]any{"value": false}) {
			t.Errorf("matcher({value:false}) = true, want false")
		}
		if matcher(map[string]any{"value": ""}) {
			t.Errorf("matcher({value:\"\"}) = true, want false")
		}
		if matcher(map[string]any{"value": nil}) {
			t.Errorf("matcher({value:nil}) = true, want false")
		}
	})

	t.Run("returns true for empty pattern", func(t *testing.T) {
		alwaysMatch := predicate.Matches(map[string]any{})
		if !alwaysMatch(map[string]any{"anything": "goes"}) {
			t.Errorf("alwaysMatch({anything:goes}) = false, want true")
		}
		if !alwaysMatch(map[string]any{}) {
			t.Errorf("alwaysMatch({}) = false, want true")
		}
	})

	t.Run("handles missing keys in target object", func(t *testing.T) {
		matcher := predicate.Matches(map[string]any{"x": 1})
		if matcher(map[string]any{}) {
			t.Errorf("matcher({}) = true, want false")
		}
		if matcher(map[string]any{"y": 1}) {
			t.Errorf("matcher({y:1}) = true, want false")
		}
	})

	t.Run("matches a present nil value", func(t *testing.T) {
		matchNil := predicate.Matches(map[string]any{"a": nil})
		if !matchNil(map[string]any{"a": nil}) {
			t.Errorf("matchNil({a:nil}) = false, want true")
		}
	})
}
