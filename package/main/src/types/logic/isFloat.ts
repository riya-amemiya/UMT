// Type to check if a number is a float
export type IsFloat<T> = T extends number
  ? `${T}` extends `${infer _}.${infer _2}`
    ? true
    : false
  : false;
