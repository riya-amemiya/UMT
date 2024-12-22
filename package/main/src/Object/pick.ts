/**
 * 指定されたキーを持つプロパティのみを抽出して新しいオブジェクトを作成します。
 *
 * @template T - 元のオブジェクトの型。
 * @template K - 抽出するキーの型（Tのキーのサブセット）。
 * @param {T} object - プロパティを抽出する元のオブジェクト。
 * @param {...K[]} keys - 抽出したいプロパティのキー。
 * @returns {Pick<T, K>} 指定されたキーを持つプロパティのみを含む新しいオブジェクト。
 *
 * @example
 * ```typescript
 * const user = { id: 1, name: 'Alice', age: 30 };
 * const selected = pick(user, 'id', 'name'); // { id: 1, name: 'Alice' }
 * ```
 */
export const pick = <T extends object, K extends keyof T>(
  object: T,
  ...keys: K[]
): Pick<T, K> => {
  const result = {} as Pick<T, K>;
  for (const key of keys) {
    result[key] = object[key];
  }
  return result;
};
