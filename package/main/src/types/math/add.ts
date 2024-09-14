import type { NumberToArray } from "./numberToArray";
import type { Subtract } from "./subtract";

import type { Length } from "$/logic/length";

// たし算
export type Add<
  X extends number,
  Y extends number,
> = `${X}` extends `-${infer N extends number}`
  ? `${Y}` extends `-${infer M extends number}`
    ? `-${Add<N, M>}` extends `${infer R extends number}`
      ? R
      : never
    : Subtract<Y, N>
  : `${Y}` extends `-${infer M extends number}`
    ? Subtract<X, M>
    : Length<[...NumberToArray<X>, ...NumberToArray<Y>]>;
