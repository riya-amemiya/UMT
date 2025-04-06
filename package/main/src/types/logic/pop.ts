// Type to remove last element from array
export type Pop<T extends unknown[]> = T extends [...infer R, unknown]
  ? R
  : never;
