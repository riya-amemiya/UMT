import type { BinaryComplement } from "./binaryComplement";

// Type to calculate absolute value of a binary number
export type BinaryAbs<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = X extends `${infer F}${infer _}`
  ? F extends "1"
    ? BinaryComplement<X>
    : X
  : never;
