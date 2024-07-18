export type FirstNChars<
  S extends string,
  N extends number,
  C extends unknown[] = [],
  A extends string = "",
> = C["length"] extends N
  ? A
  : S extends `${infer Head}${infer Rest}`
    ? FirstNChars<Rest, N, [...C, Head], `${A}${Head}`>
    : never;
