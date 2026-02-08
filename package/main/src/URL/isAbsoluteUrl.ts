/**
 * Checks whether a URL string is absolute (RFC 3986).
 *
 * An absolute URL starts with a scheme followed by a colon,
 * where the scheme begins with a letter and may contain
 * letters, digits, plus, hyphen, or period.
 *
 * @param url - The URL string to check
 * @returns True if the URL is absolute, false otherwise
 *
 * @example
 * ```typescript
 * isAbsoluteUrl("https://example.com");  // true
 * isAbsoluteUrl("ftp://files.example");  // true
 * isAbsoluteUrl("/path/to/page");        // false
 * isAbsoluteUrl("relative/path");        // false
 * isAbsoluteUrl("mailto:user@host");     // true
 * ```
 */
export const isAbsoluteUrl = (url: string): boolean => {
  return /^[a-z][a-z\d+\-.]*:/i.test(url);
};
