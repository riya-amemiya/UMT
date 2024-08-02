import type { ConvertMonTypeZero } from "./convertMonTypeZero";
import type { DayType } from "./dayType";
import type { DayTypeInt } from "./dayTypeInt";
import type { MonTypeInt } from "./monTypeInt";
import type { MonTypeNoZero } from "./monTypeNoZero";

export type DateType<T extends MonTypeNoZero | MonTypeInt> =
  | `${number}-${T | ConvertMonTypeZero<T>}-${T extends MonTypeNoZero
      ? DayType<T>
      : T extends MonTypeInt
        ? DayTypeInt<T>
        : never}`
  | `${number}/${T | ConvertMonTypeZero<T>}/${T extends MonTypeNoZero
      ? DayType<T>
      : T extends MonTypeInt
        ? DayTypeInt<T>
        : never}`
  | `${number}:${T | ConvertMonTypeZero<T>}:${T extends MonTypeNoZero
      ? DayType<T>
      : T extends MonTypeInt
        ? DayTypeInt<T>
        : never}`;
