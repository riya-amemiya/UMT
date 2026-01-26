package str

import "testing"

func TestCamelCase(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Convert kebab-case to camelCase
		{"kebab hello-world", "hello-world", "helloWorld"},
		{"kebab foo-bar-baz", "foo-bar-baz", "fooBarBaz"},
		// Convert snake_case to camelCase
		{"snake hello_world", "hello_world", "helloWorld"},
		{"snake foo_bar_baz", "foo_bar_baz", "fooBarBaz"},
		// Convert space-separated words
		{"space hello world", "hello world", "helloWorld"},
		{"space foo bar baz", "foo bar baz", "fooBarBaz"},
		// Handle PascalCase input
		{"pascal HelloWorld", "HelloWorld", "helloWorld"},
		{"pascal FooBarBaz", "FooBarBaz", "fooBarBaz"},
		// Handle already camelCase
		{"camel helloWorld", "helloWorld", "helloWorld"},
		{"camel fooBarBaz", "fooBarBaz", "fooBarBaz"},
		// Handle mixed separators
		{"mixed hello-world_test case", "hello-world_test case", "helloWorldTestCase"},
		{"mixed foo_bar-baz qux", "foo_bar-baz qux", "fooBarBazQux"},
		// Handle numbers
		{"numbers hello-world-2", "hello-world-2", "helloWorld2"},
		{"numbers test_case_123", "test_case_123", "testCase123"},
		// Handle empty string
		{"empty string", "", ""},
		// Handle single word
		{"single hello", "hello", "hello"},
		{"single HELLO", "HELLO", "hELLO"},
		// Handle special characters
		{"special hello@world", "hello@world", "helloWorld"},
		{"special foo#bar$baz", "foo#bar$baz", "fooBarBaz"},
		// Handle leading/trailing separators
		{"leading/trailing -hello-world-", "-hello-world-", "helloWorld"},
		{"leading/trailing _foo_bar_", "_foo_bar_", "fooBar"},
		// Handle multiple consecutive separators
		{"consecutive hello---world", "hello---world", "helloWorld"},
		{"consecutive foo___bar", "foo___bar", "fooBar"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := CamelCase(tt.input)
			if result != tt.expected {
				t.Errorf("CamelCase(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestKebabCase(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Convert camelCase to kebab-case
		{"camel helloWorld", "helloWorld", "hello-world"},
		{"camel fooBarBaz", "fooBarBaz", "foo-bar-baz"},
		// Convert PascalCase
		{"pascal HelloWorld", "HelloWorld", "hello-world"},
		{"pascal FooBarBaz", "FooBarBaz", "foo-bar-baz"},
		// Convert snake_case
		{"snake hello_world", "hello_world", "hello-world"},
		{"snake foo_bar_baz", "foo_bar_baz", "foo-bar-baz"},
		// Convert space-separated
		{"space hello world", "hello world", "hello-world"},
		{"space foo bar baz", "foo bar baz", "foo-bar-baz"},
		// Handle already kebab-case
		{"kebab hello-world", "hello-world", "hello-world"},
		{"kebab foo-bar-baz", "foo-bar-baz", "foo-bar-baz"},
		// Handle mixed separators
		{"mixed helloWorld_test case", "helloWorld_test case", "hello-world-test-case"},
		{"mixed fooBar-baz qux", "fooBar-baz qux", "foo-bar-baz-qux"},
		// Handle numbers
		{"numbers helloWorld2", "helloWorld2", "hello-world2"},
		{"numbers testCase123", "testCase123", "test-case123"},
		// Handle empty string
		{"empty string", "", ""},
		// Handle single word
		{"single hello", "hello", "hello"},
		{"single HELLO", "HELLO", "hello"},
		// Handle special characters
		{"special hello@world", "hello@world", "hello-world"},
		{"special foo#bar$baz", "foo#bar$baz", "foo-bar-baz"},
		// Handle leading/trailing separators
		{"leading/trailing -hello-world-", "-hello-world-", "hello-world"},
		{"leading/trailing _foo_bar_", "_foo_bar_", "foo-bar"},
		// Handle multiple consecutive separators
		{"consecutive hello---world", "hello---world", "hello-world"},
		{"consecutive foo___bar", "foo___bar", "foo-bar"},
		// Handle complex mixed case
		{"complex XMLHttpRequest", "XMLHttpRequest", "xml-http-request"},
		{"complex getElementById", "getElementById", "get-element-by-id"},
		// Handle acronyms
		{"acronym HTML", "HTML", "html"},
		{"acronym XMLParser", "XMLParser", "xml-parser"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := KebabCase(tt.input)
			if result != tt.expected {
				t.Errorf("KebabCase(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestSlugify(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// JSDoc examples
		{"Hello World!", "Hello World!", "hello-world"},
		{"This is a Test", "This is a Test", "this-is-a-test"},
		{"Japanese", "Japanese: \u3053\u3093\u306b\u3061\u306f", "japanese"},
		// Multiple spaces
		{"multiple spaces", "Hello    World", "hello-world"},
		// Leading and trailing spaces
		{"leading/trailing spaces", "  Hello World  ", "hello-world"},
		// Remove special characters
		{"special characters", "Special!@#$%Characters", "special-characters"},
		// Handle underscores
		{"underscores", "snake_case_text", "snake-case-text"},
		// Handle unicode characters
		{"unicode cafe", "caf\u00e9", "cafe"},
		{"unicode naive", "na\u00efve", "naive"},
		// Handle numbers
		{"numbers Test 123", "Test 123", "test-123"},
		{"numbers Version 2.5", "Version 2.5", "version-2-5"},
		// Handle empty string
		{"empty string", "", ""},
		// Handle consecutive hyphens
		{"consecutive hyphens", "Hello---World", "hello-world"},
		// Handle mixed case
		{"mixed case", "CamelCase", "camelcase"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := Slugify(tt.input)
			if result != tt.expected {
				t.Errorf("Slugify(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}
