/**
 * 指定された文字列の先頭から、指定された文字を取り除きます。
 *
 * @param {string} string_ - 処理対象の文字列。
 * @param {string} chars - 取り除く文字が含まれている文字列。
 * @returns {string} 先頭から指定された文字を取り除いた新しい文字列。
 */
export const trimStartCharacters = (string_: string, chars: string): string => {
  let startIndex = 0;
  while (startIndex < string_.length && chars.includes(string_[startIndex])) {
    startIndex++;
  }
  return string_.slice(startIndex);
};
