import type { isBoolean } from "./isBoolean";

export type XOR<X, Y> = isBoolean<X> extends true
  ? isBoolean<Y> extends true
    ? false
    : true
  : isBoolean<Y> extends true
    ? true
    : false;
