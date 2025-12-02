import type { isBoolean } from "./isBoolean";

export type OR<X, Y> =
  isBoolean<X> extends true ? true : isBoolean<Y> extends true ? true : false;
