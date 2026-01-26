package str

import (
	"math/rand"
	"sort"
	"strings"
)

// levenshteinDistance calculates the Levenshtein distance between two strings.
// It returns the minimum number of single-character edits (insertions, deletions,
// or substitutions) required to change one string into the other.
// Uses O(min(N, M)) space complexity.
func levenshteinDistance(s1, s2 string) int {
	r1 := []rune(s1)
	r2 := []rune(s2)

	if string(r1) == string(r2) {
		return 0
	}

	len1 := len(r1)
	len2 := len(r2)

	if len1 == 0 {
		return len2
	}
	if len2 == 0 {
		return len1
	}

	// Ensure r1 is the shorter string to minimize space complexity
	if len1 > len2 {
		return levenshteinDistance(s2, s1)
	}

	// Create a single row array
	row := make([]int, len1+1)
	for i := 0; i <= len1; i++ {
		row[i] = i
	}

	for i := 1; i <= len2; i++ {
		previousDiagonal := row[0]
		row[0] = i
		char2 := r2[i-1]

		for j := 1; j <= len1; j++ {
			temporary := row[j]
			cost := 0
			if r1[j-1] != char2 {
				cost = 1
			}

			deletion := row[j] + 1
			insertion := row[j-1] + 1
			substitution := previousDiagonal + cost

			minVal := deletion
			if insertion < minVal {
				minVal = insertion
			}
			if substitution < minVal {
				minVal = substitution
			}
			row[j] = minVal

			previousDiagonal = temporary
		}
	}

	return row[len1]
}

// StringSimilarity calculates the similarity between two strings as a value between 0 and 1.
// It uses Levenshtein distance normalized by the length of the longer string.
// Returns 1 for identical strings and 0 for completely different strings or when
// one string is empty (and the other is not).
func StringSimilarity(a, b string) float64 {
	if a == b {
		return 1
	}

	if len(a) == 0 || len(b) == 0 {
		return 0
	}

	distance := levenshteinDistance(a, b)
	maxLength := len([]rune(a))
	if len([]rune(b)) > maxLength {
		maxLength = len([]rune(b))
	}

	return 1 - float64(distance)/float64(maxLength)
}

// FuzzySearchResult represents a fuzzy search match with its similarity score.
type FuzzySearchResult struct {
	Item  string
	Score float64
}

// FuzzySearch performs fuzzy string matching on an array of strings.
// It returns items that meet the similarity threshold (default 0.6),
// sorted by score in descending order.
func FuzzySearch(query string, candidates []string) []FuzzySearchResult {
	return FuzzySearchWithThreshold(query, candidates, 0.6)
}

// FuzzySearchWithThreshold performs fuzzy string matching with a custom threshold.
func FuzzySearchWithThreshold(query string, candidates []string, threshold float64) []FuzzySearchResult {
	if len(query) == 0 {
		return []FuzzySearchResult{}
	}

	var results []FuzzySearchResult
	queryLower := strings.ToLower(query)

	for _, item := range candidates {
		itemLower := strings.ToLower(item)
		distance := levenshteinDistance(queryLower, itemLower)
		maxLength := len([]rune(queryLower))
		if len([]rune(itemLower)) > maxLength {
			maxLength = len([]rune(itemLower))
		}

		score := 1 - float64(distance)/float64(maxLength)

		if score >= threshold {
			results = append(results, FuzzySearchResult{
				Item:  item,
				Score: score,
			})
		}
	}

	// Sort by score descending
	sort.Slice(results, func(i, j int) bool {
		return results[i].Score > results[j].Score
	})

	return results
}

// RandomString generates a random alphanumeric string of the specified length.
// It uses the default character set of digits, uppercase, and lowercase letters.
func RandomString(length int) string {
	const chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
	return RandomStringWithChars(length, chars)
}

// RandomStringWithChars generates a random string of the specified length using
// the provided character set.
func RandomStringWithChars(length int, chars string) string {
	runeChars := []rune(chars)
	charLen := len(runeChars)
	result := make([]rune, length)
	for i := 0; i < length; i++ {
		result[i] = runeChars[rand.Intn(charLen)]
	}
	return string(result)
}
