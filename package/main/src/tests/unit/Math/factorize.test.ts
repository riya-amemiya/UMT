import { factorize } from "@/Math/factorize";
test("{factorize}", () => {
  expect(factorize(12)).toEqual([2, 2, 3]);
  expect(factorize(14)).toEqual([2, 7]);
  expect(factorize(15)).toEqual([3, 5]);
  expect(factorize(16)).toEqual([2, 2, 2, 2]);
  expect(factorize(17)).toEqual([17]);
  expect(factorize(18)).toEqual([2, 3, 3]);
  expect(factorize(19)).toEqual([19]);
  expect(factorize(20)).toEqual([2, 2, 5]);
});
