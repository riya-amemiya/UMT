type OmitDeepPath<T, P extends string> = P extends `${infer Key}.${infer Rest}`
  ? Key extends keyof T
    ? {
        [K in keyof T]: K extends Key
          ? T[K] extends object
            ? OmitDeepPath<T[K], Rest>
            : T[K]
          : T[K];
      }
    : T
  : Omit<T, P>;

/**
Removes multiple dot-notation paths from an object type, applying each path
in order.

@template T - Source object type
@template K - Tuple of dot-separated paths to remove
@example
type Result = OmitDeep<{ a: { b: 1; c: 2 }; d: 3 }, ["a.b", "d"]>;
// { a: { c: 2 } }
*/
export type OmitDeep<T, K extends readonly PropertyKey[]> = K extends readonly [
  infer Head extends PropertyKey,
  ...infer Tail extends readonly PropertyKey[],
]
  ? OmitDeep<Head extends string ? OmitDeepPath<T, Head> : Omit<T, Head>, Tail>
  : T;
