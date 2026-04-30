/**
 * Returns a uniformly random element from the input array.
 * @template T - Element type
 * @param {readonly T[]} items - Source array (must be non-empty)
 * @returns {T} A randomly selected element
 * @example
 * randomChoice(["a", "b", "c"]);
 */
export const randomChoice = <T>(items: readonly T[]): T => {
  const index = Math.floor(Math.random() * items.length);
  return items[index];
};
