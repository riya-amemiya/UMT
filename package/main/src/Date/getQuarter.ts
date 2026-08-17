/**
 * Returns the calendar quarter (1-4) for the date's local month.
 * Matches startOf(..., "quarter"): Jan-Mar=1, Apr-Jun=2, Jul-Sep=3,
 * Oct-Dec=4.
 *
 * @param {Date} date - Date to inspect
 * @returns {1 | 2 | 3 | 4} Quarter number
 * @example
 * getQuarter(new Date(2025, 3, 15)); // 2
 * getQuarter(new Date(2025, 0, 1)); // 1
 */
export const getQuarter = (date: Date): 1 | 2 | 3 | 4 =>
  (Math.floor(date.getMonth() / 3) + 1) as 1 | 2 | 3 | 4;
