import { OneDayMs } from "@/Consts/clock";

/**
 * Generate an array containing all dates between the specified start and end dates.
 *
 * @param {Date} startDate - The start date of the range
 * @param {Date} endDate - The end date of the range
 * @returns {Date[]} An array of Date objects from startDate to endDate (inclusive)
 * @example
 * dateRange(new Date('2025-01-01'), new Date('2025-01-03'))
 * // Returns array of dates: [2025-01-01, 2025-01-02, 2025-01-03]
 */
export const dateRange = (startDate: Date, endDate: Date): Date[] => {
  const dates: Date[] = [];
  let currentDate = startDate;
  while (currentDate <= endDate) {
    dates.push(new Date(currentDate));
    currentDate = new Date(currentDate.getTime() + OneDayMs);
  }
  return dates;
};
