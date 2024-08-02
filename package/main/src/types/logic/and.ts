import type { isBoolean } from "./isBoolean";

export type AND<X, Y> = isBoolean<X> extends true
  ? isBoolean<Y> extends true
    ? true
    : false
  : false;
