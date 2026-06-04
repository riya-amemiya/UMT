package number

import "strconv"

// ToOrdinal converts an integer to its English ordinal string representation.
//
// It handles the special cases for 11, 12, and 13 (and any number whose last
// two digits fall in 11..13), which end in "th" despite their final digit. The
// remainder operations follow Go's sign-preserving "%" (matching JavaScript),
// so a negative value such as -21 keeps its sign and falls through to "th".
//
// Example:
//
//	ToOrdinal(1)   // "1st"
//	ToOrdinal(2)   // "2nd"
//	ToOrdinal(3)   // "3rd"
//	ToOrdinal(11)  // "11th"
//	ToOrdinal(21)  // "21st"
//	ToOrdinal(112) // "112th"
func ToOrdinal(value int) string {
	s := strconv.Itoa(value)

	module100 := value % 100
	if module100 >= 11 && module100 <= 13 {
		return s + "th"
	}

	switch value % 10 {
	case 1:
		return s + "st"
	case 2:
		return s + "nd"
	case 3:
		return s + "rd"
	default:
		return s + "th"
	}
}
