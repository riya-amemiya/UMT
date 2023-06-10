// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export const isArr = <T>(arr: any): arr is T[] => {
  return Array.isArray(arr);
};
