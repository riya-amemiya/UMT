/**
 * 辞書型のオブジェクトかどうかを判定する
 * @param object
 * @returns {boolean} true: 辞書型オブジェクト, false: 辞書型オブジェクトでない
 * @example isDictionaryObject({}); // true
 * isDictionaryObject([]); // false
 */
export const isDictionaryObject = <T extends { [key: string]: unknown }>(
  object: unknown,
): object is T => {
  return (
    typeof object === "object" && object !== null && !Array.isArray(object)
  );
};
