import type { Subtract } from "$/math/subtract";

// Type for array/string slice operation
export type Slice<
  S extends string,
  Start extends number,
  End extends number,
  A extends string = "",
> = S extends `${infer F}${infer R}`
  ? Start extends 0
    ? End extends 0
      ? A
      : Slice<R, 0, Subtract<End, 1>, `${A}${F}`>
    : Slice<R, Subtract<Start, 1>, Subtract<End, 1>, A>
  : A;
