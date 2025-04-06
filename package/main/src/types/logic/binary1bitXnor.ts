// Type to calculate NOT XOR (XNOR) of 1-bit binary numbers
export type Binary1bitXnor<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitXnorParser<X, Y>;

export type Binary1bitXnorParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "1" : "0") : Y extends "1" ? "0" : "1";
