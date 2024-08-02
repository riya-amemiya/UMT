import type { isBoolean } from "./isBoolean";

export type NOT<X> = isBoolean<X> extends true ? false : true;
