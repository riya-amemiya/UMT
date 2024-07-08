// 数値を配列に変換する
export type NumberToArray<
  T extends number,
  U extends number[] = [],
> = U["length"] extends T ? U : NumberToArray<T, [...U, U["length"]]>;
