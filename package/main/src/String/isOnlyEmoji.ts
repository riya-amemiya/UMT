/**
 * Check if the text contains only emojis
 * @param text
 * @returns [boolean] true if the text contains only emojis, false otherwise
 */
export const isOnlyEmoji = (text: string) => {
  // Emoji pattern
  // \p[L] = Unicode letter
  // \p[N] = Unicode number
  // /u = Unicode flag
  // [^\p{L}\p{N}]* = Match any character that is not a Unicode letter or number
  const nonTextPattern = /^[^\p{L}\p{N}]*$/u;
  return nonTextPattern.test(text);
};
