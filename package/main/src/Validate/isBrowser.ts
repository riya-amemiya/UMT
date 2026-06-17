/**
 * Determines if the current environment is a browser
 */
export const isBrowser = () => {
  try {
     
    return typeof window !== "undefined" && typeof document !== "undefined";
  } catch {
    return false;
  }
};
