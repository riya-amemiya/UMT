import { words } from "./words";

/**
 * Converts a string to snake_case.
 * @param {string} string_ - Input string
 * @returns {string} snake_case string
 * @example
 * snakeCase("helloWorld"); // "hello_world"
 * snakeCase("Hello World"); // "hello_world"
 */
export const snakeCase = (string_: string): string =>
  words(string_)
    .map((word) => word.toLowerCase())
    .join("_");
