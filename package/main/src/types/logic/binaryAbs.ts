import type { BinaryComplement } from "./binaryComplement";

// 2進数の絶対値を求める型
export type BinaryAbs<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = X extends `${infer F}${infer _}`
  ? F extends "1"
    ? BinaryComplement<X>
    : X
  : never;
