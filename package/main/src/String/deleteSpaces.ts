/**
 * 文字列からスペースを削除する
 * @param char
 * @returns string
 * @example deleteSpace("Hello World"); // "HelloWorld"
 */
export const deleteSpaces = (string_: string) => {
  return string_.replaceAll(/\s/g, "");
};
