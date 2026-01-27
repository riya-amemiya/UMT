package str

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

// Formatter is a function that takes a value and optional arguments and returns
// a formatted string.
type Formatter func(value any, args ...string) string

// FormatOptions holds configuration for FormatString.
type FormatOptions struct {
	Formatters map[string]Formatter
}

// defaultFormatters provides the built-in formatter functions.
var defaultFormatters = map[string]Formatter{
	"upper": func(value any, args ...string) string {
		return strings.ToUpper(fmt.Sprintf("%v", value))
	},
	"lower": func(value any, args ...string) string {
		return strings.ToLower(fmt.Sprintf("%v", value))
	},
	"plural": func(value any, args ...string) string {
		singular := ""
		plural := ""
		if len(args) > 0 {
			singular = args[0]
		}
		if len(args) > 1 {
			plural = args[1]
		}
		n, _ := toFloat64(value)
		if n == 1 {
			return singular
		}
		return plural
	},
	"pad": func(value any, args ...string) string {
		length := 2
		char := "0"
		if len(args) > 0 {
			if l, err := strconv.Atoi(args[0]); err == nil {
				length = l
			}
		}
		if len(args) > 1 {
			char = args[1]
		}
		s := fmt.Sprintf("%v", value)
		for len(s) < length {
			s = char + s
		}
		return s
	},
}

// toFloat64 attempts to convert an arbitrary value to float64.
func toFloat64(value any) (float64, bool) {
	switch v := value.(type) {
	case int:
		return float64(v), true
	case int8:
		return float64(v), true
	case int16:
		return float64(v), true
	case int32:
		return float64(v), true
	case int64:
		return float64(v), true
	case uint:
		return float64(v), true
	case uint8:
		return float64(v), true
	case uint16:
		return float64(v), true
	case uint32:
		return float64(v), true
	case uint64:
		return float64(v), true
	case float32:
		return float64(v), true
	case float64:
		return v, true
	case string:
		f, err := strconv.ParseFloat(v, 64)
		if err == nil {
			return f, true
		}
		return 0, false
	default:
		return 0, false
	}
}

// FormatString replaces placeholders in a template string with specified values.
//
// It supports two modes:
//
//  1. Indexed mode: Pass variadic values and use numbered placeholders like {0}, {1}.
//  2. Named mode: Pass a single map[string]any as data and use named placeholders
//     like {name}, {age}. Optionally pass FormatOptions as the second argument.
//
// Features:
//   - Nested object access with dot notation: {user.name}, {user.address.city}
//   - Array access with brackets: {items[0]}, {items[-1]}
//   - Default values with pipe: {name|Unknown}
//   - Formatters: {name:upper}, {id:pad(4,0)}
//   - Escape sequences: {{name}} renders as literal {name}
//
// Built-in formatters: upper, lower, plural, pad.
func FormatString(template string, args ...any) string {
	// Escape double braces
	escaped := strings.ReplaceAll(template, "{{", "\x00")
	escaped = strings.ReplaceAll(escaped, "}}", "\x01")

	data, options := detectMode(args)
	formatters := mergeFormatters(defaultFormatters, options.Formatters)

	re := regexp.MustCompile(`\{([^}]+)\}`)
	result := re.ReplaceAllStringFunc(escaped, func(match string) string {
		// Extract content between braces
		content := match[1 : len(match)-1]

		// Split default value
		pathAndFormatter, defaultValue := splitFirst(content, "|")
		pathAndFormatter = strings.TrimSpace(pathAndFormatter)
		hasDefault := defaultValue != ""
		if hasDefault {
			defaultValue = strings.TrimSpace(defaultValue)
		} else {
			// Check if there was actually a pipe
			if strings.Contains(content, "|") {
				hasDefault = true
				defaultValue = strings.TrimSpace(strings.SplitN(content, "|", 2)[1])
			}
		}

		// Split formatter
		path, formatterString := splitFirst(pathAndFormatter, ":")

		var value any
		var found bool

		switch d := data.(type) {
		case []any:
			index, err := strconv.Atoi(path)
			if err != nil {
				value = nil
				found = false
			} else if index >= 0 && index < len(d) {
				value = d[index]
				found = true
			}
		case map[string]any:
			value = getValue(d, path)
			found = value != nil
		default:
			value = nil
			found = false
		}

		if !found || value == nil {
			if hasDefault {
				value = defaultValue
			} else {
				return match
			}
		}

		if formatterString != "" {
			return applyFormatter(value, formatterString, formatters)
		}

		return fmt.Sprintf("%v", value)
	})

	// Restore escaped braces
	result = strings.ReplaceAll(result, "\x00", "{")
	result = strings.ReplaceAll(result, "\x01", "}")
	return result
}

// detectMode determines whether to use indexed or named mode.
func detectMode(args []any) (data any, options FormatOptions) {
	if len(args) == 0 {
		return []any{}, FormatOptions{}
	}

	first := args[0]

	// Check if the first argument is a map (named mode)
	if m, ok := first.(map[string]any); ok {
		// Named mode
		if len(args) >= 2 {
			if opts, ok := args[1].(FormatOptions); ok {
				return m, opts
			}
			// Check for *FormatOptions
			if opts, ok := args[1].(*FormatOptions); ok && opts != nil {
				return m, *opts
			}
		}
		if len(args) == 1 {
			return m, FormatOptions{}
		}
	}

	// Indexed mode - collect all args into a slice
	return args, FormatOptions{}
}

// getValue retrieves a value from a map using a dot-notation path with optional
// array index support (e.g., "items[0]", "users[-1].name").
func getValue(obj any, path string) any {
	parts := strings.Split(path, ".")
	type segment struct {
		key   string
		index *int
	}

	var segments []segment

	arrayRe := regexp.MustCompile(`^(.+?)\[(-?\d+)\]$`)
	for _, part := range parts {
		matches := arrayRe.FindStringSubmatch(part)
		if matches != nil {
			idx, _ := strconv.Atoi(matches[2])
			segments = append(segments, segment{key: matches[1], index: &idx})
		} else {
			segments = append(segments, segment{key: part, index: nil})
		}
	}

	current := obj
	for _, seg := range segments {
		m, ok := current.(map[string]any)
		if !ok {
			return nil
		}
		current, ok = m[seg.key]
		if !ok {
			return nil
		}

		if seg.index != nil {
			arr, ok := current.([]any)
			if !ok {
				return nil
			}
			idx := *seg.index
			if idx < 0 {
				idx = len(arr) + idx
			}
			if idx < 0 || idx >= len(arr) {
				return nil
			}
			current = arr[idx]
		}
	}

	return current
}

// applyFormatter applies a named formatter to a value.
func applyFormatter(value any, formatterString string, formatters map[string]Formatter) string {
	re := regexp.MustCompile(`^(\w+)(?:\(([^)]*)\))?$`)
	match := re.FindStringSubmatch(formatterString)
	if match == nil {
		return fmt.Sprintf("%v", value)
	}

	formatterName := match[1]
	argsString := match[2]

	formatter, ok := formatters[formatterName]
	if !ok {
		return fmt.Sprintf("%v", value)
	}

	var fmtArgs []string
	if argsString != "" {
		fmtArgs = parseFormatterArgs(argsString)
	}

	return formatter(value, fmtArgs...)
}

// parseFormatterArgs parses comma-separated arguments, respecting quoted strings.
func parseFormatterArgs(s string) []string {
	var args []string
	current := ""
	inQuotes := false
	quoteChar := byte(0)

	for i := 0; i < len(s); i++ {
		ch := s[i]
		if !inQuotes && (ch == '"' || ch == '\'') {
			inQuotes = true
			quoteChar = ch
			continue
		}
		if inQuotes && ch == quoteChar {
			inQuotes = false
			quoteChar = 0
			continue
		}
		if !inQuotes && ch == ',' {
			trimmed := strings.TrimSpace(current)
			if trimmed == "" {
				trimmed = " "
			}
			args = append(args, trimmed)
			current = ""
			continue
		}
		current += string(ch)
	}

	trimmed := strings.TrimSpace(current)
	if trimmed == "" {
		trimmed = " "
	}
	args = append(args, trimmed)

	return args
}

// splitFirst splits a string on the first occurrence of sep.
// Returns (original, "") if sep is not found.
func splitFirst(s, sep string) (string, string) {
	idx := strings.Index(s, sep)
	if idx < 0 {
		return s, ""
	}
	return s[:idx], s[idx+len(sep):]
}

// mergeFormatters merges two formatter maps. Values in override take precedence.
func mergeFormatters(base, override map[string]Formatter) map[string]Formatter {
	result := make(map[string]Formatter)
	for k, v := range base {
		result[k] = v
	}
	for k, v := range override {
		result[k] = v
	}
	return result
}
