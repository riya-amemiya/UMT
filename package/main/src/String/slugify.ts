/**
 * Convert a string to a URL-friendly slug
 * @param str - The string to convert
 * @returns The slugified string
 * @example
 * slugify("Hello World!"); // "hello-world"
 * slugify("This is a Test"); // "this-is-a-test"
 * slugify("Japanese: こんにちは"); // "japanese"
 */
export const slugify = (string_: string): string => {
  return (
    string_
      .normalize("NFD")
      // Strip combining diacritical marks (e.g. accents)
      .replaceAll(/[\u0300-\u036F]/g, "")
      .toLowerCase()
      // Consolidate non-word chars, whitespace, and underscores into a single
      // hyphen in one pass instead of three separate regex scans
      .replaceAll(/[^\w-]+|_+/g, "-")
      // Collapse consecutive hyphens and strip leading/trailing hyphens
      .replaceAll(/-+/g, "-")
      .replaceAll(/^-|-$/g, "")
  );
};
