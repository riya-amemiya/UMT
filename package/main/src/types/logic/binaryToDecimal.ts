import type { Length } from "./length";
import type { LengthOfString } from "./lengthOfString";
import type { Shift } from "./shift";
import type { StringReverse } from "./stringReverse";

export type BinaryToDecimalParser<
  X extends string,
  C extends unknown[] = [""],
  A extends unknown[] = [""],
  FL extends boolean = false,
> = X extends `${infer F}${infer R}`
  ? LengthOfString<X> extends 8
    ? F extends "1"
      ? `-${BinaryToDecimalParser<StringReverse<R>, C, A, true>}`
      : BinaryToDecimalParser<StringReverse<R>, C, A, FL>
    : FL extends false
      ? F extends "1"
        ? BinaryToDecimalParser<R, [...C, ...C], [...A, ...C], FL>
        : BinaryToDecimalParser<R, [...C, ...C], A, FL>
      : F extends "1"
        ? BinaryToDecimalParser<R, [...C, ...C], A, FL>
        : BinaryToDecimalParser<R, [...C, ...C], [...A, ...C], FL>
  : FL extends true
    ? Length<A>
    : Length<Shift<A>>;

// Type to convert binary to decimal
export type BinaryToDecimal<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = BinaryToDecimalParser<X>;
