/**
 * Creates a new object with keys and values swapped.
 * If multiple keys share the same value, later entries overwrite earlier ones.
 *
 * @template K - Key type of the input object (must be a PropertyKey)
 * @template V - Value type of the input object (must be a PropertyKey)
 * @param {Record<K, V>} object - Source object
 * @returns {Record<V, K>} A new object with keys and values swapped
 * @example
 * invert({ a: 1, b: 2 }); // { 1: "a", 2: "b" }
 */
export const invert = <K extends PropertyKey, V extends PropertyKey>(
  object: Record<K, V>,
): Record<V, K> => {
  const result = {} as Record<V, K>;
  for (const key of Object.keys(object) as K[]) {
    const value = object[key];
    result[value] = key;
  }
  return result;
};
