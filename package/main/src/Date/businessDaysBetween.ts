import { OneDayMs } from "@/Consts/clock";
import { isBusinessDay } from "@/Date/isBusinessDay";
import { startOf } from "@/Date/startOf";

/**
Counts business days from `start` to `end`, using the same definition as
`isBusinessDay` (weekdays that are not listed holidays).
Days are compared by local calendar date; the time of day is ignored.
The start day is excluded and the end day is included, so the result is the
inverse of `addBusinessDays`: when `end` is a business day,
`addBusinessDays(start, businessDaysBetween(start, end, holidays), holidays)`
lands on the calendar day of `end`.
When `end` is before `start`, the result is negative (the start day is still
excluded and the end day is still included).

@param {Date} start - Start date (excluded)
@param {Date} end - End date (included)
@param {Date[]} [holidays=[]] - Optional holiday dates
@returns {number} Signed number of business days between the dates
@example
businessDaysBetween(new Date(2025, 3, 18), new Date(2025, 3, 21)); // 1 (Fri -> Mon)
businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 3, 25)); // 4 (Mon -> Fri)
businessDaysBetween(new Date(2025, 3, 21), new Date(2025, 3, 18)); // -1
*/
export const businessDaysBetween = (
  start: Date,
  end: Date,
  holidays: Date[] = [],
): number => {
  const from = startOf(start, "day");
  const to = startOf(end, "day");
  const days = Math.round((to.getTime() - from.getTime()) / OneDayMs);
  const step = days < 0 ? -1 : 1;
  const count = Array.from({ length: Math.abs(days) }, (_, index) => {
    const day = new Date(from);
    day.setDate(from.getDate() + step * (index + 1));
    return day;
  }).filter((day) => isBusinessDay(day, holidays)).length;
  return step * count + 0;
};
