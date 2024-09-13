/**
 * オブジェクトが指定されたパスを持っているかどうかを判定する
 * @param {T} object - 判定するオブジェクト
 * @param {string | string[]} path - 判定するパス
 * @returns {boolean} パスが存在すればtrue、そうでなければfalse
 * @example has({ a: { b: 1 } }, "a.b"); // true
 * has({ a: { b: 1 } }, ["a", "b"]); // true
 * has({ a: { b: 1 } }, "a.c"); // false
 */
export const has = <T extends { [key: string]: unknown }>(
  object: T,
  path: string | string[],
): boolean => {
  const localPath = typeof path === "string" ? path.split(".") : path;
  let current = { ...object };
  for (const key of localPath) {
    if (
      current == null ||
      !Object.prototype.hasOwnProperty.call(current, key)
    ) {
      return false;
    }
    current = current[key] as T;
  }
  return true;
};
