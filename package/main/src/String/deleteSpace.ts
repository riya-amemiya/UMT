/**
 * 文字列からスペースを削除する
 * @param char
 * @returns string
 * @example deleteSpace("Hello World"); // "HelloWorld"
 */
export const deleteSpace = (char: string) => {
  return char.replaceAll(/\s/g, "");
};
