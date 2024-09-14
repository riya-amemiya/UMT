// 1bitの2進数のXORを求める型
export type Binary1bitXOr<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitXorParser<X, Y>;

export type Binary1bitXorParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "0" : "1") : Y extends "1" ? "1" : "0";
