import { capitalizeWord } from "./capitalizeWord";
import { words } from "./words";

/**
 * Converts a string to Title Case (each word capitalized, separated by spaces).
 * @param {string} string_ - Input string
 * @returns {string} Title Case string
 * @example
 * titleCase("hello world"); // "Hello World"
 * titleCase("a-quick brown_fox"); // "A Quick Brown Fox"
 */
export const titleCase = (string_: string): string =>
  words(string_)
    .map((word) => capitalizeWord(word))
    .join(" ");
