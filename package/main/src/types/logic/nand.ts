import type { AND } from "./and";
import type { NOT } from "./not";

export type NAND<X, Y> = NOT<AND<X, Y>>;
