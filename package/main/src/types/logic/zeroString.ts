import type { LengthOfString } from "./lengthOfString";

export type ZeroString<N extends number, A extends string = ""> =
  LengthOfString<A> extends N ? A : ZeroString<N, `${A}0`>;
