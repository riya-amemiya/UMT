import type { BinaryAdd } from "./binaryAdd";
import type { First8Chars } from "./first8Chars";

export type BinaryComplementParser<X extends string> =
  X extends `${infer L}${infer R}`
    ? L extends "0"
      ? `1${BinaryComplementParser<R>}`
      : `0${BinaryComplementParser<R>}`
    : "";

// Type to calculate two's complement of a 1-byte binary number
export type BinaryComplement<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = First8Chars<BinaryAdd<BinaryComplementParser<X>, "00000001">>;
