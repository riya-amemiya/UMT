import type { Add } from "./add";
import type { Subtract } from "./subtract";

// Type for multiplication operation
export type Multiply<A extends number, B extends number> = MultiHelper<A, B>;

// Helper type for multiplication operation
type MultiHelper<
  X extends number,
  Y extends number,
  A extends number = 0,
> = `${X}` extends `-${infer N extends number}`
  ? `${Y}` extends `-${infer M extends number}`
    ? MultiHelper<N, M, A>
    : `-${MultiHelper<N, Y, A>}` extends `${infer R extends number}`
      ? R
      : never
  : `${Y}` extends `-${infer M extends number}`
    ? `-${MultiHelper<X, M, A>}` extends `${infer R extends number}`
      ? R
      : never
    : Y extends 0
      ? A
      : MultiHelper<X, Subtract<Y, 1>, Add<X, A>>;
