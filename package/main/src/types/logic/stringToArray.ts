// Type to convert string to array
export type StringToArray<
  S extends string,
  T extends unknown[] = [],
> = S extends `${infer F}${infer R}` ? StringToArray<R, [...T, F]> : T;
