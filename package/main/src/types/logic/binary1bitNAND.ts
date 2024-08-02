// 1bitの2進数のNot ANDを求める型
export type Binary1bitNAND<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitNANDParser<X, Y>;

export type Binary1bitNANDParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "0" : "1") : "1";
