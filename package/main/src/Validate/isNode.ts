/**
 * Determines if the current environment is Node.js
 */
export const isNode = () => {
  try {
    return typeof process !== "undefined" && typeof require !== "undefined";
  } catch {
    return false;
  }
};
