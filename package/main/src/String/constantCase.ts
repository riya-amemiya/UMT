import { words } from "./words";

/**
 * Converts a string to CONSTANT_CASE.
 * @param {string} string_ - Input string
 * @returns {string} CONSTANT_CASE string
 * @example
 * constantCase("helloWorld"); // "HELLO_WORLD"
 * constantCase("Hello World"); // "HELLO_WORLD"
 */
export const constantCase = (string_: string): string =>
  words(string_)
    .map((word) => word.toUpperCase())
    .join("_");
