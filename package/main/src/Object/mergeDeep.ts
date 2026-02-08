import type { UnionToIntersection } from "$/logic/unionToIntersection";
import { isPlainObject } from "@/Object/isPlainObject";

/**
 * Deeply merges multiple objects into a single object
 * @param target - The target object to merge into
 * @param sources - The source objects to merge from
 * @returns The deeply merged object
 */
export const mergeDeep = <
  T extends Record<string, unknown>,
  U extends Record<string, unknown>[],
>(
  target: T,
  ...sources: U
): T & UnionToIntersection<U[number]> => {
  if (sources.length === 0) {
    return target as T & UnionToIntersection<U[number]>;
  }

  const source = sources.shift();

  if (isPlainObject(target) && isPlainObject(source)) {
    const result = { ...target };

    for (const key in source) {
      if (Object.hasOwn(source, key)) {
        const sourceValue = source[key];
        const targetValue = result[key];

        (result as Record<string, unknown>)[key] =
          isPlainObject(targetValue) && isPlainObject(sourceValue)
            ? mergeDeep(targetValue, sourceValue)
            : sourceValue;
      }
    }

    return mergeDeep(result, ...sources) as T & UnionToIntersection<U[number]>;
  }

  return mergeDeep(source as T, ...sources) as T &
    UnionToIntersection<U[number]>;
};
