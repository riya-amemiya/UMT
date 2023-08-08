import { nHr } from "../../../module/Math/nHr";
test("{nHr}", () => {
  expect(nHr(1, 1)).toBe(1);
  expect(nHr(2, 1)).toBe(2);
  expect(nHr(2, 2)).toBe(3);
});
