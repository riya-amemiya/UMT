// 配列の末尾を削除する型
export type Pop<T extends unknown[]> = T extends [...infer R, unknown]
  ? R
  : never;
