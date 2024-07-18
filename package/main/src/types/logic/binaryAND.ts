// 2進数のANDを求める型
export type BinaryAND<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryANDParser<X, Y>;

export type BinaryANDParser<
  X extends string,
  Y extends string,
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? F extends "1"
      ? F2 extends "1"
        ? BinaryANDParser<R, R2, `${A}1`>
        : BinaryANDParser<R, R2, `${A}0`>
      : BinaryANDParser<R, R2, `${A}0`>
    : A
  : A;
