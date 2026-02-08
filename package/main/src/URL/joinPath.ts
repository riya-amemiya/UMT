/**
 * Joins multiple path segments into a single path,
 * normalizing slashes between segments.
 *
 * Leading slash of the first segment is preserved.
 * Trailing slash of the last segment is preserved.
 * All intermediate slashes are normalized to a single slash.
 *
 * @param segments - The path segments to join
 * @returns The joined and normalized path
 *
 * @example
 * ```typescript
 * joinPath("https://example.com/", "/api/", "/users");
 * // "https://example.com/api/users"
 *
 * joinPath("/a/", "/b/", "/c");
 * // "/a/b/c"
 *
 * joinPath("a", "b", "c");
 * // "a/b/c"
 * ```
 */
export const joinPath = (...segments: string[]): string => {
  if (segments.length === 0) {
    return "";
  }

  const normalized: string[] = [];
  for (let index = 0; index < segments.length; index++) {
    let segment = segments[index];
    if (index > 0) {
      segment = segment.replace(/^\/+/, "");
    }
    if (index < segments.length - 1) {
      segment = segment.replace(/\/+$/, "");
    }
    if (segment.length > 0) {
      normalized.push(segment);
    }
  }

  return normalized.join("/");
};
