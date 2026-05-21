/**
 * Removes diacritical marks from a string by decomposing characters and
 * stripping the combining marks (e.g. "é" becomes "e").
 * @param {string} string_ - Input string
 * @returns {string} String without diacritical marks
 * @example
 * deburr("déjà vu"); // "deja vu"
 * deburr("Crème brûlée"); // "Creme brulee"
 */
export const deburr = (string_: string): string =>
  string_.normalize("NFD").replaceAll(/\p{Diacritic}/gu, "");
