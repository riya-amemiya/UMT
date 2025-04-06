// Type to calculate NOT AND (NAND) operation on 1-bit binary numbers
export type Binary1bitNand<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitNandParser<X, Y>;

export type Binary1bitNandParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "0" : "1") : "1";
