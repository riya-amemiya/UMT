package validate_test

import (
	"testing"

	umt_validate "github.com/riya-amemiya/umt-go/src/validate"
)

// These tests mirror the TypeScript collection-validator suites
// (Validate/array/arrayOf.test.ts, Validate/map/core.test.ts,
// Validate/set/core.test.ts). Pieces that are purely TS type-level assertions
// (the `@ts-expect-error` cases and SchemaToInterface inference checks) have no
// runtime behavior to port and so are exercised through the equivalent runtime
// failure paths instead.

// --- ArrayOf (mirrors Validate/array/arrayOf.test.ts) ---

func TestArrayOf_EmptyArrayIsValid(t *testing.T) {
	v := umt_validate.ArrayOf(umt_validate.StringValidator(nil, ""), "")
	if !v([]string{}).Validate {
		t.Fatalf("expected empty array valid")
	}
}

func TestArrayOf_ValidatesEveryElement(t *testing.T) {
	v := umt_validate.ArrayOf(umt_validate.StringValidator(nil, ""), "")
	res := v([]string{"a", "b", "c"})
	if !res.Validate {
		t.Fatalf("expected all-string array valid")
	}
	if res.Message != "" {
		t.Fatalf("expected empty message on success, got %q", res.Message)
	}
}

func TestArrayOf_NumberRuleChains(t *testing.T) {
	minValue := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v >= 0 },
		Message:  "must be non-negative",
	}
	v := umt_validate.ArrayOf(
		umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{minValue}, ""),
		"",
	)
	if !v([]float64{0, 1, 2}).Validate {
		t.Fatalf("expected non-negative array valid")
	}
	if v([]float64{0, -1}).Validate {
		t.Fatalf("expected array with negative rejected")
	}
}

func TestArrayOf_PropagatesElementMessage(t *testing.T) {
	message := "value must be non-negative"
	rule := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v >= 0 },
		Message:  message,
	}
	v := umt_validate.ArrayOf(
		umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{rule}, ""),
		"",
	)
	res := v([]float64{1, -1})
	if res.Validate {
		t.Fatalf("expected invalid")
	}
	if res.Message != message {
		t.Fatalf("expected propagated element message, got %q", res.Message)
	}
}

func TestArrayOf_PreservesOriginalArrayInType(t *testing.T) {
	data := []string{"John"}
	v := umt_validate.ArrayOf(umt_validate.StringValidator(nil, ""), "")
	got := v(data)
	if len(got.Type) != len(data) || got.Type[0] != data[0] {
		t.Fatalf("expected original array carried in Type, got %v", got.Type)
	}
}

func TestArrayOf_StopsAtFirstFailingElement(t *testing.T) {
	var calls []float64
	tracking := func(value float64) umt_validate.ValidateCoreReturnType[float64] {
		calls = append(calls, value)
		valid := value >= 0
		message := ""
		if !valid {
			message = "negative"
		}
		return umt_validate.ValidateCoreReturnType[float64]{
			Validate: valid,
			Message:  message,
			Type:     value,
		}
	}
	v := umt_validate.ArrayOf(tracking, "")
	res := v([]float64{1, -1, -2})
	if res.Validate {
		t.Fatalf("expected invalid")
	}
	if res.Message != "negative" {
		t.Fatalf("expected first failing message, got %q", res.Message)
	}
	if len(calls) != 2 || calls[0] != 1 || calls[1] != -1 {
		t.Fatalf("expected short-circuit after second element, got %v", calls)
	}
}

func TestArrayOf_NestedMatrix(t *testing.T) {
	inner := umt_validate.NumberValidator(nil, "")
	rowValidator := func(row []float64) umt_validate.ValidateCoreReturnType[[]float64] {
		return umt_validate.ValidateArray(row, inner, "")
	}
	v := umt_validate.ArrayOf(rowValidator, "")
	ok := v([][]float64{{1, 2}, {3, 4}})
	if !ok.Validate {
		t.Fatalf("expected nested matrix valid")
	}

	failingInner := umt_validate.ValidateReturnType[float64]{
		Validate: func(value float64) bool { return value < 10 },
		Message:  "too big",
	}
	rowValidatorFail := func(row []float64) umt_validate.ValidateCoreReturnType[[]float64] {
		return umt_validate.ValidateArray(row, umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{failingInner}, ""), "")
	}
	vFail := umt_validate.ArrayOf(rowValidatorFail, "")
	if vFail([][]float64{{1, 99}}).Validate {
		t.Fatalf("expected nested matrix with out-of-range inner rejected")
	}
}

func TestArrayValidator_AliasMatchesArrayOf(t *testing.T) {
	v := umt_validate.ArrayValidator(umt_validate.StringValidator(nil, ""), "")
	if !v([]string{"a", "b"}).Validate {
		t.Fatalf("expected ArrayValidator alias to validate like ArrayOf")
	}
}

func TestValidateArray_Direct(t *testing.T) {
	res := umt_validate.ValidateArray([]string{"a", "b"}, umt_validate.StringValidator(nil, ""), "")
	if !res.Validate || res.Message != "" {
		t.Fatalf("expected direct array validation success, got %+v", res)
	}
}

// --- Map (mirrors Validate/map/core.test.ts) ---

func TestMap_AcceptsWithoutEntryValidators(t *testing.T) {
	v := umt_validate.MapValidator[string, int](nil, nil, "")
	if !v(map[string]int{}).Validate {
		t.Fatalf("expected empty map valid")
	}
	if !v(map[string]int{"a": 1}).Validate {
		t.Fatalf("expected map without validators valid")
	}
}

func TestMap_ValidatesKeysWithKeyValidator(t *testing.T) {
	nonEmptyKey := umt_validate.ValidateReturnType[string]{
		Validate: func(v string) bool { return v != "" },
		Message:  "key must be non-empty",
	}
	v := umt_validate.MapValidator(
		umt_validate.StringValidator([]umt_validate.ValidateReturnType[string]{nonEmptyKey}, ""),
		umt_validate.NumberValidator(nil, ""),
		"",
	)
	if !v(map[string]float64{"a": 1, "b": 2}).Validate {
		t.Fatalf("expected valid keys accepted")
	}
	if v(map[string]float64{"": 1}).Validate {
		t.Fatalf("expected empty key rejected")
	}
}

func TestMap_ValidatesValuesWithValueValidator(t *testing.T) {
	positive := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v > 0 },
		Message:  "value must be positive",
	}
	v := umt_validate.MapValidator(
		umt_validate.StringValidator(nil, ""),
		umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{positive}, ""),
		"",
	)
	if v(map[string]float64{"a": -1}).Validate {
		t.Fatalf("expected non-positive value rejected")
	}
}

func TestMap_PropagatesFailingEntryMessage(t *testing.T) {
	valueRule := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v > 0 },
		Message:  "value must be number",
	}
	v := umt_validate.MapValidator(
		umt_validate.StringValidator(nil, ""),
		umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{valueRule}, ""),
		"",
	)
	res := v(map[string]float64{"a": -1})
	if res.Message != "value must be number" {
		t.Fatalf("expected propagated value message, got %q", res.Message)
	}
}

func TestMap_NilKeyValidatorSkipsKeySide(t *testing.T) {
	positive := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v > 0 },
		Message:  "must be positive",
	}
	v := umt_validate.MapValidator[string, float64](
		nil,
		umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{positive}, ""),
		"",
	)
	if !v(map[string]float64{"": 1}).Validate {
		t.Fatalf("expected nil key validator to skip empty-key check")
	}
}

func TestMap_SuccessClearsMessage(t *testing.T) {
	res := umt_validate.ValidateMap(
		map[string]float64{"a": 1},
		umt_validate.StringValidator(nil, ""),
		umt_validate.NumberValidator(nil, ""),
		"must be a Map",
	)
	if !res.Validate {
		t.Fatalf("expected valid map")
	}
	if res.Message != "" {
		t.Fatalf("expected message cleared on success, got %q", res.Message)
	}
}

func TestMap_PreservesOriginalMapInType(t *testing.T) {
	data := map[string]float64{"a": 1}
	res := umt_validate.ValidateMap(data, umt_validate.StringValidator(nil, ""), umt_validate.NumberValidator(nil, ""), "")
	if len(res.Type) != 1 || res.Type["a"] != 1 {
		t.Fatalf("expected original map carried in Type, got %v", res.Type)
	}
}

// --- Set (mirrors Validate/set/core.test.ts) ---

func TestSet_AcceptsWithoutItemValidator(t *testing.T) {
	v := umt_validate.SetValidator[int](nil, "")
	if !v(map[int]struct{}{}).Validate {
		t.Fatalf("expected empty set valid")
	}
	if !v(map[int]struct{}{1: {}, 2: {}}).Validate {
		t.Fatalf("expected set without validator valid")
	}
}

func TestSet_ValidatesItemsWithItemValidator(t *testing.T) {
	nonEmpty := umt_validate.ValidateReturnType[string]{
		Validate: func(v string) bool { return v != "" },
		Message:  "must be non-empty",
	}
	v := umt_validate.SetValidator(
		umt_validate.StringValidator([]umt_validate.ValidateReturnType[string]{nonEmpty}, ""),
		"",
	)
	if !v(map[string]struct{}{"a": {}, "b": {}}).Validate {
		t.Fatalf("expected non-empty items accepted")
	}
	if v(map[string]struct{}{"": {}}).Validate {
		t.Fatalf("expected empty item rejected")
	}
}

func TestSet_PropagatesFailingItemMessage(t *testing.T) {
	rule := umt_validate.ValidateReturnType[string]{
		Validate: func(v string) bool { return v != "" },
		Message:  "must be string",
	}
	v := umt_validate.SetValidator(
		umt_validate.StringValidator([]umt_validate.ValidateReturnType[string]{rule}, ""),
		"",
	)
	res := v(map[string]struct{}{"": {}})
	if res.Message != "must be string" {
		t.Fatalf("expected propagated item message, got %q", res.Message)
	}
}

func TestSet_NilValidatorAcceptsAny(t *testing.T) {
	res := umt_validate.ValidateSet(map[string]struct{}{"a": {}, "b": {}}, nil, "")
	if !res.Validate {
		t.Fatalf("expected nil validator to accept any set")
	}
}

func TestSet_SuccessClearsMessage(t *testing.T) {
	res := umt_validate.ValidateSet(
		map[string]struct{}{"a": {}},
		umt_validate.StringValidator(nil, ""),
		"must be a Set",
	)
	if !res.Validate {
		t.Fatalf("expected valid set")
	}
	if res.Message != "" {
		t.Fatalf("expected message cleared on success, got %q", res.Message)
	}
}

func TestSet_PreservesOriginalSetInType(t *testing.T) {
	data := map[string]struct{}{"a": {}}
	res := umt_validate.ValidateSet(data, umt_validate.StringValidator(nil, ""), "")
	if len(res.Type) != 1 {
		t.Fatalf("expected original set carried in Type, got %v", res.Type)
	}
	if _, ok := res.Type["a"]; !ok {
		t.Fatalf("expected element preserved in carried set")
	}
}
