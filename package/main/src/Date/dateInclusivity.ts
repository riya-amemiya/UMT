/**
 * Inclusivity for range comparisons.
 * - "()": exclusive start and end
 * - "[]": inclusive start and end
 * - "[)": inclusive start, exclusive end
 * - "(]": exclusive start, inclusive end
 */
export type DateInclusivity = "()" | "[]" | "[)" | "(]";
