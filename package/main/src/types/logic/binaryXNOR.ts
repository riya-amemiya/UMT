// 2進数のNot XORを求める型
export type BinaryXNOR<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryXNORParser<X, Y>;

export type BinaryXNORParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryXNORParser<R, R2, `${A}1`>
        : BinaryXNORParser<R, R2, `${A}0`>
      : F2 extends "1"
        ? BinaryXNORParser<R, R2, `${A}0`>
        : BinaryXNORParser<R, R2, `${A}1`>
    : A
  : A;
