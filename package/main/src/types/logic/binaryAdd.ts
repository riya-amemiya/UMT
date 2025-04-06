import type { First8Chars } from "./first8Chars";
import type { StringReverse } from "./stringReverse";

// Type for binary addition (up to 1 byte)
export type BinaryAddParser<
  A extends string,
  B extends string,
  C extends "0" | "1" = "0",
> = [A, B] extends [`${infer A1}${infer A2}`, `${infer B1}${infer B2}`]
  ? A1 extends "0"
    ? B1 extends "0"
      ? C extends "0"
        ? `${BinaryAddParser<A2, B2, "0">}0`
        : `${BinaryAddParser<A2, B2, "0">}1`
      : C extends "0"
        ? `${BinaryAddParser<A2, B2, "0">}1`
        : `${BinaryAddParser<A2, B2, "1">}0`
    : B1 extends "0"
      ? C extends "0"
        ? `${BinaryAddParser<A2, B2, "0">}1`
        : `${BinaryAddParser<A2, B2, "1">}0`
      : C extends "0"
        ? `${BinaryAddParser<A2, B2, "1">}0`
        : `${BinaryAddParser<A2, B2, "1">}1`
  : `${C}`;

export type BinaryAdd<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = StringReverse<
  First8Chars<
    StringReverse<BinaryAddParser<StringReverse<X>, StringReverse<Y>>>
  >
>;
