package str

import "testing"

func TestEscapeHtml(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"escape ampersand", "Tom & Jerry", "Tom &amp; Jerry"},
		{"escape less than", "5 < 10", "5 &lt; 10"},
		{"escape greater than", "10 > 5", "10 &gt; 5"},
		{"escape double quotes", `Say "Hello"`, "Say &quot;Hello&quot;"},
		{"escape single quotes", "It's working", "It&#39;s working"},
		{"escape all special characters",
			`<script>alert("XSS & 'injection'");</script>`,
			"&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;"},
		{"empty string", "", ""},
		{"no special characters", "Hello World", "Hello World"},
		{"only special characters", `&<>"'`, "&amp;&lt;&gt;&quot;&#39;"},
		{"repeated ampersands", "&&&", "&amp;&amp;&amp;"},
		{"repeated less than", "<<<", "&lt;&lt;&lt;"},
		{"mixed content",
			"User input: <b>Hello & 'World'</b>",
			"User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;"},
		{"HTML attributes",
			`<img src="test.jpg" alt="Tom & Jerry's picture">`,
			"&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;"},
		{"JavaScript code",
			`if (x < 5 && y > 3) { alert("Hello 'World'"); }`,
			"if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := EscapeHtml(tt.input)
			if result != tt.expected {
				t.Errorf("EscapeHtml(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestUnescapeHtml(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"unescape ampersand", "Tom &amp; Jerry", "Tom & Jerry"},
		{"unescape less than", "5 &lt; 10", "5 < 10"},
		{"unescape greater than", "10 &gt; 5", "10 > 5"},
		{"unescape double quotes", "Say &quot;Hello&quot;", `Say "Hello"`},
		{"unescape single quotes", "It&#39;s working", "It's working"},
		{"unescape hex single quotes", "It&#x27;s working", "It's working"},
		{"unescape all basic entities",
			"&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;",
			`<script>alert("XSS & 'injection'");</script>`},
		{"empty string", "", ""},
		{"no entities", "Hello World", "Hello World"},
		{"only entities", "&amp;&lt;&gt;&quot;&#39;", "&<>\"'"},
		{"repeated ampersand entities", "&amp;&amp;&amp;", "&&&"},
		{"repeated less than entities", "&lt;&lt;&lt;", "<<<"},
		{"mixed content",
			"User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;",
			"User input: <b>Hello & 'World'</b>"},
		{"HTML attributes",
			"&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;",
			`<img src="test.jpg" alt="Tom & Jerry's picture">`},
		{"JavaScript code",
			"if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }",
			`if (x < 5 && y > 3) { alert("Hello 'World'"); }`},
		// Decimal numeric references
		{"decimal A", "&#65;", "A"},
		{"decimal euro", "&#8364;", "\u20AC"},
		{"decimal grin emoji", "&#128512;", "\U0001F600"},
		// Hexadecimal numeric references
		{"hex A", "&#x41;", "A"},
		{"hex euro", "&#x20AC;", "\u20AC"},
		{"hex grin emoji", "&#x1F600;", "\U0001F600"},
		// Mixed case hex
		{"hex lowercase", "&#x20ac;", "\u20AC"},
		{"hex uppercase", "&#x20AC;", "\u20AC"},
		{"uppercase X not matched", "&#X41;", "&#X41;"},
		// Extended entities
		{"hex slash", "&#x2F;", "/"},
		{"hex backtick", "&#x60;", "`"},
		{"hex equals", "&#x3D;", "="},
		// Invalid numeric references
		{"empty decimal", "&#;", "&#;"},
		{"empty hex", "&#x;", "&#x;"},
		{"invalid decimal", "&#invalid;", "&#invalid;"},
		{"invalid hex", "&#xinvalid;", "&#xinvalid;"},
		// Malformed entities
		{"no semicolon", "&lt", "&lt"},
		{"unknown entity", "&unknown;", "&unknown;"},
		{"ampersand hash only", "&#", "&#"},
		{"nonexistent entity", "&nonexistent;", "&nonexistent;"},
		{"notinmap entity", "&notinmap;", "&notinmap;"},
		{"test entity", "&test;", "&test;"},
		// Unicode in numeric references
		{"decimal hiragana a", "&#12354;", "\u3042"},
		{"hex hiragana a", "&#x3042;", "\u3042"},
		{"decimal bullet", "&#8226;", "\u2022"},
		{"hex bullet", "&#x2022;", "\u2022"},
		// Preserve already unescaped content
		{"already unescaped", "Already < unescaped & content with 'quotes'", "Already < unescaped & content with 'quotes'"},
		// Complex HTML document
		{"complex HTML doc",
			"&lt;!DOCTYPE html&gt;\n&lt;html&gt;\n&lt;head&gt;\n    &lt;title&gt;Test &amp; Demo&lt;/title&gt;\n&lt;/head&gt;\n&lt;body&gt;\n    &lt;p&gt;Hello &#39;World&#39; &amp; welcome!&lt;/p&gt;\n&lt;/body&gt;\n&lt;/html&gt;",
			"<!DOCTYPE html>\n<html>\n<head>\n    <title>Test & Demo</title>\n</head>\n<body>\n    <p>Hello 'World' & welcome!</p>\n</body>\n</html>"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := UnescapeHtml(tt.input)
			if result != tt.expected {
				t.Errorf("UnescapeHtml(%q) = %q, want %q", tt.input, result, tt.expected)
			}
		})
	}
}

func TestEscapeUnescapeRoundtrip(t *testing.T) {
	testCases := []string{
		"Hello & World",
		`<script>alert("test");</script>`,
		"It's a 'test' & more",
		"5 < 10 > 3",
		"",
		"No special chars",
	}

	for _, tc := range testCases {
		escaped := EscapeHtml(tc)
		unescaped := UnescapeHtml(escaped)
		if unescaped != tc {
			t.Errorf("Roundtrip failed for %q: escaped=%q, unescaped=%q", tc, escaped, unescaped)
		}
	}
}
