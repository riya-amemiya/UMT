/**
 * 指定したオブジェクトのプロパティをenumerable:falseに設定する
 * @param obj プロパティを変更するオブジェクト
 * @param key 対象のプロパティキー
 */
export function setEnumerableFalse<T extends object>(
  object: T,
  key: keyof T,
): T {
  return Object.defineProperty(object, key, {
    enumerable: false,
    configurable: true,
    writable: true,
    value: object[key],
  });
}
