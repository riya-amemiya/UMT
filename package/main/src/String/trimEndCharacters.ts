/**
 * 指定された文字列の末尾から、指定された文字をすべて取り除きます。
 *
 * @param {string} string_ - トリム処理を行う元の文字列。
 * @param {string} chars - 除去する文字のセット。
 * @returns {string} 末尾から指定された文字が取り除かれた新しい文字列。
 */
export const trimEndCharacters = (string_: string, chars: string): string => {
  let endIndex = string_.length - 1;
  while (endIndex >= 0 && chars.includes(string_[endIndex])) {
    endIndex--;
  }
  return string_.slice(0, endIndex + 1);
};
