/**
 * Determines if the value is a dictionary-type object
 * @param object - Value to check
 * @returns {boolean} true if the value is a dictionary object, false otherwise
 * @example isDictionaryObject({}); // true
 * isDictionaryObject([]); // false
 */
export const isDictionaryObject = <T extends { [key: string]: unknown }>(
  object: unknown,
): object is T => {
  return (
    typeof object === "object" && object !== null && !Array.isArray(object)
  );
};
