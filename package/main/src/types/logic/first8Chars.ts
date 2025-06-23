// Type to get first 8 characters from string
export type First8Chars<
  S extends string,
  T extends unknown[] = [],
> = T["length"] extends 8
  ? T extends [
      infer C1,
      infer C2,
      infer C3,
      infer C4,
      infer C5,
      infer C6,
      infer C7,
      infer C8,
      ...unknown[],
    ]
    ? `${C1 & string}${C2 & string}${C3 & string}${C4 & string}${C5 &
        string}${C6 & string}${C7 & string}${C8 & string}`
    : never
  : S extends `${infer Head}${infer Rest}`
    ? First8Chars<Rest, [...T, Head]>
    : never;
