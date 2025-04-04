/**
 * Determines if the current environment is Bun runtime
 */
export const isBun = () => {
  try {
    return typeof Bun !== "undefined";
  } catch {
    return false;
  }
};
