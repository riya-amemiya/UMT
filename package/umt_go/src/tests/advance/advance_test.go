package advance_test

import (
	"reflect"
	"testing"

	"github.com/riya-amemiya/umt-go/src/advance"
)

func TestRangeAdvanceBasic(t *testing.T) {
	result := advance.RangeAdvance(0, 5, nil)
	expected := []int{0, 1, 2, 3, 4}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("RangeAdvance(0, 5, nil) = %v, want %v", result, expected)
	}
}

func TestRangeAdvanceOffset(t *testing.T) {
	result := advance.RangeAdvance(2, 5, nil)
	expected := []int{2, 3, 4}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("RangeAdvance(2, 5, nil) = %v, want %v", result, expected)
	}
}

func TestRangeAdvanceWithCondition(t *testing.T) {
	isEven := func(n int) bool { return n%2 == 0 }
	result := advance.RangeAdvance(0, 10, isEven)
	expected := []int{0, 2, 4, 6, 8}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("RangeAdvance(0, 10, isEven) = %v, want %v", result, expected)
	}
}

func TestRangeAdvanceInvalidRange(t *testing.T) {
	result := advance.RangeAdvance(5, 2, nil)
	if len(result) != 0 {
		t.Errorf("RangeAdvance(5, 2, nil) = %v, want empty slice", result)
	}
}

func TestRangeAdvanceSameStartEnd(t *testing.T) {
	result := advance.RangeAdvance(3, 3, nil)
	if len(result) != 0 {
		t.Errorf("RangeAdvance(3, 3, nil) = %v, want empty slice", result)
	}
}

func TestRangeAdvanceOddCondition(t *testing.T) {
	isOdd := func(n int) bool { return n%2 != 0 }
	result := advance.RangeAdvance(0, 10, isOdd)
	expected := []int{1, 3, 5, 7, 9}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("RangeAdvance(0, 10, isOdd) = %v, want %v", result, expected)
	}
}

func TestRangeAdvanceNegativeRange(t *testing.T) {
	result := advance.RangeAdvance(-3, 3, nil)
	expected := []int{-3, -2, -1, 0, 1, 2}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("RangeAdvance(-3, 3, nil) = %v, want %v", result, expected)
	}
}
