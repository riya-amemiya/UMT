// 2進数のXORを求める型
export type BinaryXOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryXORParser<X, Y>;

export type BinaryXORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryXORParser<R, R2, `${A}0`>
        : BinaryXORParser<R, R2, `${A}1`>
      : F2 extends "1"
        ? BinaryXORParser<R, R2, `${A}1`>
        : BinaryXORParser<R, R2, `${A}0`>
    : A
  : A;
