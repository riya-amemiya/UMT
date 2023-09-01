export const valueIsNaN = (value: unknown, loose = false): boolean => {
  // rome-ignore lint/nursery/noGlobalIsNan: <explanation>
  return loose ? isNaN(value as number) : Number.isNaN(value);
};
