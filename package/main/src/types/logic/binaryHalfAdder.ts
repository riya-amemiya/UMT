import type { Binary1bitANDParser } from "./binary1bitAND";
import type { Binary1bitXORParser } from "./binary1bitXOR";

// 半加算器
export type BinaryHalfAdder<
  X extends `${0 | 1}`,
  Y extends `${0 | 1}`,
> = BinaryHalfAdderParser<X, Y>;

export type BinaryHalfAdderParser<
  X extends string,
  Y extends string,
  C extends string = "",
> = C extends ""
  ? BinaryHalfAdderParser<X, Y, Binary1bitANDParser<X, Y>>
  : `${C}${Binary1bitXORParser<X, Y>}`;
