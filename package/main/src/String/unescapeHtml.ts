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
  const entityRegex =
    /&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D|test);|&#(\d*);|&#x([0-9a-fA-F]*);/g;

  return string_.replaceAll(entityRegex, (match, dec, hex) => {
    if (dec !== undefined) {
      const codePoint = Number.parseInt(dec, 10);
      return Number.isNaN(codePoint) ? match : String.fromCodePoint(codePoint);
    }
    if (hex !== undefined) {
      const codePoint = Number.parseInt(hex, 16);
      return Number.isNaN(codePoint) ? match : String.fromCodePoint(codePoint);
    }
    return htmlUnescapeMap[match] || match;
  });
};
