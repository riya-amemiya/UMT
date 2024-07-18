import type { NOT } from "./not";
import type { OR } from "./or";

export type NOR<X, Y> = NOT<OR<X, Y>>;
