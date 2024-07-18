// 配列の先頭を取得する型
export type First<T extends unknown[]> = T extends [infer F, ...unknown[]]
  ? F
  : never;
