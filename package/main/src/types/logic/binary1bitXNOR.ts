// 1bitの2進数のNot XORを求める型
export type Binary1bitXNOR<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = Binary1bitXNORParser<X, Y>;

export type Binary1bitXNORParser<
  X extends string,
  Y extends string,
> = X extends "1" ? (Y extends "1" ? "1" : "0") : Y extends "1" ? "0" : "1";
