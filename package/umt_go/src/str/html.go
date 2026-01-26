package str

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

// htmlEscapeMap maps HTML special characters to their entity equivalents.
var htmlEscapeMap = map[rune]string{
	'&':  "&amp;",
	'<':  "&lt;",
	'>':  "&gt;",
	'"':  "&quot;",
	'\'': "&#39;",
}

// htmlUnescapeMap maps HTML entities to their character equivalents.
var htmlUnescapeMap = map[string]string{
	"&amp;":  "&",
	"&lt;":   "<",
	"&gt;":   ">",
	"&quot;": "\"",
	"&#39;":  "'",
	"&#x27;": "'",
	"&#x2F;": "/",
	"&#x60;": "`",
	"&#x3D;": "=",
}

// EscapeHtml escapes HTML special characters in a string.
// Characters escaped: & < > " '
func EscapeHtml(s string) string {
	var b strings.Builder
	b.Grow(len(s))
	for _, r := range s {
		if escaped, ok := htmlEscapeMap[r]; ok {
			b.WriteString(escaped)
		} else {
			b.WriteRune(r)
		}
	}
	return b.String()
}

// UnescapeHtml unescapes HTML entities in a string.
// It handles named entities (&amp;, &lt;, &gt;, &quot;, &#39;, &#x27;, &#x2F;, &#x60;, &#x3D;),
// decimal numeric references (&#65;), and hexadecimal numeric references (&#x41;).
func UnescapeHtml(s string) string {
	// Match named entities, decimal numeric refs, and hex numeric refs
	re := regexp.MustCompile(`&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);|&#(\d+);|&#x([0-9a-fA-F]+);`)

	return re.ReplaceAllStringFunc(s, func(match string) string {
		submatches := re.FindStringSubmatch(match)

		// Check decimal numeric reference (group 1)
		if submatches[1] != "" {
			codePoint, err := strconv.ParseInt(submatches[1], 10, 32)
			if err != nil {
				return match
			}
			return fmt.Sprintf("%c", rune(codePoint))
		}

		// Check hexadecimal numeric reference (group 2)
		if submatches[2] != "" {
			codePoint, err := strconv.ParseInt(submatches[2], 16, 32)
			if err != nil {
				return match
			}
			return fmt.Sprintf("%c", rune(codePoint))
		}

		// Named entity lookup
		if unescaped, ok := htmlUnescapeMap[match]; ok {
			return unescaped
		}

		return match
	})
}
