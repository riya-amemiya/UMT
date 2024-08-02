// 2進数のORを求める型
export type BinaryOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryORParser<X, Y>;

export type BinaryORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? BinaryORParser<R, R2, `${A}1`>
      : F2 extends "1"
        ? BinaryORParser<R, R2, `${A}1`>
        : BinaryORParser<R, R2, `${A}0`>
    : A
  : A;
