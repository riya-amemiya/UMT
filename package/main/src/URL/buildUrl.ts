/**
 * Builds a URL with query parameters appended.
 *
 * @param base - The base URL string
 * @param params - An object of key-value pairs to append as query parameters
 * @returns The complete URL string with query parameters
 *
 * @example
 * ```typescript
 * buildUrl("https://example.com", { page: "1", q: "search" });
 * // "https://example.com/?page=1&q=search"
 *
 * buildUrl("https://example.com/path", { foo: "bar" });
 * // "https://example.com/path?foo=bar"
 * ```
 */
export const buildUrl = (
  base: string,
  parameters: Record<string, string> = {},
): string => {
  const url = new URL(base);
  for (const key of Object.keys(parameters)) {
    url.searchParams.append(key, parameters[key]);
  }
  return url.toString();
};
