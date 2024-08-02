import type { BinaryAdd } from "./binaryAdd";
import type { First8Chars } from "./first8Chars";

export type BinaryComplementParser<X extends string> =
  X extends `${infer L}${infer R}`
    ? L extends "0"
      ? `1${BinaryComplementParser<R>}`
      : `0${BinaryComplementParser<R>}`
    : "";

// 1バイトの2の補数を求める型
export type BinaryComplement<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = First8Chars<BinaryAdd<BinaryComplementParser<X>, "00000001">>;
