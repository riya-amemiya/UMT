/**
@internal
Collects the first value of each distinct key from the given sources, in encounter order.
Keys are compared with SameValueZero, so `NaN` matches `NaN` and `0` matches `-0`.
The first value of each key is returned unchanged (a leading `-0` stays `-0`).
@param sources Iterables scanned one after another
@param keyOf Maps a value to its identity key; defaults to the value itself
@param onDuplicate Called with every value whose key was already seen
@returns The first-seen value of each key
*/
export const orderedUnique = <T, K = T>(
  sources: readonly Iterable<T>[],
  keyOf?: (value: T) => K,
  onDuplicate?: (value: T) => void,
): T[] => {
  const seen = new Set<unknown>();
  const result: T[] = [];
  for (const source of sources) {
    for (const value of source) {
      const key = keyOf === undefined ? value : keyOf(value);
      if (seen.has(key)) {
        onDuplicate?.(value);
        continue;
      }
      seen.add(key);
      result.push(value);
    }
  }
  return result;
};
