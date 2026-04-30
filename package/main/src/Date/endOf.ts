import type { DateBoundaryUnit } from "./startOf";

/**
 * Returns a new Date set to the end of the given unit.
 * Week ends on Saturday at 23:59:59.999.
 *
 * @param {Date} date - Base date
 * @param {DateBoundaryUnit} unit - Boundary unit
 * @returns {Date} A new Date at the end of the specified unit
 * @example
 * endOf(new Date("2025-04-15T10:30:00"), "day"); // 2025-04-15T23:59:59.999
 * endOf(new Date("2025-04-15"), "month"); // 2025-04-30T23:59:59.999
 */
export const endOf = (date: Date, unit: DateBoundaryUnit): Date => {
  const result = new Date(date);
  switch (unit) {
    case "second": {
      result.setMilliseconds(999);
      return result;
    }
    case "minute": {
      result.setSeconds(59, 999);
      return result;
    }
    case "hour": {
      result.setMinutes(59, 59, 999);
      return result;
    }
    case "day": {
      result.setHours(23, 59, 59, 999);
      return result;
    }
    case "week": {
      const dayOfWeek = result.getDay();
      result.setDate(result.getDate() + (6 - dayOfWeek));
      result.setHours(23, 59, 59, 999);
      return result;
    }
    case "month": {
      result.setMonth(result.getMonth() + 1, 0);
      result.setHours(23, 59, 59, 999);
      return result;
    }
    case "quarter": {
      const quarterEndMonth = Math.floor(result.getMonth() / 3) * 3 + 2;
      result.setMonth(quarterEndMonth + 1, 0);
      result.setHours(23, 59, 59, 999);
      return result;
    }
    case "year": {
      result.setMonth(11, 31);
      result.setHours(23, 59, 59, 999);
      return result;
    }
    default: {
      return result;
    }
  }
};
