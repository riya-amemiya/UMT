// 1bitの2進数のORを求める型
export type Binary1bitOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitORParser<X, Y>;

export type Binary1bitORParser<
  X extends string,
  Y extends string,
> = X extends "1" ? "1" : Y extends "1" ? "1" : "0";
