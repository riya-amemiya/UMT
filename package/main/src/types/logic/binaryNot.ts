// 2進数のNOTを求める型
export type BinaryNot<X extends `${0 | 1}`> = BinaryNotParser<X>;

export type BinaryNotParser<X extends string> = X extends "1" ? "0" : "1";
