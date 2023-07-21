export const isArr = <T>(arr: unknown): arr is T[] => {
  return Array.isArray(arr);
};
