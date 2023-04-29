import { int } from "./int";

export type millisecondsType = `${0 | 1 | 2 | 3 | 4 | 5}${int}` | "60";
export type secondsType = `${0 | 1 | 2 | 3 | 4 | 5}${int}` | "60";
export type minutesType = `${0 | 1 | 2 | 3 | 4 | 5}${int}` | "60";
export type hoursType = `${0 | 1}${int}` | "24";
