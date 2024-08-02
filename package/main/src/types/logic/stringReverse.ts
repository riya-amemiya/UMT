// 文字列を反転する型
export type StringReverse<S extends string> = S extends `${infer F}${infer R}`
  ? `${StringReverse<R>}${F}`
  : "";
