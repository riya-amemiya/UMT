/**
 * Determines if the current environment is Node.js
 */
export const isNode = () => {
  try {
    // biome-ignore lint/nursery/noProcessGlobal: ignore
    return typeof process !== "undefined" && typeof require !== "undefined";
  } catch {
    return false;
  }
};
