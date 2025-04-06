/**
 * Checks if an object is not empty
 * @param {object} object - The object to check
 * @returns {boolean} true if the object is not empty, false if it is empty
 * @example isNotEmpty({}); // false
 * isNotEmpty({ a: 1 }); // true
 */
export const isNotEmpty = (object: object): boolean => {
  // Check if the object has any keys
  return Object.keys(object).length > 0;
};
