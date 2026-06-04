package validate_test

import (
	"math"
	"testing"

	umt_validate "github.com/riya-amemiya/umt-go/src/validate"
)

// --- Double (mirrors Validate/number/double.test.ts) ---

func TestDouble_TrueForDoubleNumbers(t *testing.T) {
	rule := umt_validate.Double("")
	for _, v := range []float64{1.5, 2.22, 0.1} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be a double", v)
		}
	}
}

func TestDouble_FalseForNonDoubleNumbers(t *testing.T) {
	rule := umt_validate.Double("")
	for _, v := range []float64{1, 100, 0} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be a double", v)
		}
	}
}

func TestDouble_NonFiniteRejected(t *testing.T) {
	rule := umt_validate.Double("")
	for _, v := range []float64{math.NaN(), math.Inf(1), math.Inf(-1)} {
		if rule.Validate(v) {
			t.Fatalf("expected non-finite %v not to be a double", v)
		}
	}
}

func TestDouble_CustomMessage(t *testing.T) {
	const customMessage = "This is not a double number"
	rule := umt_validate.Double(customMessage)
	if rule.Validate(1) {
		t.Fatalf("expected 1 to fail the double rule")
	}
	if rule.Message != customMessage {
		t.Fatalf("expected custom message %q, got %q", customMessage, rule.Message)
	}
}

// --- Even (mirrors Validate/number/even.test.ts) ---

func TestEven_TrueForEvenNumbers(t *testing.T) {
	rule := umt_validate.Even("")
	for _, v := range []float64{2, 4, 6, 8, 10, 100, 1000} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be even", v)
		}
	}
}

func TestEven_FalseForOddNumbers(t *testing.T) {
	rule := umt_validate.Even("")
	for _, v := range []float64{1, 3, 5, 7, 9, 99, 999} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be even", v)
		}
	}
}

func TestEven_CustomMessage(t *testing.T) {
	const customMessage = "This is a custom message"
	rule := umt_validate.Even(customMessage)
	if rule.Validate(1) {
		t.Fatalf("expected 1 to fail the even rule")
	}
	if rule.Message != customMessage {
		t.Fatalf("expected custom message %q, got %q", customMessage, rule.Message)
	}
}

func TestEven_Zero(t *testing.T) {
	if !umt_validate.Even("").Validate(0) {
		t.Fatalf("expected 0 to be even")
	}
}

func TestEven_NegativeNumbers(t *testing.T) {
	rule := umt_validate.Even("")
	for _, v := range []float64{-2, -4, -6} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be even", v)
		}
	}
	for _, v := range []float64{-1, -3, -5} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be even", v)
		}
	}
}

func TestEven_DecimalNumbers(t *testing.T) {
	rule := umt_validate.Even("")
	for _, v := range []float64{2.5, 4.7, 1.3, 3.9} {
		if rule.Validate(v) {
			t.Fatalf("expected decimal %v not to be even", v)
		}
	}
}

func TestEven_LargeNumbers(t *testing.T) {
	rule := umt_validate.Even("")
	// Number.MAX_SAFE_INTEGER == 2^53 - 1 (odd); the predecessor is even.
	const maxSafeInteger = float64(1<<53 - 1)
	if !rule.Validate(1_000_000) {
		t.Fatalf("expected 1_000_000 to be even")
	}
	if rule.Validate(999_999) {
		t.Fatalf("expected 999_999 not to be even")
	}
	if !rule.Validate(maxSafeInteger - 1) {
		t.Fatalf("expected MAX_SAFE_INTEGER-1 to be even")
	}
	if rule.Validate(maxSafeInteger) {
		t.Fatalf("expected MAX_SAFE_INTEGER not to be even")
	}
}

func TestEven_NonFiniteRejected(t *testing.T) {
	rule := umt_validate.Even("")
	for _, v := range []float64{math.NaN(), math.Inf(1), math.Inf(-1)} {
		if rule.Validate(v) {
			t.Fatalf("expected non-finite %v not to be even", v)
		}
	}
}

// --- Odd (mirrors Validate/number/odd.test.ts) ---

func TestOdd_TrueForOddNumbers(t *testing.T) {
	rule := umt_validate.Odd("")
	for _, v := range []float64{1, 3, 5, 7, 9, 99, 999} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be odd", v)
		}
	}
}

func TestOdd_FalseForEvenNumbers(t *testing.T) {
	rule := umt_validate.Odd("")
	for _, v := range []float64{2, 4, 6, 8, 10, 100, 1000} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be odd", v)
		}
	}
}

func TestOdd_CustomMessage(t *testing.T) {
	const customMessage = "This is a custom message"
	rule := umt_validate.Odd(customMessage)
	if rule.Validate(2) {
		t.Fatalf("expected 2 to fail the odd rule")
	}
	if rule.Message != customMessage {
		t.Fatalf("expected custom message %q, got %q", customMessage, rule.Message)
	}
}

func TestOdd_Zero(t *testing.T) {
	if umt_validate.Odd("").Validate(0) {
		t.Fatalf("expected 0 not to be odd")
	}
}

func TestOdd_NegativeNumbers(t *testing.T) {
	rule := umt_validate.Odd("")
	for _, v := range []float64{-1, -3, -5} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be odd", v)
		}
	}
	for _, v := range []float64{-2, -4, -6} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be odd", v)
		}
	}
}

func TestOdd_DecimalNumbers(t *testing.T) {
	rule := umt_validate.Odd("")
	for _, v := range []float64{1.5, 3.7, 2.3, 4.9} {
		if rule.Validate(v) {
			t.Fatalf("expected decimal %v not to be odd", v)
		}
	}
}

func TestOdd_LargeNumbers(t *testing.T) {
	rule := umt_validate.Odd("")
	const maxSafeInteger = float64(1<<53 - 1)
	if !rule.Validate(999_999) {
		t.Fatalf("expected 999_999 to be odd")
	}
	if rule.Validate(1_000_000) {
		t.Fatalf("expected 1_000_000 not to be odd")
	}
	if !rule.Validate(maxSafeInteger) {
		t.Fatalf("expected MAX_SAFE_INTEGER to be odd")
	}
	if rule.Validate(maxSafeInteger - 1) {
		t.Fatalf("expected MAX_SAFE_INTEGER-1 not to be odd")
	}
}

func TestOdd_NonFiniteRejected(t *testing.T) {
	rule := umt_validate.Odd("")
	for _, v := range []float64{math.NaN(), math.Inf(1), math.Inf(-1)} {
		if rule.Validate(v) {
			t.Fatalf("expected non-finite %v not to be odd", v)
		}
	}
}

// --- Prime (mirrors Validate/number/prime.test.ts) ---

func TestPrime_TrueForPrimeNumbers(t *testing.T) {
	rule := umt_validate.Prime("")
	for _, v := range []float64{2, 3, 5, 7} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be prime", v)
		}
	}
}

func TestPrime_FalseForNonPrimeNumbers(t *testing.T) {
	rule := umt_validate.Prime("")
	for _, v := range []float64{4, 6, 8, 9} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be prime", v)
		}
	}
}

func TestPrime_CustomMessage(t *testing.T) {
	const customMessage = "This is not a prime number"
	rule := umt_validate.Prime(customMessage)
	if rule.Validate(4) {
		t.Fatalf("expected 4 to fail the prime rule")
	}
	if rule.Message != customMessage {
		t.Fatalf("expected custom message %q, got %q", customMessage, rule.Message)
	}
}

func TestPrime_EdgeCases(t *testing.T) {
	rule := umt_validate.Prime("")
	// <= 1, non-integers, and larger composites/primes (matching isPrimeNumber).
	for _, v := range []float64{1, 0, -3, 2.5, 1.0001} {
		if rule.Validate(v) {
			t.Fatalf("expected %v not to be prime", v)
		}
	}
	for _, v := range []float64{11, 13, 17, 19, 23, 97} {
		if !rule.Validate(v) {
			t.Fatalf("expected %v to be prime", v)
		}
	}
	for _, v := range []float64{25, 49, 91, 100} {
		if rule.Validate(v) {
			t.Fatalf("expected composite %v not to be prime", v)
		}
	}
}

func TestPrime_NonFiniteRejected(t *testing.T) {
	rule := umt_validate.Prime("")
	for _, v := range []float64{math.NaN(), math.Inf(1), math.Inf(-1)} {
		if rule.Validate(v) {
			t.Fatalf("expected non-finite %v not to be prime", v)
		}
	}
}

// --- MinValue (mirrors Validate/number/minValue.test.ts) ---

func TestMinValue_TrueForAtOrAboveMinimum(t *testing.T) {
	rule := umt_validate.MinValue(10, "")
	if !rule.Validate(10) || !rule.Validate(15) {
		t.Fatalf("expected 10 and 15 to satisfy minValue(10)")
	}
}

func TestMinValue_FalseForBelowMinimum(t *testing.T) {
	rule := umt_validate.MinValue(10, "")
	if rule.Validate(9) {
		t.Fatalf("expected 9 not to satisfy minValue(10)")
	}
}

func TestMinValue_CustomMessage(t *testing.T) {
	const customMessage = "Value must be at least 10"
	rule := umt_validate.MinValue(10, customMessage)
	if rule.Validate(9) {
		t.Fatalf("expected 9 to fail minValue(10)")
	}
	if rule.Message != customMessage {
		t.Fatalf("expected custom message %q, got %q", customMessage, rule.Message)
	}
}

// --- MaxValue (mirrors Validate/number/maxValue.test.ts) ---

func TestMaxValue_TrueForAtOrBelowMaximum(t *testing.T) {
	rule := umt_validate.MaxValue(10, "")
	if !rule.Validate(5) || !rule.Validate(10) {
		t.Fatalf("expected 5 and 10 to satisfy maxValue(10)")
	}
}

func TestMaxValue_FalseForAboveMaximum(t *testing.T) {
	rule := umt_validate.MaxValue(10, "")
	if rule.Validate(11) {
		t.Fatalf("expected 11 not to satisfy maxValue(10)")
	}
}

func TestMaxValue_CustomMessage(t *testing.T) {
	const customMessage = "Value must be less than or equal to 10"
	rule := umt_validate.MaxValue(10, customMessage)
	if rule.Validate(11) {
		t.Fatalf("expected 11 to fail maxValue(10)")
	}
	if rule.Message != customMessage {
		t.Fatalf("expected custom message %q, got %q", customMessage, rule.Message)
	}
}

// --- Composition through NumberValidator (rules supplied via option slice) ---

func TestNumberRules_ComposeThroughValidator(t *testing.T) {
	v := umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{
		umt_validate.MinValue(3, "too small"),
		umt_validate.MaxValue(10, "too large"),
		umt_validate.Even("must be even"),
	}, "")

	if !v(4).Validate {
		t.Fatalf("expected 4 to pass min/max/even")
	}
	if got := v(2); got.Validate || got.Message != "too small" {
		t.Fatalf("expected 2 to fail with min message, got %+v", got)
	}
	if got := v(12); got.Validate || got.Message != "too large" {
		t.Fatalf("expected 12 to fail with max message, got %+v", got)
	}
	if got := v(5); got.Validate || got.Message != "must be even" {
		t.Fatalf("expected 5 to fail with even message, got %+v", got)
	}
}
