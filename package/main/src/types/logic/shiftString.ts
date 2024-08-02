// 文字列の先頭を削除する型
export type ShiftString<S extends string> = S extends `${string}${infer R}`
  ? R
  : never;
