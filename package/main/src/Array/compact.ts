/**
 * Creates an array with all falsey values removed
 * @param {T[]} array The array to compact
 * @returns {T[]} Returns the new array of filtered values
 * @example
 * compact([0, 1, false, 2, '', 3]); // [1, 2, 3]
 * compact([null, undefined, NaN, 0, false, '']); // []
 */
export const compact = <T>(array: T[]): T[] => array.filter(Boolean) as T[];
