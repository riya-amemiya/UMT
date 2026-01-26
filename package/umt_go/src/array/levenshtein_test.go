package array

import "testing"

// TestLevenshteinDistance tests the LevenshteinDistance function
func TestLevenshteinDistance(t *testing.T) {
	t.Run("identical strings", func(t *testing.T) {
		if d := LevenshteinDistance("hello", "hello"); d != 0 {
			t.Errorf("Expected 0, got %d", d)
		}
	})

	t.Run("empty strings", func(t *testing.T) {
		if d := LevenshteinDistance("", ""); d != 0 {
			t.Errorf("Expected 0, got %d", d)
		}
	})

	t.Run("one empty string", func(t *testing.T) {
		if d := LevenshteinDistance("", "hello"); d != 5 {
			t.Errorf("Expected 5, got %d", d)
		}
		if d := LevenshteinDistance("hello", ""); d != 5 {
			t.Errorf("Expected 5, got %d", d)
		}
	})

	t.Run("single character substitution", func(t *testing.T) {
		if d := LevenshteinDistance("cat", "bat"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("single character insertion", func(t *testing.T) {
		if d := LevenshteinDistance("cat", "cats"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("single character deletion", func(t *testing.T) {
		if d := LevenshteinDistance("cats", "cat"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("multiple differences - kitten/sitting", func(t *testing.T) {
		if d := LevenshteinDistance("kitten", "sitting"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("multiple differences - saturday/sunday", func(t *testing.T) {
		if d := LevenshteinDistance("saturday", "sunday"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("completely different strings", func(t *testing.T) {
		if d := LevenshteinDistance("abc", "xyz"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
		if d := LevenshteinDistance("hello", "world"); d != 4 {
			t.Errorf("Expected 4, got %d", d)
		}
	})

	t.Run("case sensitive", func(t *testing.T) {
		if d := LevenshteinDistance("Hello", "hello"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
		if d := LevenshteinDistance("ABC", "abc"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("unicode characters", func(t *testing.T) {
		if d := LevenshteinDistance("cafe\u0301", "cafe"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("unicode Japanese", func(t *testing.T) {
		if d := LevenshteinDistance("\u3053\u3093\u306b\u3061\u306f", "\u3053\u3093\u3070\u3093\u306f"); d != 2 {
			t.Errorf("Expected 2, got %d", d)
		}
	})

	t.Run("symmetric", func(t *testing.T) {
		d1 := LevenshteinDistance("hello", "world")
		d2 := LevenshteinDistance("world", "hello")
		if d1 != d2 {
			t.Errorf("Expected symmetric: %d != %d", d1, d2)
		}
	})

	t.Run("single character strings", func(t *testing.T) {
		if d := LevenshteinDistance("a", "b"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
		if d := LevenshteinDistance("a", "a"); d != 0 {
			t.Errorf("Expected 0, got %d", d)
		}
	})

	t.Run("prefix string", func(t *testing.T) {
		if d := LevenshteinDistance("abc", "abcdef"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("suffix string", func(t *testing.T) {
		if d := LevenshteinDistance("def", "abcdef"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("longer strings", func(t *testing.T) {
		s1 := "the quick brown fox"
		s2 := "the quick brown dog"
		d := LevenshteinDistance(s1, s2)
		// "fox" -> "dog": f->d substitution, o->o same, x->g substitution = 2 edits
		if d != 2 {
			t.Errorf("Expected 2, got %d", d)
		}
	})

	t.Run("transposition is two edits", func(t *testing.T) {
		// Levenshtein distance counts transposition as 2 edits (delete + insert)
		if d := LevenshteinDistance("ab", "ba"); d != 2 {
			t.Errorf("Expected 2, got %d", d)
		}
	})
}
