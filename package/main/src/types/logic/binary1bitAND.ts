// 1bitの2進数のANDを求める型
export type Binary1bitAND<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitANDParser<X, Y>;

export type Binary1bitANDParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "1" : "0") : "0";
