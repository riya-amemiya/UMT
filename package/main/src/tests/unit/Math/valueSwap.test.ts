import { valueSwap } from "@/Math/valueSwap";
test("{valueSwap}", () => {
  expect(valueSwap(1, 2)).toEqual([1, 2]);
  expect(valueSwap(2, 1)).toEqual([1, 2]);
  expect(valueSwap(1, 1)).toEqual([1, 1]);
  expect(valueSwap(2, 2)).toEqual([2, 2]);
  expect(valueSwap(3, 3)).toEqual([3, 3]);
});
