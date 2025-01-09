/**
 * Evaluates true strict equality
 * @param {unknown} a
 * @param {unknown} b
 * @returns boolean
 *
 * @example
 * isEqual(1, 1); // true
 * isEqual("test", "test"); // true
 * isEqual(Number.NaN, Number.NaN); // true
 * isEqual(-0, +0); // false
 *
 * @example
 * const obj = { a: 1 };
 * isEqual(obj, obj); // true
 * isEqual(obj, { a: 1 }); // false
 *
 * @example
 * const arr = [1, 2, 3];
 * isEqual(arr, arr); // true
 * isEqual(arr, [1, 2, 3]); // false
 */
export const isEqual = (a: unknown, b: unknown): boolean => Object.is(a, b);
