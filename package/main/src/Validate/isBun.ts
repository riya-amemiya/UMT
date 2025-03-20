/**
 * Determines if the current environment is Bun runtime
 */
export const isBun = () => {
  try {
    // @ts-expect-error: Bun is only defined in Bun runtime
    return typeof Bun !== "undefined";
  } catch {
    return false;
  }
};
