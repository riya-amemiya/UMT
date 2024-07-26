import type { DoubleDigitInt } from "$/int/doubleDigitInt";
import type { Int } from "$/int/int";
import type { TripleDigitInt } from "$/int/tripleDigitInt";

export type MillisecondsType =
  | `00${Int}`
  | `0${DoubleDigitInt}`
  | `${TripleDigitInt}`;
