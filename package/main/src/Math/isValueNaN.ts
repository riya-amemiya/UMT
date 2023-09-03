export const isValueNaN = (value: unknown, loose = false): boolean => {
  // rome-ignore lint/nursery/noGlobalIsNan: <explanation>
  return loose ? isNaN(value as number) : Number.isNaN(value);
};
