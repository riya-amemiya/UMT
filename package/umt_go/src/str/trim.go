package str

import "strings"

// TrimStartCharacters removes specified characters from the start of a string.
func TrimStartCharacters(s string, chars string) string {
	runes := []rune(s)
	startIndex := 0
	for startIndex < len(runes) && strings.ContainsRune(chars, runes[startIndex]) {
		startIndex++
	}
	return string(runes[startIndex:])
}

// TrimEndCharacters removes specified characters from the end of a string.
func TrimEndCharacters(s string, chars string) string {
	runes := []rune(s)
	endIndex := len(runes) - 1
	for endIndex >= 0 && strings.ContainsRune(chars, runes[endIndex]) {
		endIndex--
	}
	return string(runes[:endIndex+1])
}

// TrimCharacters removes specified characters from both ends of a string.
func TrimCharacters(s string, chars string) string {
	return TrimEndCharacters(TrimStartCharacters(s, chars), chars)
}

// PadStart pads the start of a string with another string until the target length is reached.
func PadStart(s string, length int, pad string) string {
	if pad == "" || len(s) >= length {
		return s
	}

	paddingLength := length - len(s)
	var padding strings.Builder
	for padding.Len() < paddingLength {
		padding.WriteString(pad)
	}

	return padding.String()[:paddingLength] + s
}

// PadEnd pads the end of a string with another string until the target length is reached.
func PadEnd(s string, length int, pad string) string {
	if pad == "" {
		return s
	}

	result := s
	for len(result) < length {
		remaining := length - len(result)
		if remaining >= len(pad) {
			result += pad
		} else {
			result += pad[:remaining]
		}
	}

	return result
}
