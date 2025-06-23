import type { Equal } from "./equal";
import type { ZeroAorB } from "./zeroAorB";

import type { Subtract } from "$/math/subtract";

// Type to compare if B is greater than A
export type BGreaterThanA<
  A extends number,
  B extends number,
> = `${A}` extends `-${infer N extends number}`
  ? `${B}` extends `-${infer M extends number}`
    ? BGreaterThanA<N, M>
    : BGreaterThanA<N, B>
  : `${B}` extends `-${infer M extends number}`
    ? BGreaterThanA<A, M>
    : ZeroAorB<A, B> extends true
      ? Equal<A, B> extends true
        ? false
        : A extends 0
          ? true
          : false
      : BGreaterThanA<Subtract<A, 1>, Subtract<B, 1>>;
