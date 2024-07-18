// 配列の長さを取得する型
export type Length<T extends unknown[]> = T extends { length: infer L }
  ? L extends number
    ? L
    : never
  : never;
