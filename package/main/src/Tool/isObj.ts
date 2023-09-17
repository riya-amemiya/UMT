export const isObj = <T extends { [key: string]: unknown }>(
  obj: unknown,
): obj is T => {
  return typeof obj === "object" && obj !== null && !Array.isArray(obj);
};
