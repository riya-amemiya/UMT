import type { PickDeep } from "$/object/pickDeep";
import type { PickDeepKey } from "$/object/pickDeepKey";

/**
 * オブジェクトから指定されたキーに基づいて深くプロパティを選択し、新しいオブジェクトを作成します。
 *
 * @template T - 元のオブジェクトの型。
 * @template K - 選択するプロパティキーの型。PickDeepKey<T>のサブセット。
 * @param {T} object - プロパティを抽出する元のオブジェクト。
 * @param {...K[]} keys - 抽出したいプロパティのキー。ネストされたキーをドットで区切ることができます。
 * @returns {PickDeep<T>} 指定されたキーを持つプロパティのみを含む新しいオブジェクト。
 *
 * @example
 * ```typescript
 * const obj = { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 };
 * const picked = pickDeep(obj, 'a.b.c', 'f');
 * // picked は { a: { b: { c: 1 } }, f: 4 } となる
 * ```
 */
export const pickDeep = <T extends object, K extends PickDeepKey<T>>(
  object: T,
  ...keys: K[]
): PickDeep<T> => {
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const result: any = {};

  for (const key of keys) {
    const parts = (key as string).split(".");
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    let current: any = { ...object };
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    let target: any = result;

    for (const [index, part] of parts.entries()) {
      if (current && typeof current === "object" && part in current) {
        if (index === parts.length - 1) {
          target[part] = current[part];
        } else {
          target[part] = target[part] || {};
          current = current[part];
          target = target[part];
        }
      }
    }
  }

  return result;
};
