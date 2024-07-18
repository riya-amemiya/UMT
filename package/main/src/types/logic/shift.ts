// 配列の先頭を削除する型
export type Shift<T extends unknown[]> = T extends [unknown, ...infer R]
  ? R
  : never;
