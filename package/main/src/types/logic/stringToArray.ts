// 文字列を配列に変換する型
export type StringToArray<
  S extends string,
  T extends unknown[] = [],
> = S extends `${infer F}${infer R}` ? StringToArray<R, [...T, F]> : T;
