export const isValueNaN = (value: unknown, loose = false): boolean => {
  // biome-ignore lint/nursery/noGlobalIsNan: <explanation>
  return loose ? isNaN(value as number) : Number.isNaN(value);
};
