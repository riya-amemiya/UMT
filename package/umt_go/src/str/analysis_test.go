package str

import (
	"math"
	"regexp"
	"testing"
)

func TestLevenshteinDistance(t *testing.T) {
	tests := []struct {
		name     string
		s1       string
		s2       string
		expected int
	}{
		{"identical strings", "hello", "hello", 0},
		{"both empty", "", "", 0},
		{"first empty", "", "hello", 5},
		{"second empty", "hello", "", 5},
		{"substitution", "cat", "bat", 1},
		{"insertion", "cat", "cats", 1},
		{"deletion", "cats", "cat", 1},
		{"multiple differences kitten/sitting", "kitten", "sitting", 3},
		{"multiple differences saturday/sunday", "saturday", "sunday", 3},
		{"completely different abc/xyz", "abc", "xyz", 3},
		{"completely different hello/world", "hello", "world", 4},
		{"case sensitive Hello/hello", "Hello", "hello", 1},
		{"case sensitive ABC/abc", "ABC", "abc", 3},
		// Unicode characters
		{"unicode cafe", "caf\u00e9", "cafe", 1},
		{"unicode emoji", "\U0001F600", "\U0001F601", 1},
		{"unicode japanese", "\u3053\u3093\u306b\u3061\u306f", "\u3053\u3093\u3070\u3093\u306f", 2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := levenshteinDistance(tt.s1, tt.s2)
			if result != tt.expected {
				t.Errorf("levenshteinDistance(%q, %q) = %d, want %d", tt.s1, tt.s2, result, tt.expected)
			}
		})
	}
}

func TestStringSimilarity(t *testing.T) {
	// Identical strings
	t.Run("identical strings", func(t *testing.T) {
		if got := StringSimilarity("hello", "hello"); got != 1 {
			t.Errorf("StringSimilarity(hello, hello) = %f, want 1", got)
		}
		if got := StringSimilarity("world", "world"); got != 1 {
			t.Errorf("StringSimilarity(world, world) = %f, want 1", got)
		}
	})

	// Empty strings
	t.Run("empty strings", func(t *testing.T) {
		if got := StringSimilarity("", "hello"); got != 0 {
			t.Errorf("StringSimilarity('', hello) = %f, want 0", got)
		}
		if got := StringSimilarity("hello", ""); got != 0 {
			t.Errorf("StringSimilarity(hello, '') = %f, want 0", got)
		}
		if got := StringSimilarity("", ""); got != 1 {
			t.Errorf("StringSimilarity('', '') = %f, want 1", got)
		}
	})

	// Similar strings
	t.Run("similar strings", func(t *testing.T) {
		sim1 := StringSimilarity("cat", "bat")
		if math.Abs(sim1-0.667) > 0.001 {
			t.Errorf("StringSimilarity(cat, bat) = %f, want ~0.667", sim1)
		}

		sim2 := StringSimilarity("kitten", "sitting")
		if math.Abs(sim2-0.571) > 0.001 {
			t.Errorf("StringSimilarity(kitten, sitting) = %f, want ~0.571", sim2)
		}
	})

	// Completely different strings
	t.Run("completely different strings", func(t *testing.T) {
		sim := StringSimilarity("abc", "xyz")
		if sim != 0 {
			t.Errorf("StringSimilarity(abc, xyz) = %f, want 0", sim)
		}
	})

	// Different lengths
	t.Run("different lengths", func(t *testing.T) {
		sim1 := StringSimilarity("cat", "cats")
		if sim1 != 0.75 {
			t.Errorf("StringSimilarity(cat, cats) = %f, want 0.75", sim1)
		}

		sim2 := StringSimilarity("hello", "helo")
		if sim2 != 0.8 {
			t.Errorf("StringSimilarity(hello, helo) = %f, want 0.8", sim2)
		}
	})

	// Case sensitive
	t.Run("case sensitive", func(t *testing.T) {
		sim := StringSimilarity("Hello", "hello")
		if sim != 0.8 {
			t.Errorf("StringSimilarity(Hello, hello) = %f, want 0.8", sim)
		}
	})

	// Unicode
	t.Run("unicode", func(t *testing.T) {
		sim1 := StringSimilarity("caf\u00e9", "cafe")
		if sim1 != 0.75 {
			t.Errorf("StringSimilarity(cafe, cafe) = %f, want 0.75", sim1)
		}
	})

	// Values between 0 and 1
	t.Run("values between 0 and 1", func(t *testing.T) {
		pairs := [][2]string{
			{"hello", "world"},
			{"test", "testing"},
			{"a", "b"},
			{"similar", "similarity"},
		}
		for _, pair := range pairs {
			sim := StringSimilarity(pair[0], pair[1])
			if sim < 0 || sim > 1 {
				t.Errorf("StringSimilarity(%q, %q) = %f, expected between 0 and 1", pair[0], pair[1], sim)
			}
		}
	})
}

func TestFuzzySearch(t *testing.T) {
	// JSDoc example
	t.Run("basic fuzzy search", func(t *testing.T) {
		result := FuzzySearch("hello", []string{"hello", "world", "helo", "help"})
		if len(result) != 3 {
			t.Fatalf("Expected 3 results, got %d", len(result))
		}

		// Check that results contain the expected items
		found := map[string]bool{}
		for _, r := range result {
			found[r.Item] = true
		}
		if !found["hello"] {
			t.Error("Expected 'hello' in results")
		}
		if !found["helo"] {
			t.Error("Expected 'helo' in results")
		}
		if !found["help"] {
			t.Error("Expected 'help' in results")
		}

		// Check scores
		for _, r := range result {
			if r.Item == "hello" && r.Score != 1 {
				t.Errorf("Expected score 1 for 'hello', got %f", r.Score)
			}
			if r.Item == "helo" && r.Score != 0.8 {
				t.Errorf("Expected score 0.8 for 'helo', got %f", r.Score)
			}
			if r.Item == "help" && r.Score != 0.6 {
				t.Errorf("Expected score 0.6 for 'help', got %f", r.Score)
			}
		}
	})

	// Exact matches
	t.Run("exact matches", func(t *testing.T) {
		result := FuzzySearch("test", []string{"test", "best", "rest"})
		if len(result) == 0 {
			t.Fatal("Expected at least one result")
		}
		if result[0].Item != "test" || result[0].Score != 1 {
			t.Errorf("Expected first result to be test with score 1, got %s with %f", result[0].Item, result[0].Score)
		}
	})

	// Sorted by score descending
	t.Run("sorted by score descending", func(t *testing.T) {
		result := FuzzySearch("test", []string{"test", "tests", "testing"})
		for i := 0; i < len(result)-1; i++ {
			if result[i].Score < result[i+1].Score {
				t.Errorf("Results not sorted: index %d score %f < index %d score %f", i, result[i].Score, i+1, result[i+1].Score)
			}
		}
	})

	// Custom threshold
	t.Run("custom threshold", func(t *testing.T) {
		highThreshold := FuzzySearchWithThreshold("hello", []string{"hello", "helo", "help"}, 0.9)
		lowThreshold := FuzzySearchWithThreshold("hello", []string{"hello", "helo", "help"}, 0.3)
		if len(highThreshold) > len(lowThreshold) {
			t.Errorf("High threshold results (%d) should be <= low threshold results (%d)", len(highThreshold), len(lowThreshold))
		}
	})

	// Case insensitive
	t.Run("case insensitive", func(t *testing.T) {
		result := FuzzySearch("Hello", []string{"HELLO", "hello", "Hello"})
		if len(result) != 3 {
			t.Fatalf("Expected 3 results, got %d", len(result))
		}
		for _, r := range result {
			if r.Score != 1 {
				t.Errorf("Expected score 1 for %q, got %f", r.Item, r.Score)
			}
		}
	})

	// Empty query
	t.Run("empty query", func(t *testing.T) {
		result := FuzzySearch("", []string{"hello", "world"})
		if len(result) != 0 {
			t.Errorf("Expected empty results for empty query, got %d", len(result))
		}
	})

	// Empty items
	t.Run("empty items", func(t *testing.T) {
		result := FuzzySearch("hello", []string{})
		if len(result) != 0 {
			t.Errorf("Expected empty results for empty items, got %d", len(result))
		}
	})

	// Filter out items below threshold
	t.Run("filter below threshold", func(t *testing.T) {
		result := FuzzySearchWithThreshold("hello", []string{"world", "xyz"}, 0.8)
		if len(result) != 0 {
			t.Errorf("Expected empty results, got %d", len(result))
		}
	})

	// Single character matches
	t.Run("single character matches", func(t *testing.T) {
		result := FuzzySearch("a", []string{"a", "b", "ab"})
		found := false
		for _, r := range result {
			if r.Item == "a" && r.Score == 1 {
				found = true
			}
		}
		if !found {
			t.Error("Expected 'a' with score 1 in results")
		}
	})

	// Special characters
	t.Run("special characters", func(t *testing.T) {
		result := FuzzySearch("hello!", []string{"hello!", "hello"})
		if len(result) == 0 {
			t.Fatal("Expected at least one result")
		}
		if result[0].Item != "hello!" || result[0].Score != 1 {
			t.Errorf("Expected first result to be 'hello!' with score 1, got %s with %f", result[0].Item, result[0].Score)
		}
	})
}

func TestRandomString(t *testing.T) {
	// Default length and characters
	t.Run("default length and characters", func(t *testing.T) {
		s := RandomString(8)
		if len(s) != 8 {
			t.Errorf("Expected length 8, got %d", len(s))
		}
		matched, _ := regexp.MatchString(`^[0-9A-Za-z]{8}$`, s)
		if !matched {
			t.Errorf("Expected alphanumeric string, got %q", s)
		}
	})

	// Custom character set
	t.Run("custom character set", func(t *testing.T) {
		s := RandomStringWithChars(10, "abc123")
		if len(s) != 10 {
			t.Errorf("Expected length 10, got %d", len(s))
		}
		matched, _ := regexp.MatchString(`^[abc123]{10}$`, s)
		if !matched {
			t.Errorf("Expected string from charset abc123, got %q", s)
		}
	})

	// Specified length
	t.Run("specified length", func(t *testing.T) {
		s := RandomString(20)
		if len(s) != 20 {
			t.Errorf("Expected length 20, got %d", len(s))
		}
	})
}
