import { getArraysDiff } from "@/Array/getArraysDiff";
test("{getArraysDiff}", () => {
  expect(getArraysDiff([1, 2, 3], [2, 3, 4], [3, 4, 5])).toEqual([1, 5]);
});
