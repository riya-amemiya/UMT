/**
 * Creates a predicate that negates the given predicate
 * @param {(...arguments_: T) => boolean} function_ - The predicate to negate
 * @returns {(...arguments_: T) => boolean} A new predicate that returns the opposite
 * @example
 * const isEven = (n: number) => n % 2 === 0;
 * const isOdd = not(isEven);
 * isOdd(3); // true
 */
export const not =
  <T extends unknown[]>(
    function_: (...arguments_: T) => boolean,
  ): ((...arguments_: T) => boolean) =>
  (...arguments_: T): boolean =>
    !function_(...arguments_);
