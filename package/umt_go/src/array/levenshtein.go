package array

// LevenshteinDistance calculates the Levenshtein distance between two strings.
// Returns the minimum number of single-character edits (insertions, deletions,
// or substitutions) required to change one string into the other.
// Uses O(min(N, M)) space by maintaining a single row of the DP matrix.
func LevenshteinDistance(a, b string) int {
	if a == b {
		return 0
	}

	runesA := []rune(a)
	runesB := []rune(b)

	lenA := len(runesA)
	lenB := len(runesB)

	if lenA == 0 {
		return lenB
	}
	if lenB == 0 {
		return lenA
	}

	// Ensure runesA is the shorter string to minimize space complexity
	if lenA > lenB {
		runesA, runesB = runesB, runesA
		lenA, lenB = lenB, lenA
	}

	// Create a single row array to store distances
	row := make([]int, lenA+1)
	for i := 0; i <= lenA; i++ {
		row[i] = i
	}

	// Iterate through each character of the longer string
	for i := 1; i <= lenB; i++ {
		prevDiag := row[0]
		row[0] = i
		charB := runesB[i-1]

		for j := 1; j <= lenA; j++ {
			temp := row[j]
			cost := 0
			if runesA[j-1] != charB {
				cost = 1
			}

			del := row[j] + 1
			ins := row[j-1] + 1
			sub := prevDiag + cost

			minVal := del
			if ins < minVal {
				minVal = ins
			}
			if sub < minVal {
				minVal = sub
			}
			row[j] = minVal

			prevDiag = temp
		}
	}

	return row[lenA]
}
