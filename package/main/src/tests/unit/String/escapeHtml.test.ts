import { escapeHtml } from "@/String/escapeHtml";

describe("escapeHtml", () => {
  it("should escape ampersand", () => {
    expect(escapeHtml("Tom & Jerry")).toBe("Tom &amp; Jerry");
  });

  it("should escape less than", () => {
    expect(escapeHtml("5 < 10")).toBe("5 &lt; 10");
  });

  it("should escape greater than", () => {
    expect(escapeHtml("10 > 5")).toBe("10 &gt; 5");
  });

  it("should escape double quotes", () => {
    expect(escapeHtml('Say "Hello"')).toBe("Say &quot;Hello&quot;");
  });

  it("should escape single quotes", () => {
    expect(escapeHtml("It's working")).toBe("It&#39;s working");
  });

  it("should escape all special characters", () => {
    const input = `<script>alert("XSS & 'injection'");</script>`;
    const expected =
      "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;";

    expect(escapeHtml(input)).toBe(expected);
  });

  it("should handle empty string", () => {
    expect(escapeHtml("")).toBe("");
  });

  it("should handle string with no special characters", () => {
    expect(escapeHtml("Hello World")).toBe("Hello World");
  });

  it("should handle string with only special characters", () => {
    expect(escapeHtml("&<>\"'")).toBe("&amp;&lt;&gt;&quot;&#39;");
  });

  it("should handle repeated special characters", () => {
    expect(escapeHtml("&&&")).toBe("&amp;&amp;&amp;");
    expect(escapeHtml("<<<")).toBe("&lt;&lt;&lt;");
  });

  it("should handle mixed content", () => {
    const input = "User input: <b>Hello & 'World'</b>";
    const expected =
      "User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;";

    expect(escapeHtml(input)).toBe(expected);
  });

  it("should handle HTML attributes", () => {
    const input = `<img src="test.jpg" alt="Tom & Jerry's picture">`;
    const expected =
      "&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;";

    expect(escapeHtml(input)).toBe(expected);
  });

  it("should handle JavaScript code", () => {
    const input = `if (x < 5 && y > 3) { alert("Hello 'World'"); }`;
    const expected =
      "if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }";

    expect(escapeHtml(input)).toBe(expected);
  });
});
