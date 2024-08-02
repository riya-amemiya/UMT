// floot型かどうかを判定する型
export type IsFloat<T> = T extends number
  ? `${T}` extends `${infer _}.${infer _2}`
    ? true
    : false
  : false;
