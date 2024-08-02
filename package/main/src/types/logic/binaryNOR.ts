// 2進数のNot ORを求める型
export type BinaryNOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryNORParser<X, Y>;

export type BinaryNORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? BinaryNORParser<R, R2, `${A}0`>
      : F2 extends "1"
        ? BinaryNORParser<R, R2, `${A}0`>
        : BinaryNORParser<R, R2, `${A}1`>
    : A
  : A;
