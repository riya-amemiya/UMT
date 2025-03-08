/**
 * Linear Congruential Generator for random number generation
 * @param seed Initial seed value
 * @param max Maximum value (default: 4294967296)
 * @param multiplier Multiplier parameter (default: 1664525)
 * @param increment Increment parameter (default: 1013904223)
 * @returns Random number generator function
 * @example
 * const lcg = linearCongruentialGenerator(12345);
 */
export const linearCongruentialGenerator = (
  seed: number,
  max = 4_294_967_296,
  multiplier = 1_664_525,
  increment = 1_013_904_223,
) => (multiplier * seed + increment) % max;
