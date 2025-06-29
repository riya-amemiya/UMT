/**
 * Creates an object without the specified keys
 * @param object - The source object
 * @param keys - The keys to omit
 * @returns A new object without the specified keys
 */
export const omit = <T extends object, K extends keyof T>(
  object: T,
  ...keys: K[]
): Omit<T, K> => {
  const result = { ...object };

  for (const key of keys) {
    delete result[key];
  }

  return result;
};
