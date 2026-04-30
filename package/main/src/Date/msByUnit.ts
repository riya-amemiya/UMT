import {
  OneDayMs,
  OneHourMs,
  OneMinuteMs,
  OneSecondMs,
  OneWeekMs,
} from "../Consts/clock";

import type { DurationUnit } from "./durationUnit";

export const msByUnit: Partial<Record<DurationUnit, number>> = {
  ms: 1,
  s: OneSecondMs,
  m: OneMinuteMs,
  h: OneHourMs,
  d: OneDayMs,
  w: OneWeekMs,
};
