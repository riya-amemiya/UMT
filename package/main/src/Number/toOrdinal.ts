/**
 * Converts a number to its English ordinal string representation.
 *
 * Handles the special cases for 11th, 12th, and 13th
 * (which end in "th" despite their last digit).
 *
 * @param value - The number to convert to ordinal form
 * @returns The ordinal string (e.g., "1st", "2nd", "3rd", "11th")
 *
 * @example
 * ```typescript
 * toOrdinal(1);   // "1st"
 * toOrdinal(2);   // "2nd"
 * toOrdinal(3);   // "3rd"
 * toOrdinal(11);  // "11th"
 * toOrdinal(21);  // "21st"
 * toOrdinal(112); // "112th"
 * ```
 */
export const toOrdinal = (value: number): string => {
  const module100 = value % 100;
  if (module100 >= 11 && module100 <= 13) {
    return `${value}th`;
  }

  const module10 = value % 10;
  if (module10 === 1) {
    return `${value}st`;
  }
  if (module10 === 2) {
    return `${value}nd`;
  }
  if (module10 === 3) {
    return `${value}rd`;
  }

  return `${value}th`;
};
