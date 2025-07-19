/**
 * HTML entities map for unescaping
 */
const htmlUnescapeMap: Record<string, string> = {
  "&amp;": "&",
  "&lt;": "<",
  "&gt;": ">",
  "&quot;": '"',
  "&#39;": "'",
  "&#x27;": "'",
  "&#x2F;": "/",
  "&#x60;": "`",
  "&#x3D;": "=",
};

/**
 * Unescapes HTML entities in a string
 * @param str - The string to unescape
 * @returns The unescaped string with HTML entities converted back to their original characters
 * @example
 * ```typescript
 * unescapeHtml("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;");
 * // Returns: "<script>alert("Hello");</script>"
 *
 * unescapeHtml("Tom &amp; Jerry");
 * // Returns: "Tom & Jerry"
 *
 * unescapeHtml("5 &lt; 10 &amp;&amp; 10 &gt; 5");
 * // Returns: "5 < 10 && 10 > 5"
 * ```
 */
export const unescapeHtml = (string_: string): string => {
  let result = string_;

  for (const [entity, character] of Object.entries(htmlUnescapeMap)) {
    result = result.replaceAll(entity, character);
  }

  result = result.replaceAll(/&#(\d+);/g, (_, code) => {
    const codePoint = Number.parseInt(code, 10);
    return Number.isNaN(codePoint) ? _ : String.fromCodePoint(codePoint);
  });

  result = result.replaceAll(/&#x([0-9a-fA-F]+);/g, (_, code) => {
    const codePoint = Number.parseInt(code, 16);
    return Number.isNaN(codePoint) ? _ : String.fromCodePoint(codePoint);
  });

  return result;
};
