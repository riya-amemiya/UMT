// Type to calculate NOT AND (NAND) operation on binary numbers
export type BinaryNand<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryNandParser<X, Y>;

export type BinaryNandParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryNandParser<R, R2, `${A}0`>
        : BinaryNandParser<R, R2, `${A}1`>
      : BinaryNandParser<R, R2, `${A}1`>
    : A
  : A;
