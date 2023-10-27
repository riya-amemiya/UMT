export const isArray = <T>(array: unknown): array is T[] => {
  return Array.isArray(array);
};
