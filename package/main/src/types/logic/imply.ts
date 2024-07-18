import type { isBoolean } from "./isBoolean";
import type { NOT } from "./not";

export type IMPLY<X, Y> = NOT<X> extends true ? true : isBoolean<Y>;
