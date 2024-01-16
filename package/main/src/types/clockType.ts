import type {
  doubleDigitInt,
  int,
  tripleDigitInt,
  upToFifty,
  upToForty,
  upToSixty,
  upToThirty,
  upToTwenty,
} from "./int";

export type hoursAmInt = int | 10 | 11 | 12;
export type hoursAm = `0${int}` | `${10 | 11 | 12}`;

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
  | int
  | upToTwenty
  | upToThirty
  | upToForty
  | upToFifty
  | upToSixty;
export type minutesType =
  | `0${int}`
  | `${upToTwenty | upToThirty | upToForty | upToFifty | upToSixty}`;

export type secondsType = minutesType;
export type secondsTypeInt = minutesTypeInt;

export type millisecondsTypeInt = int | doubleDigitInt | tripleDigitInt;

export type millisecondsType =
  | `00${int}`
  | `0${doubleDigitInt}`
  | `${tripleDigitInt}`;
