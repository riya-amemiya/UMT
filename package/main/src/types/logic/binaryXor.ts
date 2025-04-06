// Type to calculate XOR operation on binary numbers
export type BinaryXor<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryXorParser<X, Y>;

export type BinaryXorParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryXorParser<R, R2, `${A}0`>
        : BinaryXorParser<R, R2, `${A}1`>
      : F2 extends "1"
        ? BinaryXorParser<R, R2, `${A}1`>
        : BinaryXorParser<R, R2, `${A}0`>
    : A
  : A;
