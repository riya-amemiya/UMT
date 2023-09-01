export const isObj = <T>(obj: unknown): obj is T => {
  return typeof obj === "object" && obj !== null;
};
