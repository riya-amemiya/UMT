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
 * Security: Checks whether a numeric code point is safe to decode.
 * Rejects NULL (0), C0 control chars (1-31 except TAB 9, LF 10, CR 13),
 * DEL (127), C1 control chars (128-159), lone surrogates (0xD800-0xDFFF),
 * and values beyond the Unicode maximum (>0x10FFFF).
 */
const isSafeCodePoint = (codePoint: number): boolean => {
  if (codePoint > 0x10_ff_ff) {
    return false;
  }
  if (codePoint === 0) {
    return false;
  }
  if (codePoint >= 0xd8_00 && codePoint <= 0xdf_ff) {
    return false;
  }
  if (
    codePoint <= 0x1f &&
    codePoint !== 0x09 &&
    codePoint !== 0x0a &&
    codePoint !== 0x0d
  ) {
    return false;
  }
  if (codePoint === 0x7f) {
    return false;
  }
  if (codePoint >= 0x80 && codePoint <= 0x9f) {
    return false;
  }
  return true;
};

/**
 * Unescapes HTML entities in a string
 * @param string_ - The string to unescape
 * @returns The unescaped string with HTML entities converted back to their original characters
 * @example
 * ```typescript
 * unescapeHtml("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;");
 * // Returns: "<script>alert(\"Hello\");</script>"
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
    /&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);|&#(\d{1,7});|&#x([\dA-Fa-f]{1,6});/g;

  return string_.replaceAll(entityRegex, (match, dec, hex) => {
    if (dec !== undefined) {
      const codePoint = Number.parseInt(dec, 10);
      // Security: reject dangerous code points to prevent injection attacks.
      // NULL bytes (0) enable null-byte injection that can truncate strings in
      // downstream systems. C0 control chars (1-31 except TAB/LF/CR) and the
      // DEL char (127) can break parsers. C1 control chars (128-159) are
      // invalid in HTML. Surrogate code points (0xD800-0xDFFF) produce
      // malformed strings. Out-of-range values (>0x10FFFF) are not valid
      // Unicode. Leave these entity references unmodified rather than decoding
      // them into potentially dangerous characters.
      if (!isSafeCodePoint(codePoint)) {
        return match;
      }
      return String.fromCodePoint(codePoint);
    }
    if (hex !== undefined) {
      const codePoint = Number.parseInt(hex, 16);
      if (!isSafeCodePoint(codePoint)) {
        return match;
      }
      return String.fromCodePoint(codePoint);
    }
    return htmlUnescapeMap[match];
  });
};
