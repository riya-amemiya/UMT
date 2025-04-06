// Type to convert number to array
export type NumberToArray<
  T extends number,
  U extends number[] = [],
> = U["length"] extends T ? U : NumberToArray<T, [...U, U["length"]]>;
