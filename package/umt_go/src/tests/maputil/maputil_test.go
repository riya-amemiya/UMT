package maputil_test

import (
	"math"
	"reflect"
	"testing"

	"github.com/riya-amemiya/umt-go/src/maputil"
)

// ---------------------------------------------------------------------------
// GroupByToMap tests
// ---------------------------------------------------------------------------

func TestGroupByToMapGroupsNumbersIntoOddAndEven(t *testing.T) {
	array := []int{1, 2, 3, 4, 5}
	result := maputil.GroupByToMap(array, func(num int, _ int) string {
		if num%2 == 0 {
			return "even"
		}
		return "odd"
	})

	if got := result.Keys(); !reflect.DeepEqual(got, []string{"odd", "even"}) {
		t.Errorf("expected keys [odd even], got %v", got)
	}
	if odd, _ := result.Get("odd"); !reflect.DeepEqual(odd, []int{1, 3, 5}) {
		t.Errorf("expected odd [1 3 5], got %v", odd)
	}
	if even, _ := result.Get("even"); !reflect.DeepEqual(even, []int{2, 4}) {
		t.Errorf("expected even [2 4], got %v", even)
	}
}

func TestGroupByToMapPreservesInsertionOrderOfKeys(t *testing.T) {
	array := []int{3, 1, 2, 1, 3, 2}
	result := maputil.GroupByToMap(array, func(num int, _ int) int {
		return num
	})

	if got := result.Keys(); !reflect.DeepEqual(got, []int{3, 1, 2}) {
		t.Errorf("expected keys [3 1 2], got %v", got)
	}
}

func TestGroupByToMapAcceptsObjectKeys(t *testing.T) {
	type tag struct{ id string }
	type item struct {
		tag   tag
		value int
	}
	a := tag{id: "a"}
	b := tag{id: "b"}
	array := []item{
		{tag: a, value: 1},
		{tag: b, value: 2},
		{tag: a, value: 3},
	}
	result := maputil.GroupByToMap(array, func(it item, _ int) tag {
		return it.tag
	})

	wantA := []item{{tag: a, value: 1}, {tag: a, value: 3}}
	if got, _ := result.Get(a); !reflect.DeepEqual(got, wantA) {
		t.Errorf("expected group a %v, got %v", wantA, got)
	}
	wantB := []item{{tag: b, value: 2}}
	if got, _ := result.Get(b); !reflect.DeepEqual(got, wantB) {
		t.Errorf("expected group b %v, got %v", wantB, got)
	}
}

func TestGroupByToMapGroupsStringsByLength(t *testing.T) {
	array := []string{"one", "two", "three", "four", "five"}
	result := maputil.GroupByToMap(array, func(str string, _ int) int {
		return len(str)
	})

	if got := result.Keys(); !reflect.DeepEqual(got, []int{3, 5, 4}) {
		t.Errorf("expected keys [3 5 4], got %v", got)
	}
	if g, _ := result.Get(3); !reflect.DeepEqual(g, []string{"one", "two"}) {
		t.Errorf("expected length-3 group [one two], got %v", g)
	}
	if g, _ := result.Get(5); !reflect.DeepEqual(g, []string{"three"}) {
		t.Errorf("expected length-5 group [three], got %v", g)
	}
	if g, _ := result.Get(4); !reflect.DeepEqual(g, []string{"four", "five"}) {
		t.Errorf("expected length-4 group [four five], got %v", g)
	}
}

func TestGroupByToMapPassesIndexToIteratee(t *testing.T) {
	array := []string{"a", "b", "c", "d"}
	receivedIndices := []int{}
	maputil.GroupByToMap(array, func(_ string, index int) int {
		receivedIndices = append(receivedIndices, index)
		return index % 2
	})

	if !reflect.DeepEqual(receivedIndices, []int{0, 1, 2, 3}) {
		t.Errorf("expected indices [0 1 2 3], got %v", receivedIndices)
	}
}

func TestGroupByToMapWithFloorIteratee(t *testing.T) {
	array := []float64{6.1, 4.2, 6.3}
	result := maputil.GroupByToMap(array, func(v float64, _ int) float64 {
		return math.Floor(v)
	})

	if got := result.Keys(); !reflect.DeepEqual(got, []float64{6, 4}) {
		t.Errorf("expected keys [6 4], got %v", got)
	}
	if g, _ := result.Get(6); !reflect.DeepEqual(g, []float64{6.1, 6.3}) {
		t.Errorf("expected group 6 [6.1 6.3], got %v", g)
	}
	if g, _ := result.Get(4); !reflect.DeepEqual(g, []float64{4.2}) {
		t.Errorf("expected group 4 [4.2], got %v", g)
	}
}

func TestGroupByToMapEmptyArray(t *testing.T) {
	result := maputil.GroupByToMap([]int{}, func(num int, _ int) string {
		if num%2 == 0 {
			return "even"
		}
		return "odd"
	})

	if result.Size() != 0 {
		t.Errorf("expected empty map, got size %d", result.Size())
	}
}

// ---------------------------------------------------------------------------
// ZipToMap tests
// ---------------------------------------------------------------------------

func TestZipToMapPairsKeysAndValues(t *testing.T) {
	result := maputil.ZipToMap([]string{"a", "b", "c"}, []int{1, 2, 3})

	if got := result.Keys(); !reflect.DeepEqual(got, []string{"a", "b", "c"}) {
		t.Errorf("expected keys [a b c], got %v", got)
	}
	for k, want := range map[string]int{"a": 1, "b": 2, "c": 3} {
		if got, _ := result.Get(k); got != want {
			t.Errorf("expected %s => %d, got %d", k, want, got)
		}
	}
}

func TestZipToMapStopsAtShorterWhenKeysLonger(t *testing.T) {
	result := maputil.ZipToMap([]string{"a", "b", "c"}, []int{1, 2})

	if result.Size() != 2 {
		t.Errorf("expected size 2, got %d", result.Size())
	}
	if got := result.Keys(); !reflect.DeepEqual(got, []string{"a", "b"}) {
		t.Errorf("expected keys [a b], got %v", got)
	}
}

func TestZipToMapStopsAtShorterWhenValuesLonger(t *testing.T) {
	result := maputil.ZipToMap([]string{"a", "b"}, []int{1, 2, 3})

	if result.Size() != 2 {
		t.Errorf("expected size 2, got %d", result.Size())
	}
	if got := result.Keys(); !reflect.DeepEqual(got, []string{"a", "b"}) {
		t.Errorf("expected keys [a b], got %v", got)
	}
}

func TestZipToMapKeepsLatestValueForDuplicateKeys(t *testing.T) {
	result := maputil.ZipToMap([]string{"a", "b", "a"}, []int{1, 2, 3})

	if got, _ := result.Get("a"); got != 3 {
		t.Errorf("expected a => 3, got %d", got)
	}
	if got, _ := result.Get("b"); got != 2 {
		t.Errorf("expected b => 2, got %d", got)
	}
	if result.Size() != 2 {
		t.Errorf("expected size 2, got %d", result.Size())
	}
}

func TestZipToMapAcceptsObjectKeys(t *testing.T) {
	type key struct{ id int }
	k1 := key{id: 1}
	k2 := key{id: 2}
	result := maputil.ZipToMap([]key{k1, k2}, []string{"x", "y"})

	if got, _ := result.Get(k1); got != "x" {
		t.Errorf("expected k1 => x, got %s", got)
	}
	if got, _ := result.Get(k2); got != "y" {
		t.Errorf("expected k2 => y, got %s", got)
	}
}

func TestZipToMapEmptyInputs(t *testing.T) {
	if size := maputil.ZipToMap([]string{}, []int{1, 2}).Size(); size != 0 {
		t.Errorf("expected size 0 for empty keys, got %d", size)
	}
	if size := maputil.ZipToMap([]string{"a"}, []int{}).Size(); size != 0 {
		t.Errorf("expected size 0 for empty values, got %d", size)
	}
}

func TestZipToMapPreservesInsertionOrder(t *testing.T) {
	result := maputil.ZipToMap([]string{"c", "a", "b"}, []int{1, 2, 3})

	if got := result.Keys(); !reflect.DeepEqual(got, []string{"c", "a", "b"}) {
		t.Errorf("expected keys [c a b], got %v", got)
	}
}
