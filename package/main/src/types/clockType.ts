import type { doubleDigitInt, tripleDigitInt } from "./int";
import type { Int } from "./int/int";
import type { UpToFifty } from "./int/upToFifty";
import type { UpToForty } from "./int/upToForty";
import type { UpToSixty } from "./int/upToSixty";
import type { UpToThirty } from "./int/upToThirty";
import type { UpToTwenty } from "./int/upToTwenty";

export type hoursAmInt = Int | 10 | 11 | 12;
export type hoursAm = `0${Int}` | `${10 | 11 | 12}`;

export type hoursPmInt =
  | 12
  | 13
  | 14
  | 15
  | 16
  | 17
  | 18
  | 19
  | 20
  | 21
  | 22
  | 23;
export type hoursPm = `${hoursPmInt}`;

export type hoursTypeInt = hoursAmInt | hoursPmInt;
export type hoursType = hoursAm | hoursPm;

export type minutesTypeInt =
  | Int
  | UpToTwenty
  | UpToThirty
  | UpToForty
  | UpToFifty
  | UpToSixty;
export type minutesType =
  | `0${Int}`
  | `${UpToTwenty | UpToThirty | UpToForty | UpToFifty | UpToSixty}`;

export type secondsType = minutesType;
export type secondsTypeInt = minutesTypeInt;

export type millisecondsTypeInt = Int | doubleDigitInt | tripleDigitInt;

export type millisecondsType =
  | `00${Int}`
  | `0${doubleDigitInt}`
  | `${tripleDigitInt}`;
