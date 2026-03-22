import type { UnionToIntersection } from "$/logic/unionToIntersection";

/**
 * Merges multiple objects into a single object (shallow merge)
 * @param target - The target object to merge into
 * @param sources - The source objects to merge from
 * @returns The merged object
 */
export const merge = <
  T extends Record<string, unknown>,
  U extends Record<string, unknown>[],
>(
  target: T,
  ...sources: U
): T & UnionToIntersection<U[number]> => {
  const result: Record<string, unknown> = {};
  for (const object of [target, ...sources]) {
    for (const key of Object.keys(object)) {
      // Prevent prototype pollution by skipping dangerous keys
      if (key === "__proto__" || key === "constructor" || key === "prototype") {
        continue;
      }
      result[key] = object[key];
    }
  }
  return result as T & UnionToIntersection<U[number]>;
};
