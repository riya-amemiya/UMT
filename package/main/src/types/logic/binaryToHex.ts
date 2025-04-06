import type { BinaryToDecimalParser } from "./binaryToDecimal";
import type { Decimal4bitToHex } from "./decimal4bitToHex";
import type { LengthOfString } from "./lengthOfString";

export type BinaryToHexParser<
  X extends string,
  C extends string = "",
  A extends string = "",
> = X extends `${infer F}${infer R}`
  ? LengthOfString<`${F}${C}`> extends 4
    ? BinaryToHexParser<
        R,
        "",
        `${A}${Decimal4bitToHex<`${BinaryToDecimalParser<`${F}${C}`>}`>}`
      >
    : BinaryToHexParser<R, `${F}${C}`, A>
  : A;

// Type to convert 1-byte binary to hexadecimal
export type BinaryToHex<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryToHexParser<X>;
