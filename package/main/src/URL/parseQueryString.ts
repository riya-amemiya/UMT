/**
 * Parses a query string into a key-value record.
 *
 * Accepts either a full URL or a raw query string
 * (with or without leading "?").
 *
 * @param query - The query string or URL to parse
 * @returns A record of key-value pairs from the query string
 *
 * @example
 * ```typescript
 * parseQueryString("?page=1&q=search");
 * // { page: "1", q: "search" }
 *
 * parseQueryString("foo=bar&baz=qux");
 * // { foo: "bar", baz: "qux" }
 *
 * parseQueryString("https://example.com?a=1&b=2");
 * // { a: "1", b: "2" }
 * ```
 */
export const parseQueryString = (query: string): Record<string, string> => {
  let searchString = query;
  if (query.includes("://")) {
    searchString = new URL(query).search;
  }

  const parameters = new URLSearchParams(searchString);
  const result: Record<string, string> = {};
  for (const [key, value] of parameters) {
    result[key] = value;
  }
  return result;
};
