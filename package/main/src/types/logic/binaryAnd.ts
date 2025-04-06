// Type to calculate AND operation on binary numbers
export type BinaryAnd<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryAndParser<X, Y>;

export type BinaryAndParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryAndParser<R, R2, `${A}1`>
        : BinaryAndParser<R, R2, `${A}0`>
      : BinaryAndParser<R, R2, `${A}0`>
    : A
  : A;
