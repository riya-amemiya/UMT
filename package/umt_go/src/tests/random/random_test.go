package random_test

import (
	"regexp"
	"testing"

	"github.com/riya-amemiya/umt-go/src/random"
)

// --- RandomBoolean tests ---

func TestRandomBoolean(t *testing.T) {
	t.Run("returns true when probability is 1", func(t *testing.T) {
		for i := 0; i < 50; i++ {
			if !random.RandomBoolean(1) {
				t.Fatalf("RandomBoolean(1) = false, want true")
			}
		}
	})

	t.Run("returns false when probability is 0", func(t *testing.T) {
		for i := 0; i < 50; i++ {
			if random.RandomBoolean(0) {
				t.Fatalf("RandomBoolean(0) = true, want false")
			}
		}
	})

	t.Run("defaults to roughly even split", func(t *testing.T) {
		trueCount := 0
		trials := 4000
		for i := 0; i < trials; i++ {
			if random.RandomBoolean() {
				trueCount++
			}
		}
		ratio := float64(trueCount) / float64(trials)
		if ratio < 0.4 || ratio > 0.6 {
			t.Errorf("default ratio = %f, want within [0.4, 0.6]", ratio)
		}
	})

	t.Run("respects probability bias roughly", func(t *testing.T) {
		trueCount := 0
		trials := 2000
		for i := 0; i < trials; i++ {
			if random.RandomBoolean(0.9) {
				trueCount++
			}
		}
		if ratio := float64(trueCount) / float64(trials); ratio <= 0.7 {
			t.Errorf("biased ratio = %f, want > 0.7", ratio)
		}
	})
}

// --- RandomChoice tests ---

func TestRandomChoice(t *testing.T) {
	t.Run("returns an element from the slice", func(t *testing.T) {
		items := []string{"a", "b", "c"}
		allowed := map[string]bool{"a": true, "b": true, "c": true}
		for i := 0; i < 50; i++ {
			if got := random.RandomChoice(items); !allowed[got] {
				t.Fatalf("RandomChoice returned %q, not in the source slice", got)
			}
		}
	})

	t.Run("returns the only element for single-item slices", func(t *testing.T) {
		if got := random.RandomChoice([]int{42}); got != 42 {
			t.Errorf("RandomChoice([42]) = %d, want 42", got)
		}
	})
}

// --- RandomFloat tests ---

func TestRandomFloat(t *testing.T) {
	t.Run("returns values within the half-open interval", func(t *testing.T) {
		for i := 0; i < 100; i++ {
			value := random.RandomFloat(5, 10)
			if value < 5 || value >= 10 {
				t.Fatalf("RandomFloat(5, 10) = %f, want within [5, 10)", value)
			}
		}
	})

	t.Run("returns the lower bound when min equals max", func(t *testing.T) {
		if got := random.RandomFloat(7, 7); got != 7 {
			t.Errorf("RandomFloat(7, 7) = %f, want 7", got)
		}
	})

	t.Run("supports negative ranges", func(t *testing.T) {
		for i := 0; i < 50; i++ {
			value := random.RandomFloat(-5, -1)
			if value < -5 || value >= -1 {
				t.Fatalf("RandomFloat(-5, -1) = %f, want within [-5, -1)", value)
			}
		}
	})
}

// --- RandomInt tests ---

func TestRandomInt(t *testing.T) {
	t.Run("returns integers within the closed interval", func(t *testing.T) {
		for i := 0; i < 200; i++ {
			value := random.RandomInt(1, 6)
			if value < 1 || value > 6 {
				t.Fatalf("RandomInt(1, 6) = %d, want within [1, 6]", value)
			}
		}
	})

	t.Run("returns the bound when min equals max", func(t *testing.T) {
		if got := random.RandomInt(3, 3); got != 3 {
			t.Errorf("RandomInt(3, 3) = %d, want 3", got)
		}
	})

	t.Run("supports negative ranges", func(t *testing.T) {
		for i := 0; i < 50; i++ {
			value := random.RandomInt(-3, 3)
			if value < -3 || value > 3 {
				t.Fatalf("RandomInt(-3, 3) = %d, want within [-3, 3]", value)
			}
		}
	})

	t.Run("ceils the lower bound and floors the upper bound", func(t *testing.T) {
		for i := 0; i < 50; i++ {
			value := random.RandomInt(0.4, 5.7)
			if value < 1 || value > 5 {
				t.Fatalf("RandomInt(0.4, 5.7) = %d, want within [1, 5]", value)
			}
		}
	})
}

// --- RandomUUID tests ---

var uuidV4Pattern = regexp.MustCompile(
	`^[\da-f]{8}-[\da-f]{4}-4[\da-f]{3}-[89ab][\da-f]{3}-[\da-f]{12}$`,
)

func TestRandomUUID(t *testing.T) {
	t.Run("returns a v4-formatted UUID", func(t *testing.T) {
		if got := random.RandomUUID(); !uuidV4Pattern.MatchString(got) {
			t.Errorf("RandomUUID() = %q, does not match the v4 pattern", got)
		}
	})

	t.Run("returns unique values across calls", func(t *testing.T) {
		set := make(map[string]struct{})
		for i := 0; i < 100; i++ {
			set[random.RandomUUID()] = struct{}{}
		}
		if len(set) != 100 {
			t.Errorf("unique UUID count = %d, want 100", len(set))
		}
	})
}

// --- SeededRandom tests ---

func TestSeededRandom(t *testing.T) {
	t.Run("produces values in [0, 1)", func(t *testing.T) {
		rand := random.SeededRandom(42)
		for i := 0; i < 100; i++ {
			value := rand()
			if value < 0 || value >= 1 {
				t.Fatalf("SeededRandom(42)() = %f, want within [0, 1)", value)
			}
		}
	})

	t.Run("is deterministic for the same numeric seed", func(t *testing.T) {
		a := random.SeededRandom(123)
		b := random.SeededRandom(123)
		for i := 0; i < 20; i++ {
			if av, bv := a(), b(); av != bv {
				t.Fatalf("stream diverged at %d: %f != %f", i, av, bv)
			}
		}
	})

	t.Run("matches the reference stream for seed 42", func(t *testing.T) {
		rand := random.SeededRandom(42)
		expected := []float64{
			0.43152859527617693,
			0.6636104565113783,
			0.006617216160520911,
			0.45699974335730076,
			0.5961809237487614,
		}
		for i, want := range expected {
			if got := rand(); got != want {
				t.Errorf("value %d = %v, want %v", i, got, want)
			}
		}
	})

	t.Run("produces different streams for different seeds", func(t *testing.T) {
		a := random.SeededRandom(1)
		b := random.SeededRandom(2)
		differs := false
		for i := 0; i < 10; i++ {
			if a() != b() {
				differs = true
				break
			}
		}
		if !differs {
			t.Error("streams for seeds 1 and 2 did not differ")
		}
	})

	t.Run("handles a numeric seed of 0", func(t *testing.T) {
		rand := random.SeededRandom(0)
		for i := 0; i < 20; i++ {
			value := rand()
			if value < 0 || value >= 1 {
				t.Fatalf("SeededRandom(0)() = %f, want within [0, 1)", value)
			}
		}
	})
}

// --- SeededRandomString tests ---

func TestSeededRandomString(t *testing.T) {
	t.Run("is deterministic for the same string seed", func(t *testing.T) {
		a := random.SeededRandomString("hello")
		b := random.SeededRandomString("hello")
		for i := 0; i < 20; i++ {
			if av, bv := a(), b(); av != bv {
				t.Fatalf("stream diverged at %d: %f != %f", i, av, bv)
			}
		}
	})

	t.Run("matches the reference stream for seed hello", func(t *testing.T) {
		rand := random.SeededRandomString("hello")
		expected := []float64{
			0.3474058802239597,
			0.2625991594977677,
			0.827850020956248,
			0.8803532146848738,
			0.3523478051647544,
		}
		for i, want := range expected {
			if got := rand(); got != want {
				t.Errorf("value %d = %v, want %v", i, got, want)
			}
		}
	})

	t.Run("handles an empty string seed", func(t *testing.T) {
		rand := random.SeededRandomString("")
		value := rand()
		if value < 0 || value >= 1 {
			t.Errorf("SeededRandomString(\"\")() = %f, want within [0, 1)", value)
		}
	})
}

// --- WeightedChoice tests ---

func TestWeightedChoice(t *testing.T) {
	t.Run("returns one of the weighted values", func(t *testing.T) {
		items := []random.WeightedItem[string]{
			{Value: "a", Weight: 1},
			{Value: "b", Weight: 1},
		}
		allowed := map[string]bool{"a": true, "b": true}
		for i := 0; i < 50; i++ {
			if got := random.WeightedChoice(items); !allowed[got] {
				t.Fatalf("WeightedChoice returned %q, not in the pool", got)
			}
		}
	})

	t.Run("favors heavier weights statistically", func(t *testing.T) {
		items := []random.WeightedItem[string]{
			{Value: "a", Weight: 1},
			{Value: "b", Weight: 9},
		}
		bCount := 0
		trials := 2000
		for i := 0; i < trials; i++ {
			if random.WeightedChoice(items) == "b" {
				bCount++
			}
		}
		if ratio := float64(bCount) / float64(trials); ratio <= 0.8 {
			t.Errorf("b ratio = %f, want > 0.8", ratio)
		}
	})

	t.Run("ignores non-positive weights", func(t *testing.T) {
		items := []random.WeightedItem[string]{
			{Value: "skip", Weight: 0},
			{Value: "winner", Weight: 5},
		}
		for i := 0; i < 50; i++ {
			if got := random.WeightedChoice(items); got != "winner" {
				t.Fatalf("WeightedChoice = %q, want winner", got)
			}
		}
	})

	t.Run("treats negative weights as ignored", func(t *testing.T) {
		items := []random.WeightedItem[string]{
			{Value: "a", Weight: -3},
			{Value: "b", Weight: 5},
		}
		for i := 0; i < 50; i++ {
			if got := random.WeightedChoice(items); got != "b" {
				t.Fatalf("WeightedChoice = %q, want b", got)
			}
		}
	})
}
