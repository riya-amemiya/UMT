import type { IntEven } from "./intEven";
import type { IntOdd } from "./intOdd";

export type IntWithoutZero = Exclude<IntEven | IntOdd, 0>;
