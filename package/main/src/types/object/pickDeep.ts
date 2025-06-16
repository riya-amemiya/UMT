import type { PickDeepKey } from "./pickDeepKey";

// Helper type to get value at nested path
type GetValueAtPath<
  T,
  P extends string,
> = P extends `${infer Key}.${infer Rest}`
  ? Key extends keyof T
    ? T[Key] extends object
      ? GetValueAtPath<T[Key], Rest>
      : never
    : never
  : P extends keyof T
    ? T[P]
    : never;

// Helper type to construct nested object from dot-notation path
type ConstructNestedObject<
  P extends string,
  V,
> = P extends `${infer Key}.${infer Rest}`
  ? { [K in Key]: ConstructNestedObject<Rest, V> }
  : { [K in P]: V };

// Convert union to intersection
type UnionToIntersection<U> = (
  U extends unknown
    ? (k: U) => void
    : never
) extends (k: infer I) => void
  ? I
  : never;

// Helper to process a tuple of keys with PickDeepKey constraint
type ProcessKeys<T extends object, K extends readonly PickDeepKey<T>[]> = {
  [I in keyof K]: K[I] extends string
    ? ConstructNestedObject<K[I], GetValueAtPath<T, K[I]>>
    : never;
}[number];

// Pick multiple deep paths and merge them with PickDeepKey constraint
export type PickDeep<
  T extends object,
  K extends readonly PickDeepKey<T>[],
> = UnionToIntersection<ProcessKeys<T, K>>;
