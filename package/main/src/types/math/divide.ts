import type { Add } from "./add";
import type { Subtract } from "./subtract";

import type { BGreaterThanA } from "$/logicType";

// 割り算
export type Divide<A extends number, B extends number> = A extends 0
  ? never
  : B extends 0
    ? never
    : `${A}` extends `-${infer N extends number}`
      ? `${B}` extends `-${infer M extends number}`
        ? DivideHelper<N, M>
        : `-${DivideHelper<N, B>}` extends `${infer R extends number}`
          ? R
          : never
      : `${B}` extends `-${infer M extends number}`
        ? `-${DivideHelper<A, M>}` extends `${infer R extends number}`
          ? R
          : never
        : DivideHelper<A, B>;

// 割り算のヘルパー
type DivideHelper<
  X extends number,
  Y extends number,
  A extends number = 0,
> = BGreaterThanA<X, Y> extends true
  ? A
  : DivideHelper<Subtract<X, Y>, Y, Add<A, 1>>;
