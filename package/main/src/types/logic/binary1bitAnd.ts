// Type to calculate AND operation on 1-bit binary numbers
export type Binary1bitAnd<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitAndParser<X, Y>;

export type Binary1bitAndParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "1" : "0") : "0";
