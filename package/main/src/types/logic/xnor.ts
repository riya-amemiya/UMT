import type { NOT } from "./not";
import type { XOR } from "./xor";

export type XNOR<X, Y> = NOT<XOR<X, Y>>;
