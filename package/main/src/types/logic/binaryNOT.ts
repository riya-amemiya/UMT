// 2進数のNOTを求める型
export type BinaryNOT<X extends `${0 | 1}`> = BinaryNOTParser<X>;

export type BinaryNOTParser<X extends string> = X extends "1" ? "0" : "1";
