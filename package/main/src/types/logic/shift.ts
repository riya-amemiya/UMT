// Type to remove first element from array
export type Shift<T extends unknown[]> = T extends [unknown, ...infer R]
  ? R
  : never;
