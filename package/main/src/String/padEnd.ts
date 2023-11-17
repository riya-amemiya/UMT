/**
 * 指定された長さに達するまで、文字列の末尾に指定された文字列を追加します。
 *
 * @param string_ - パディングを適用する元の文字列
 * @param targetLength - パディング後の目標の長さ
 * @param padString - パディングに使用する文字列
 * @returns パディングが適用された後の文字列
 */
export const padEnd = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  if (padString === "") {
    return string_;
  }
  let result = string_;
  while (result.length < targetLength) {
    result += padString.slice(0, targetLength - result.length);
  }
  return result;
};
