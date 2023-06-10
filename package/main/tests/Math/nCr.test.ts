import { nCr } from "../../module/Math/nCr";
test("{nCr}", () => {
  expect(nCr(1, 1)).toBe(1);
  expect(nCr(2, 1)).toBe(2);
  expect(nCr(2, 2)).toBe(1);
  expect(nCr(3, 1)).toBe(3);
});
