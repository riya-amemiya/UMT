import { quotient } from "@/Math/quotient";
test("{quotient}", () => {
  expect(quotient(1, 1)).toEqual([1, 0]);
  expect(quotient(1, 2)).toEqual([0, 1]);
  expect(quotient(2, 1)).toEqual([2, 0]);
  expect(quotient(2, 2)).toEqual([1, 0]);
  expect(quotient(2, 3)).toEqual([0, 2]);
  expect(quotient(3, 1)).toEqual([3, 0]);
  expect(quotient(3, 2)).toEqual([1, 1]);
  expect(quotient(3, 3)).toEqual([1, 0]);
  expect(quotient(3, 4)).toEqual([0, 3]);
});
