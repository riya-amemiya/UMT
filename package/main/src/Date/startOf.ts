export type DateBoundaryUnit =
  | "second"
  | "minute"
  | "hour"
  | "day"
  | "week"
  | "month"
  | "quarter"
  | "year";

/**
 * Returns a new Date set to the start of the given unit.
 * Week start is Sunday (day 0).
 *
 * @param {Date} date - Base date
 * @param {DateBoundaryUnit} unit - Boundary unit
 * @returns {Date} A new Date at the start of the specified unit
 * @example
 * startOf(new Date("2025-04-15T10:30:00"), "day"); // 2025-04-15T00:00:00
 * startOf(new Date("2025-04-15"), "month"); // 2025-04-01T00:00:00
 */
export const startOf = (date: Date, unit: DateBoundaryUnit): Date => {
  const result = new Date(date);
  switch (unit) {
    case "second": {
      result.setMilliseconds(0);
      return result;
    }
    case "minute": {
      result.setSeconds(0, 0);
      return result;
    }
    case "hour": {
      result.setMinutes(0, 0, 0);
      return result;
    }
    case "day": {
      result.setHours(0, 0, 0, 0);
      return result;
    }
    case "week": {
      const dayOfWeek = result.getDay();
      result.setDate(result.getDate() - dayOfWeek);
      result.setHours(0, 0, 0, 0);
      return result;
    }
    case "month": {
      result.setDate(1);
      result.setHours(0, 0, 0, 0);
      return result;
    }
    case "quarter": {
      const quarterStartMonth = Math.floor(result.getMonth() / 3) * 3;
      result.setMonth(quarterStartMonth, 1);
      result.setHours(0, 0, 0, 0);
      return result;
    }
    case "year": {
      result.setMonth(0, 1);
      result.setHours(0, 0, 0, 0);
      return result;
    }
    default: {
      return result;
    }
  }
};
