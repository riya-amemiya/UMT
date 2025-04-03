import { bitwise } from "./bitwise";

/**
 * Generates random numbers using the Xoshiro256** algorithm
 * @param {[number, number, number, number]} state Array of four 32-bit state values
 * @param {number} [min=0] Minimum value of the generated random number (inclusive)
 * @param {number} [max=1] Maximum value of the generated random number (exclusive)
 * @returns {number} Generated random number between min and max
 * @description
 * Xoshiro256** is a fast, high-quality pseudorandom number generator.
 * This implementation modifies the internal state array in place.
 */
export const xoshiro256 = (
  state: [number, number, number, number],
  min = 0,
  max = 1,
): number => {
  const sum = (state[0] >>> 0) + (state[3] >>> 0);
  const result = bitwise(sum, 23) + (state[0] >>> 0);

  const t = state[1] << 17;

  state[2] ^= state[0];
  state[3] ^= state[1];
  state[1] ^= state[2];
  state[0] ^= state[3];

  state[2] ^= t;
  state[3] = bitwise(state[3], 45);

  return min + ((result >>> 0) / 2 ** 32) * (max - min);
};
