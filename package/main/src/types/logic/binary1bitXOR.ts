// 1bitの2進数のXORを求める型
export type Binary1bitXOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitXORParser<X, Y>;

export type Binary1bitXORParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "0" : "1") : Y extends "1" ? "1" : "0";
