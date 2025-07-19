import { unescapeHtml } from "@/String/unescapeHtml";

describe("unescapeHtml", () => {
  it("should unescape ampersand", () => {
    expect(unescapeHtml("Tom &amp; Jerry")).toBe("Tom & Jerry");
  });

  it("should unescape less than", () => {
    expect(unescapeHtml("5 &lt; 10")).toBe("5 < 10");
  });

  it("should unescape greater than", () => {
    expect(unescapeHtml("10 &gt; 5")).toBe("10 > 5");
  });

  it("should unescape double quotes", () => {
    expect(unescapeHtml("Say &quot;Hello&quot;")).toBe('Say "Hello"');
  });

  it("should unescape single quotes", () => {
    expect(unescapeHtml("It&#39;s working")).toBe("It's working");
  });

  it("should unescape hexadecimal single quotes", () => {
    expect(unescapeHtml("It&#x27;s working")).toBe("It's working");
  });

  it("should unescape all basic entities", () => {
    const input =
      "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;";
    const expected = `<script>alert("XSS & 'injection'");</script>`;

    expect(unescapeHtml(input)).toBe(expected);
  });

  it("should handle empty string", () => {
    expect(unescapeHtml("")).toBe("");
  });

  it("should handle string with no entities", () => {
    expect(unescapeHtml("Hello World")).toBe("Hello World");
  });

  it("should handle string with only entities", () => {
    expect(unescapeHtml("&amp;&lt;&gt;&quot;&#39;")).toBe("&<>\"'");
  });

  it("should handle repeated entities", () => {
    expect(unescapeHtml("&amp;&amp;&amp;")).toBe("&&&");
    expect(unescapeHtml("&lt;&lt;&lt;")).toBe("<<<");
  });

  it("should handle mixed content", () => {
    const input = "User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;";
    const expected = "User input: <b>Hello & 'World'</b>";

    expect(unescapeHtml(input)).toBe(expected);
  });

  it("should handle HTML attributes", () => {
    const input =
      "&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;";
    const expected = `<img src="test.jpg" alt="Tom & Jerry's picture">`;

    expect(unescapeHtml(input)).toBe(expected);
  });

  it("should handle JavaScript code", () => {
    const input =
      "if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }";
    const expected = `if (x < 5 && y > 3) { alert("Hello 'World'"); }`;

    expect(unescapeHtml(input)).toBe(expected);
  });

  it("should handle decimal numeric character references", () => {
    expect(unescapeHtml("&#65;")).toBe("A");
    expect(unescapeHtml("&#8364;")).toBe("€");
    expect(unescapeHtml("&#128512;")).toBe("😀");
  });

  it("should handle hexadecimal numeric character references", () => {
    expect(unescapeHtml("&#x41;")).toBe("A");
    expect(unescapeHtml("&#x20AC;")).toBe("€");
    expect(unescapeHtml("&#x1F600;")).toBe("😀");
  });

  it("should handle mixed case hexadecimal references", () => {
    expect(unescapeHtml("&#x41;")).toBe("A");
    expect(unescapeHtml("&#X41;")).toBe("&#X41;"); // Should not match uppercase X
    expect(unescapeHtml("&#x20ac;")).toBe("€");
    expect(unescapeHtml("&#x20AC;")).toBe("€");
  });

  it("should handle extended entities", () => {
    expect(unescapeHtml("&#x2F;")).toBe("/");
    expect(unescapeHtml("&#x60;")).toBe("`");
    expect(unescapeHtml("&#x3D;")).toBe("=");
  });

  it("should handle invalid numeric references gracefully", () => {
    expect(unescapeHtml("&#;")).toBe("&#;");
    expect(unescapeHtml("&#x;")).toBe("&#x;");
    expect(unescapeHtml("&#invalid;")).toBe("&#invalid;");
    expect(unescapeHtml("&#xinvalid;")).toBe("&#xinvalid;");
  });

  it("should handle malformed entities", () => {
    expect(unescapeHtml("&lt")).toBe("&lt"); // Missing semicolon
    expect(unescapeHtml("&unknown;")).toBe("&unknown;"); // Unknown entity
    expect(unescapeHtml("&#")).toBe("&#"); // Incomplete
  });

  it("should handle complex HTML document", () => {
    const input = `&lt;!DOCTYPE html&gt;
&lt;html&gt;
&lt;head&gt;
    &lt;title&gt;Test &amp; Demo&lt;/title&gt;
&lt;/head&gt;
&lt;body&gt;
    &lt;p&gt;Hello &#39;World&#39; &amp; welcome!&lt;/p&gt;
&lt;/body&gt;
&lt;/html&gt;`;

    const expected = `<!DOCTYPE html>
<html>
<head>
    <title>Test & Demo</title>
</head>
<body>
    <p>Hello 'World' & welcome!</p>
</body>
</html>`;

    expect(unescapeHtml(input)).toBe(expected);
  });

  it("should be inverse of escapeHtml", async () => {
    const testCases = [
      "Hello & World",
      '<script>alert("test");</script>',
      "It's a 'test' & more",
      "5 < 10 > 3",
      "",
      "No special chars",
    ];

    // Note: We need to import escapeHtml to test roundtrip
    const { escapeHtml } = await import("@/String/escapeHtml");

    for (const testCase of testCases) {
      const escaped = escapeHtml(testCase);
      const unescaped = unescapeHtml(escaped);
      expect(unescaped).toBe(testCase);
    }
  });

  it("should handle Unicode characters in numeric references", () => {
    expect(unescapeHtml("&#12354;")).toBe("あ"); // Japanese hiragana
    expect(unescapeHtml("&#x3042;")).toBe("あ"); // Same character in hex
    expect(unescapeHtml("&#8226;")).toBe("•"); // Bullet point
    expect(unescapeHtml("&#x2022;")).toBe("•"); // Same in hex
  });

  it("should preserve already unescaped content", () => {
    const input = "Already < unescaped & content with 'quotes'";
    expect(unescapeHtml(input)).toBe(input);
  });
});
