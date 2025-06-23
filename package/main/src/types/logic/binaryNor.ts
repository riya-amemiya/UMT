// Type to calculate NOT OR (NOR) operation on binary numbers
export type BinaryNor<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryNorParser<X, Y>;

export type BinaryNorParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? BinaryNorParser<R, R2, `${A}0`>
      : F2 extends "1"
        ? BinaryNorParser<R, R2, `${A}0`>
        : BinaryNorParser<R, R2, `${A}1`>
    : A
  : A;
