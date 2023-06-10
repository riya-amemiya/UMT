import { arrayMap } from "@/Array";
export const objectMap = <T, U>(
  object: { [key: string]: T },
  callbackfn: (value: T, key: string, index: number) => U,
) => {
  return arrayMap(Object.entries(object), ([key, value], index) => {
    return callbackfn(value, key, index);
  });
};
