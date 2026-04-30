import { words } from "./words";

/**
 * Counts words in a string using the same boundaries as `words`.
 *
 * @param {string} string_ - Input string
 * @returns {number} Word count
 * @example
 * wordCount("hello world"); // 2
 * wordCount("camelCase split"); // 3
 */
export const wordCount = (string_: string): number => words(string_).length;
