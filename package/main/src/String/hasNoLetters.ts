/**
 * Check if the string has no letters (only emojis, numbers, or special characters)
 * @param text
 * @returns boolean - true if the string has no letters
 */
export const hasNoLetters = (text: string) => {
  // \p[L] = Unicode letter
  // /u = Unicode flag
  // [^\p{L}]* = any character that is not a letter
  const nonTextPattern = /^[^\p{L}]*$/u;
  return nonTextPattern.test(text);
};
