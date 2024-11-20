/**
 * Check if the text contains only emojis
 * @param text
 * @returns [boolean] true if the text contains only emojis, false otherwise
 */
export const hasNoLetters = (text: string) => {
  // \p[L] = Unicode letter
  // /u = Unicode flag
  // [^\p{L}]* = any character that is not a letter
  const nonTextPattern = /^[^\p{L}]*$/u;
  return nonTextPattern.test(text);
};
