// Type to get first element of array
export type First<T extends unknown[]> = T extends [infer F, ...unknown[]]
  ? F
  : never;
