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
  return string_
    .normalize("NFD")
    .replaceAll(/[\u0300-\u036F]/g, "")
    .toLowerCase()
    .replaceAll(/[^\w\s-]/g, "-")
    .replaceAll(/\s+/g, "-")
    .replaceAll(/_+/g, "-")
    .replaceAll(/-+/g, "-")
    .replaceAll(/^-+|-+$/g, "");
};
