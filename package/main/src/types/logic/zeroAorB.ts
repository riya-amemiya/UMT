import type { Equal } from "./equal";

// Type to check if either A or B is zero
export type ZeroAorB<A extends number, B extends number> = Equal<
  A,
  0
> extends true
  ? true
  : Equal<B, 0> extends true
    ? true
    : false;
