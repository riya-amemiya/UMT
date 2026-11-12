/**
Returns the number of days in the local-time month of the given date.
Leap years follow the Gregorian calendar (February has 29 days).

@param {Date} date - Any date within the month
@returns {number} Number of days in that month (28-31), or NaN for an invalid date
@example
daysInMonth(new Date(2025, 0, 15)); // 31
daysInMonth(new Date(2024, 1, 1)); // 29
daysInMonth(new Date(2025, 1, 1)); // 28
*/
export const daysInMonth = (date: Date): number => {
  const lastDay = new Date(date);
  lastDay.setMonth(date.getMonth() + 1, 0);
  return lastDay.getDate();
};
