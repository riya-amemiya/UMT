export const valueIsNaN = (value: unknown, loose = false): boolean => {
  return loose ? isNaN(value as number) : Number.isNaN(value);
};
