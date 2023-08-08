import { quickSort } from "@/Array/quickSort";
test("{quickSort}", () => {
  expect(quickSort([1, 2, 3])).toEqual([1, 2, 3]);
  expect(quickSort([3, 2, 1])).toEqual([1, 2, 3]);
  expect(quickSort([1, 3, 2])).toEqual([1, 2, 3]);
  expect(quickSort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])).toEqual([
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
  ]);
  expect(quickSort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])).toEqual([
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
  ]);
  expect(quickSort([10, 1, 2, 3, 4, 5, 6, 7, 8, 9])).toEqual([
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
  ]);
  expect(quickSort([1, 10, 2, 3, 4, 5, 6, 7, 8, 9])).toEqual([
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
  ]);
});
