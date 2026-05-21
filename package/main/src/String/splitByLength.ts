/**
 * Splits a string into chunks of the specified length. Surrogate-pair safe,
 * so characters outside the Basic Multilingual Plane are not split.
 * @param {string} string_ - Input string
 * @param {number} length - Maximum length of each chunk
 * @returns {string[]} Array of string chunks
 * @example
 * splitByLength("abcdefg", 3); // ["abc", "def", "g"]
 */
export const splitByLength = (string_: string, length: number): string[] => {
  const chars = [...string_];
  const result: string[] = [];
  for (let index = 0; index < chars.length; index += length) {
    result.push(chars.slice(index, index + length).join(""));
  }
  return result;
};
