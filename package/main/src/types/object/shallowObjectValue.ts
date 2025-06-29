import type { UnionToIntersection } from "$/logic/unionToIntersection";

/**
 * Extract the value type from the first level of an object
 *
 * @example
 * type Input = { a: { b: { c: {} } } }
 * type Output = ShallowObjectValue<Input> // { b: { c: {} } }
 */
export type ShallowObjectValue<T> = T extends Record<PropertyKey, infer U>
  ? UnionToIntersection<U>
  : never;
