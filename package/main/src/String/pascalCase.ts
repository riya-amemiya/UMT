import { capitalizeWord } from "./capitalizeWord";
import { words } from "./words";

/**
 * Converts a string to PascalCase.
 * @param {string} string_ - Input string
 * @returns {string} PascalCase string
 * @example
 * pascalCase("hello-world"); // "HelloWorld"
 * pascalCase("hello_world"); // "HelloWorld"
 */
export const pascalCase = (string_: string): string =>
  words(string_)
    .map((word) => capitalizeWord(word))
    .join("");
