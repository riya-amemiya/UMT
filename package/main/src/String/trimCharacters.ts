import { trimEndCharacters } from "./trimEndCharacters";
import { trimStartCharacters } from "./trimStartCharacters";

/**
 * Removes specified characters from both ends of a string.
 *
 * @param {string} string_ - The string to trim.
 * @param {string} chars - The set of characters to remove.
 * @returns {string} A new string with specified characters removed from both ends.
 */
export const trimCharacters = (string_: string, chars: string): string => {
  return trimEndCharacters(trimStartCharacters(string_, chars), chars);
};
