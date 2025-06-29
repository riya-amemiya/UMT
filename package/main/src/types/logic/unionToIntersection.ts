/**
 * Converts a union type to an intersection type
 *
 * @example
 * ```typescript
 * type A = { a: string }
 * type B = { b: number }
 * type Union = A | B
 * type Intersection = UnionToIntersection<Union> // { a: string } & { b: number }
 * ```
 */
export type UnionToIntersection<U> = (
  U extends unknown
    ? (k: U) => void
    : never
) extends (k: infer I) => void
  ? I
  : never;
