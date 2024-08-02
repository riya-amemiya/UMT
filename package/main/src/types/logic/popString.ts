// 文字列の末尾を削除する型
export type PopString<S extends string> = S extends `${infer R}${string}`
  ? R
  : never;
