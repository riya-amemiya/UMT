/**
 * Counts the number of non-overlapping occurrences of a substring within a
 * string. Returns 0 when the search string is empty.
 * @param {string} string_ - Input string
 * @param {string} search - Substring to count
 * @returns {number} Number of occurrences
 * @example
 * countOccurrences("ababab", "ab"); // 3
 * countOccurrences("aaaa", "aa"); // 2
 */
export const countOccurrences = (string_: string, search: string): number => {
  if (search === "") {
    return 0;
  }
  let count = 0;
  let index = string_.indexOf(search);
  while (index !== -1) {
    count++;
    index = string_.indexOf(search, index + search.length);
  }
  return count;
};
