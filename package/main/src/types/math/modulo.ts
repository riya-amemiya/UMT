import type { Subtract } from "./subtract";

import type { BGreaterThanA } from "$/logic/bGreaterThanA";

// Type for modulo operation
export type Modulo<
  A extends number,
  B extends number,
> = `${A}` extends `-${infer N extends number}`
  ? `${B}` extends `-${infer M extends number}`
    ? Modulo<N, M>
    : Modulo<N, B>
  : `${B}` extends `-${infer M extends number}`
    ? Modulo<A, M>
    : BGreaterThanA<A, B> extends true
      ? A
      : Modulo<Subtract<A, B>, B>;
