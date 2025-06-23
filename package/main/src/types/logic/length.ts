// Type to get array length
export type Length<T extends unknown[]> = T extends { length: infer L }
  ? L extends number
    ? L
    : never
  : never;
