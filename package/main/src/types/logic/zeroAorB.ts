import type { Equal } from "./equal";

// AまたはBが0かどうか
export type ZeroAorB<A extends number, B extends number> = Equal<
  A,
  0
> extends true
  ? true
  : Equal<B, 0> extends true
    ? true
    : false;
