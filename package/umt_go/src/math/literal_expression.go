package math

import (
	"regexp"
	"strconv"
	"strings"
)

var (
	litHasLetterRe   = regexp.MustCompile(`[A-Za-z]+`)
	litTermSplitRe   = regexp.MustCompile(`([+-]?\d*[A-Za-z]+)|([+-]?\d+)`)
	litCoeffSplitRe  = regexp.MustCompile(`(\d+)|([A-Za-z]+)`)
	litReduceMatchRe = regexp.MustCompile(`(-?)\d+/1`)
	litTrailingOneRe = regexp.MustCompile(`/1`)
)

// literalExpression solves a simple single-variable linear equation, mirroring
// the TypeScript literalExpression utility. It returns the numeric solution as a
// string, an exact fraction "a/b" when the result is not an integer, or "" for
// trivially true equations like "x=x".
//
// Example:
//
//	literalExpression("x+1=2") // "1"
//	literalExpression("2x=6")  // "3"
//	literalExpression("3x+2=8") // "2"
func literalExpression(x string) string {
	sides := strings.Split(x, "=")
	if len(sides) == 2 && sides[0] == sides[1] {
		return ""
	}

	numericalPart := ""
	var variablePart []string

	for _, part := range sides {
		if litHasLetterRe.MatchString(part) {
			variablePart = jsSplitWithCaptures(litTermSplitRe, part)
		} else {
			numericalPart = part
		}
	}

	if len(variablePart) > 1 && variablePart[1] != "" {
		inverted := variablePart[1]
		inverted = strings.ReplaceAll(inverted, "+", "plus")
		inverted = strings.ReplaceAll(inverted, "-", "minus")
		inverted = strings.ReplaceAll(inverted, "plus", "-")
		inverted = strings.ReplaceAll(inverted, "minus", "+")
		variablePart[1] = calculatorCore(inverted, nil)
	}

	if len(variablePart) > 1 && variablePart[1] != "" {
		sign := "+"
		if strings.HasPrefix(variablePart[1], "-") {
			sign = ""
		}
		numericalPart = calculatorCore(numericalPart+sign+variablePart[1], nil)
	} else {
		numericalPart = calculatorCore(numericalPart, nil)
	}

	variablePart = jsSplitWithCaptures(litCoeffSplitRe, variablePart[0])

	coefficient, err := strconv.ParseFloat(variablePart[0], 64)
	if err != nil {
		return numericalPart
	}

	numerator, _ := strconv.ParseFloat(numericalPart, 64)
	commonGcd := litGCD(coefficient, numerator)
	if commonGcd != 1 {
		numericalPart = jsNumberToString(Division(numerator, float64(commonGcd))) +
			"/" + jsNumberToString(Division(coefficient, float64(commonGcd)))
		if litReduceMatchRe.MatchString(numericalPart) {
			return litTrailingOneRe.ReplaceAllString(numericalPart, "")
		}
	} else if variablePart[0] != "1" {
		numericalPart = numericalPart + "/" + variablePart[0]
	}

	return numericalPart
}

// litGCD reproduces the integer GCD used by literalExpression. It mirrors the
// TypeScript gcd called with already-integer coefficients, rounding the inputs
// to integers before applying the Euclidean algorithm.
func litGCD(a, b float64) int {
	x := int(roundHalfAwayFromZero(a))
	y := int(roundHalfAwayFromZero(b))
	if x < 0 {
		x = -x
	}
	if y < 0 {
		y = -y
	}
	if x == 0 {
		return y
	}
	if y == 0 {
		return x
	}
	for y != 0 {
		x, y = y, x%y
	}
	return x
}

func roundHalfAwayFromZero(n float64) float64 {
	if n < 0 {
		return -roundHalfAwayFromZero(-n)
	}
	return float64(int64(n + 0.5))
}

// jsSplitWithCaptures replicates JavaScript's String.prototype.split when the
// separator is a regular expression containing capture groups, then applies the
// `filter((n) => n && n !== undefined)` step used by literalExpression: the
// returned slice interleaves the unmatched substrings with each match's capture
// groups, with empty and non-participating groups removed.
func jsSplitWithCaptures(re *regexp.Regexp, s string) []string {
	result := []string{}
	last := 0
	matches := re.FindAllStringSubmatchIndex(s, -1)
	for _, m := range matches {
		start, end := m[0], m[1]
		// Skip zero-width matches to mirror JS behavior for these patterns.
		if start == end {
			continue
		}
		if piece := s[last:start]; piece != "" {
			result = append(result, piece)
		}
		for g := 1; g*2 < len(m); g++ {
			gs, ge := m[2*g], m[2*g+1]
			if gs >= 0 && ge > gs {
				result = append(result, s[gs:ge])
			}
		}
		last = end
	}
	if piece := s[last:]; piece != "" {
		result = append(result, piece)
	}
	return result
}
