import type { Binary1bitAndParser } from "./binary1bitAnd";
import type { Binary1bitXorParser } from "./binary1bitXor";

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
  ? BinaryHalfAdderParser<X, Y, Binary1bitAndParser<X, Y>>
  : `${C}${Binary1bitXorParser<X, Y>}`;
