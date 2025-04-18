import type { Binary1bitOrParser } from "./binary1bitOr";
import type { BinaryHalfAdderParser } from "./binaryHalfAdder";
import type { FirstNChars } from "./firstNChars";
import type { LengthOfString } from "./lengthOfString";
import type { ShiftString } from "./shiftString";
import type { StringReverse } from "./stringReverse";

// Type for full adder binary operation
export type BinaryFullAdder<
  X extends string,
  Y extends string,
  B extends number = LengthOfString<X>,
> = LengthOfString<X> extends LengthOfString<Y>
  ? FirstNChars<
      ShiftString<
        StringReverse<
          BinaryFullAdderParser<
            StringReverse<FirstNChars<X, B>>,
            StringReverse<FirstNChars<Y, B>>
          >
        >
      >,
      B
    >
  : never;

// Type for multi-byte adder operation
type BinaryFullAdderParser<
  X extends string,
  Y extends string,
  A extends string = "",
  C extends string = "0",
> = X extends `${infer F}${infer R}`
  ? Y extends `${infer F2}${infer R2}`
    ? BinaryHalfAdderParser<F, F2> extends `${infer F3}${infer R3}`
      ? BinaryHalfAdderParser<R3, C> extends `${infer F4}${infer R4}`
        ? BinaryFullAdderParser<R, R2, `${A}${R4}`, Binary1bitOrParser<F3, F4>>
        : never
      : never
    : never
  : `${A}${C}`;
