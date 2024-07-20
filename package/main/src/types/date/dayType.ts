import type { MonthsWithout31Days } from "./monthsWithout31Days";

export type DayType<T extends string> = T extends "02" | "2"
  ?
      | `${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${0}${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${1}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${2}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
  :
      | `${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${0}${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${1}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${2}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
      | `${3}${T extends MonthsWithout31Days ? 0 : 0 | 1}`;
