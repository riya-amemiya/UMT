export const objectMap = <T, U>(
  object: { [key: string]: T },
  callbackfn: (value: T, key: string, index: number) => U,
) => {
  const result = { ...object } as unknown as { [key: string]: U };
  for (const key in result) {
    result[key] = callbackfn(object[key], key, Number(key));
  }
  return result;
};
