/**
 * 文字列を逆順にする
 * @param {string} char
 * @returns string
 * @example reverseString("Hello"); // "olleH"
 */
export const reverseString = (char: string): string => {
  return char.split("").reverse().join("");
};
