import type { StringToArray } from "./stringToArray";

// Type to get string length by converting to array
export type LengthOfString<S extends string> = StringToArray<S>["length"];
