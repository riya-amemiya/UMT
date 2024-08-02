// 2進数のNot ANDを求める型
export type BinaryNAND<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryNANDParser<X, Y>;

export type BinaryNANDParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryNANDParser<R, R2, `${A}0`>
        : BinaryNANDParser<R, R2, `${A}1`>
      : BinaryNANDParser<R, R2, `${A}1`>
    : A
  : A;
