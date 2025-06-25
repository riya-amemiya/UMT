/**
 * Determine if a given year is a leap year
 * @param {number} year - The year to check
 * @returns {boolean} true if the year is a leap year, false otherwise
 * @example
 * isLeapYear(2000); // Returns true (divisible by 400)
 * isLeapYear(2020); // Returns true (divisible by 4 but not 100)
 * isLeapYear(2100); // Returns false (divisible by 100 but not 400)
 * isLeapYear(2023); // Returns false (not divisible by 4)
 */
export const isLeapYear = (year: number): boolean => {
  if (!Number.isInteger(year)) {
    return false;
  }

  return (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
};
