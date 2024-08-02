import type { Decimal4bitTobinary } from "./decimal4bitTobinary";
import type { Hex4bitToDecimal } from "./hex4bitToDecimal";
import type { LengthOfString } from "./lengthOfString";

export type HexToBinaryParser<
  X extends string,
  C extends string = "",
> = LengthOfString<X> extends 1
  ? `${C}${Decimal4bitTobinary<Hex4bitToDecimal<X>>}`
  : X extends `${infer F}${infer R}`
    ? HexToBinaryParser<R, `${C}${Decimal4bitTobinary<Hex4bitToDecimal<F>>}`>
    : C;

// 16進数を2進数に変換する型
export type HexToBinary<
  X extends `${
    | 0
    | 1
    | 2
    | 3
    | 4
    | 5
    | 6
    | 7
    | 8
    | 9
    | "A"
    | "B"
    | "C"
    | "D"
    | "E"
    | "F"}${
    | 0
    | 1
    | 2
    | 3
    | 4
    | 5
    | 6
    | 7
    | 8
    | 9
    | "A"
    | "B"
    | "C"
    | "D"
    | "E"
    | "F"}`,
> = HexToBinaryParser<X>;
