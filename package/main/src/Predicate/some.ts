/**
 * Creates a predicate that returns true when at least one of
 * the given predicates returns true, using short-circuit evaluation
 * @param {((...arguments_: T) => boolean)[]} predicates - The predicates to combine
 * @returns {(...arguments_: T) => boolean} A combined predicate
 * @example
 * const isZeroOrNegative = some(
 *   (n: number) => n === 0,
 *   (n: number) => n < 0,
 * );
 * isZeroOrNegative(0); // true
 * isZeroOrNegative(-1); // true
 * isZeroOrNegative(1); // false
 */
export const some =
  <T extends unknown[]>(
    ...predicates: ((...arguments_: T) => boolean)[]
  ): ((...arguments_: T) => boolean) =>
  (...arguments_: T): boolean => {
    for (const predicate of predicates) {
      if (predicate(...arguments_)) {
        return true;
      }
    }
    return false;
  };
