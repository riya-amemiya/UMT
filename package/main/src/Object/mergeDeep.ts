import type { UnionToIntersection } from "$/logic/unionToIntersection";
import { isPlainObject } from "@/Object/isPlainObject";

const MAX_MERGE_DEPTH = 100;

const mergeDeepInternal = <
  T extends Record<string, unknown>,
  U extends Record<string, unknown>[],
>(
  target: T,
  sources: U,
  depth: number,
): T & UnionToIntersection<U[number]> => {
  if (depth > MAX_MERGE_DEPTH) {
    throw new Error(
      `mergeDeep: maximum recursion depth of ${MAX_MERGE_DEPTH} exceeded`,
    );
  }

  if (sources.length === 0) {
    return target as T & UnionToIntersection<U[number]>;
  }

  const source = sources.shift();

  if (isPlainObject(target) && isPlainObject(source)) {
    const result = { ...target };

    for (const key in source) {
      if (key === "__proto__" || key === "constructor" || key === "prototype") {
        continue;
      }

      if (Object.hasOwn(source, key)) {
        const sourceValue = source[key];
        const targetValue = result[key];

        (result as Record<string, unknown>)[key] =
          isPlainObject(targetValue) && isPlainObject(sourceValue)
            ? mergeDeepInternal(targetValue, [sourceValue], depth + 1)
            : sourceValue;
      }
    }

    return mergeDeepInternal(result, sources as unknown as U, depth + 1) as T &
      UnionToIntersection<U[number]>;
  }

  return mergeDeepInternal(
    source as T,
    sources as unknown as U,
    depth + 1,
  ) as T & UnionToIntersection<U[number]>;
};

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
  return mergeDeepInternal(target, sources, 0);
};
