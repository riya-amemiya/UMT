/**
 * 指定された長さに達するまで、文字列の先頭に別の文字列を繰り返し追加します。
 *
 * @param string_ - パディングを追加する元の文字列。
 * @param targetLength - パディングを含めた後の目標の文字列長。
 * @param padString - 元の文字列の先頭に追加される文字列。
 * @returns パディングが追加された後の文字列。
 */
export const padStart = (
  string_: string,
  targetLength: number,
  padString: string,
): string => {
  if (padString === "") {
    throw new Error("padString cannot be empty");
  }

  let padding = "";
  const paddingLength = targetLength - string_.length;
  while (padding.length < paddingLength) {
    padding += padString;
  }

  return padding.slice(0, paddingLength) + string_;
};
