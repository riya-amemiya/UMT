/**
 * HTML entities map for escaping
 */
const htmlEscapeMap: Record<string, string> = {
  "&": "&amp;",
  "<": "&lt;",
  ">": "&gt;",
  '"': "&quot;",
  "'": "&#39;",
};

/**
 * Escapes HTML special characters in a string
 * @param str - The string to escape
 * @returns The escaped string
 */
export const escapeHtml = (string_: string): string => {
  return string_.replaceAll(/[&<>"']/g, (match) => htmlEscapeMap[match]);
};
