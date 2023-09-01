export const valueIsNaN = (value: unknown, loose = false): boolean => {
  return loose ? Number.isNaN(value as number) : Number.isNaN(value);
};
