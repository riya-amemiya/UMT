// Type to calculate NOT OR (NOR) of 1-bit binary numbers
export type Binary1bitNor<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitNorParser<X, Y>;

type Binary1bitNorParser<X extends string, Y extends string> = X extends "1"
  ? "0"
  : Y extends "1"
    ? "0"
    : "1";
