import type { Add } from "./add";
import type { NumberToArray } from "./numberToArray";

import type { Length } from "$/logic/length";

// Type for subtraction operation
export type Subtract<
  A extends number,
  B extends number,
> = `${A}` extends `-${infer N extends number}`
  ? `${B}` extends `-${infer M extends number}`
    ? Subtract<M, N>
    : `-${Add<N, B>}` extends `${infer R extends number}`
      ? R
      : never
  : `${B}` extends `-${infer M extends number}`
    ? Length<[...NumberToArray<A>, ...NumberToArray<M>]>
    : NumberToArray<A> extends [...NumberToArray<B>, ...infer R]
      ? Length<R>
      : NumberToArray<B> extends [...NumberToArray<A>, ...infer R]
        ? `-${Length<R>}` extends `${infer M extends number}`
          ? M
          : never
        : never;
