/**
 * Determines if the current environment is a browser
 */
export const isBrowser = () => {
  try {
    // eslint-disable-next-line unicorn/prefer-global-this
    return typeof window !== "undefined" && typeof document !== "undefined";
  } catch {
    return false;
  }
};
