/**
 * Creates a predicate that returns true only when all given
 * predicates return true, using short-circuit evaluation
 * @param {((...arguments_: T) => boolean)[]} predicates - The predicates to combine
 * @returns {(...arguments_: T) => boolean} A combined predicate
 * @example
 * const isPositiveEven = every(
 *   (n: number) => n > 0,
 *   (n: number) => n % 2 === 0,
 * );
 * isPositiveEven(4); // true
 * isPositiveEven(-2); // false
 */
export const every =
  <T extends unknown[]>(
    ...predicates: ((...arguments_: T) => boolean)[]
  ): ((...arguments_: T) => boolean) =>
  (...arguments_: T): boolean => {
    for (const predicate of predicates) {
      if (!predicate(...arguments_)) {
        return false;
      }
    }
    return true;
  };
