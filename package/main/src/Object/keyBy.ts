type PropertyName = string | number | symbol;
type IterateeFunction<T> = (value: T) => PropertyName;
type Iteratee<T> = IterateeFunction<T> | keyof T;

/**
 * コレクションの要素をイテレータ関数の結果でキー化したオブジェクトを生成します
 * @param collection 処理対象のコレクション
 * @param iteratee キーを生成するイテレータ関数またはプロパティ名
 */
export function keyBy<T>(
  collection: T[] | Record<PropertyName, T>,
  iteratee?: Iteratee<T>,
): Record<PropertyName, T> {
  const getKey = normalizeIteratee(iteratee);
  const result: Record<PropertyName, T> = {};

  if (Array.isArray(collection)) {
    for (const value of collection) {
      const key = getKey(value);
      result[key] = value;
    }
    return result;
  }

  for (const value of Object.values(collection)) {
    const key = getKey(value);
    result[key] = value;
  }
  return result;
}

/**
 * イテレータ関数の正規化
 */
function normalizeIteratee<T>(iteratee?: Iteratee<T>): IterateeFunction<T> {
  if (!iteratee) {
    return (value) => value as unknown as PropertyName;
  }
  if (typeof iteratee === "function") {
    return iteratee;
  }
  return (object: T) => object[iteratee] as PropertyName;
}
