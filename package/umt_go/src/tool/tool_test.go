package tool

import (
	"testing"
)

// ---------------------------------------------------------------------------
// Pipe tests
// ---------------------------------------------------------------------------

func TestPipeReturnsInitialValue(t *testing.T) {
	result := Pipe(1)
	if result != 1 {
		t.Errorf("expected 1, got %v", result)
	}
}

func TestPipeSingleFunction(t *testing.T) {
	result := Pipe(1, func(v any) any {
		return v.(int) + 1
	})
	if result != 2 {
		t.Errorf("expected 2, got %v", result)
	}
}

func TestPipeMultipleFunctions(t *testing.T) {
	result := Pipe(1,
		func(v any) any { return v.(int) + 1 },
		func(v any) any { return v.(int) * 2 },
		func(v any) any { return v.(int) - 1 },
	)
	if result != 3 {
		t.Errorf("expected 3, got %v", result)
	}
}

func TestPipeTypeTransformation(t *testing.T) {
	result := Pipe(123,
		func(v any) any {
			n := v.(int)
			return n * 2
		},
		func(v any) any {
			n := v.(int)
			if n > 200 {
				return "big"
			}
			return "small"
		},
	)
	if result != "small" {
		t.Errorf("expected 'small', got %v", result)
	}
}

func TestPipeWithStringTransformations(t *testing.T) {
	result := Pipe("hello",
		func(v any) any { return v.(string) + " world" },
		func(v any) any { return len(v.(string)) },
	)
	if result != 11 {
		t.Errorf("expected 11, got %v", result)
	}
}

func TestPipeWithNilValue(t *testing.T) {
	result := Pipe(nil)
	if result != nil {
		t.Errorf("expected nil, got %v", result)
	}
}

func TestPipeWithZeroValue(t *testing.T) {
	result := Pipe(0,
		func(v any) any { return v.(int) + 1 },
		func(v any) any { return v.(int) * 2 },
	)
	if result != 2 {
		t.Errorf("expected 2, got %v", result)
	}
}

// ---------------------------------------------------------------------------
// ParseJson tests
// ---------------------------------------------------------------------------

func TestParseJsonValidObject(t *testing.T) {
	var result map[string]string
	err := ParseJson(`{"key": "value"}`, &result)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result["key"] != "value" {
		t.Errorf("expected 'value', got %q", result["key"])
	}
}

func TestParseJsonInvalidJSON(t *testing.T) {
	var result map[string]string
	err := ParseJson(`{"key": "value"`, &result)
	if err == nil {
		t.Fatal("expected error for invalid JSON, got nil")
	}
}

func TestParseJsonArray(t *testing.T) {
	var result []string
	err := ParseJson(`["value1", "value2"]`, &result)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(result) != 2 || result[0] != "value1" || result[1] != "value2" {
		t.Errorf("expected [value1, value2], got %v", result)
	}
}

func TestParseJsonNestedObjects(t *testing.T) {
	var result map[string]map[string]string
	err := ParseJson(`{"key": {"nestedKey": "nestedValue"}}`, &result)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result["key"]["nestedKey"] != "nestedValue" {
		t.Errorf("expected 'nestedValue', got %q", result["key"]["nestedKey"])
	}
}

func TestParseJsonNumbers(t *testing.T) {
	var result map[string]float64
	err := ParseJson(`{"key": 123}`, &result)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result["key"] != 123 {
		t.Errorf("expected 123, got %v", result["key"])
	}
}

func TestParseJsonBooleans(t *testing.T) {
	var result map[string]bool
	err := ParseJson(`{"key": true}`, &result)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !result["key"] {
		t.Errorf("expected true, got %v", result["key"])
	}
}

// ---------------------------------------------------------------------------
// Unwrap tests
// ---------------------------------------------------------------------------

func TestUnwrapWithStringValue(t *testing.T) {
	v := "test"
	result := Unwrap(&v, "Value is absent")
	if result != "test" {
		t.Errorf("expected 'test', got %q", result)
	}
}

func TestUnwrapWithIntValue(t *testing.T) {
	v := 42
	result := Unwrap(&v, "Value is absent")
	if result != 42 {
		t.Errorf("expected 42, got %v", result)
	}
}

func TestUnwrapWithZero(t *testing.T) {
	v := 0
	result := Unwrap(&v, "Value is absent")
	if result != 0 {
		t.Errorf("expected 0, got %v", result)
	}
}

func TestUnwrapWithEmptyString(t *testing.T) {
	v := ""
	result := Unwrap(&v, "Value is absent")
	if result != "" {
		t.Errorf("expected empty string, got %q", result)
	}
}

func TestUnwrapWithFalse(t *testing.T) {
	v := false
	result := Unwrap(&v, "Value is absent")
	if result != false {
		t.Errorf("expected false, got %v", result)
	}
}

func TestUnwrapWithStruct(t *testing.T) {
	type kv struct {
		Key   string
		Value string
	}
	v := kv{Key: "key", Value: "value"}
	result := Unwrap(&v, "Value is absent")
	if result.Key != "key" || result.Value != "value" {
		t.Errorf("expected {key value}, got %+v", result)
	}
}

func TestUnwrapWithSlice(t *testing.T) {
	v := []int{1, 2, 3}
	result := Unwrap(&v, "Value is absent")
	if len(result) != 3 || result[0] != 1 || result[1] != 2 || result[2] != 3 {
		t.Errorf("expected [1 2 3], got %v", result)
	}
}

func TestUnwrapNilPanics(t *testing.T) {
	defer func() {
		r := recover()
		if r == nil {
			t.Fatal("expected panic, but did not panic")
		}
		msg, ok := r.(string)
		if !ok || msg != "Value is undefined" {
			t.Errorf("expected panic message 'Value is undefined', got %v", r)
		}
	}()
	var p *int
	Unwrap(p, "Value is undefined")
}

func TestUnwrapNilPanicsWithCustomMessage(t *testing.T) {
	defer func() {
		r := recover()
		if r == nil {
			t.Fatal("expected panic, but did not panic")
		}
		msg, ok := r.(string)
		if !ok || msg != "Value is null" {
			t.Errorf("expected panic message 'Value is null', got %v", r)
		}
	}()
	var p *string
	Unwrap(p, "Value is null")
}
