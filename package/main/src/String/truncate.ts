/**
 * Truncate a string to a specified length
 * @param str - The string to truncate
 * @param length - The maximum length
 * @param suffix - The suffix to add when truncating (default: "...")
 * @returns The truncated string
 * @example
 * truncate("Hello World", 5); // "Hello..."
 * truncate("Hello World", 5, "~"); // "Hello~"
 * truncate("Hello", 10); // "Hello"
 */
export const truncate = (
  string_: string,
  length: number,
  suffix = "...",
): string => {
  if (string_.length <= length) {
    return string_;
  }

  return string_.slice(0, length) + suffix;
};
