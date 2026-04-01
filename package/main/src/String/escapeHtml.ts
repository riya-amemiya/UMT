/**
 * HTML entities map for escaping.
 *
 * Security: The backtick (`) is included because it can be used as an HTML
 * attribute delimiter in older browsers (e.g. IE), enabling XSS via
 * constructs like <img src=`javascript:alert(1)`>. The forward slash (/)
 * is included per OWASP XSS prevention recommendations to mitigate tag-
 * closing injection (e.g. "</script>") when user input is embedded inside
 * HTML elements.
 */
const htmlEscapeMap: Record<string, string> = {
  "&": "&amp;",
  "<": "&lt;",
  ">": "&gt;",
  '"': "&quot;",
  "'": "&#39;",
  "`": "&#x60;",
  "/": "&#x2F;",
};

/**
 * Escapes HTML special characters in a string
 * @param str - The string to escape
 * @returns The escaped string
 */
export const escapeHtml = (string_: string): string => {
  return string_.replaceAll(/["&'/<>`]/g, (match) => htmlEscapeMap[match]);
};
