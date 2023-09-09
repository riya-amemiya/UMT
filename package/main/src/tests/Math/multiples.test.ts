import { multiples } from "@/Math/multiples";
test("{multiples}", () => {
  expect(multiples(1, 1)).toEqual([1]);
  expect(multiples(1, 2)).toEqual([1, 2]);
  expect(multiples(1, 3)).toEqual([1, 2, 3]);
  expect(multiples(1, 4)).toEqual([1, 2, 3, 4]);
  expect(multiples(1, 5)).toEqual([1, 2, 3, 4, 5]);
  expect(multiples(1, 6)).toEqual([1, 2, 3, 4, 5, 6]);
  expect(multiples(2, 1)).toEqual([2]);
  expect(multiples(2, 2)).toEqual([2, 4]);
  expect(multiples(2, 3)).toEqual([2, 4, 6]);
});
