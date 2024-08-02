// 配列を反転する型
export type ArrayReverse<
  S extends unknown[],
  T extends unknown[] = [],
> = S extends [infer F, ...infer R] ? ArrayReverse<R, [F, ...T]> : T;
