/**
 * Creates a Map from two arrays, using the first as keys and the second as values.
 * Pairs are formed at corresponding positions; iteration stops at the shorter
 * length, matching the behavior of zip. Duplicate keys keep the latest value.
 * @param keys Array of keys
 * @param values Array of values
 * @returns Map pairing keys with values at the same index
 * @example
 * zipToMap(["a", "b"], [1, 2]); // Map { "a" => 1, "b" => 2 }
 * zipToMap(["a", "b", "c"], [1, 2]); // Map { "a" => 1, "b" => 2 }
 */
export const zipToMap = <K, V>(keys: K[], values: V[]): Map<K, V> => {
  const length = Math.min(keys.length, values.length);
  const result = new Map<K, V>();
  for (let index = 0; index < length; index += 1) {
    result.set(keys[index], values[index]);
  }
  return result;
};
