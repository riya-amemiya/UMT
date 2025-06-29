/**
 * Checks if an object is empty (has no own properties)
 * @param object - The object to check
 * @returns true if the object is empty, false otherwise
 */
export const isEmpty = (object: Record<string, unknown>): boolean => {
  // Check enumerable string keys
  for (const key in object) {
    if (Object.hasOwn(object, key)) {
      return false;
    }
  }

  // Check symbol properties
  const symbolKeys = Object.getOwnPropertySymbols(object);
  if (symbolKeys.length > 0) {
    return false;
  }

  return true;
};
