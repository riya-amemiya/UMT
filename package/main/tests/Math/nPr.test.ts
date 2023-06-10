import { nPr } from "../../module/Math/nPr";
test("{nPr}", () => {
  expect(nPr(1, 1)).toBe(1);
  expect(nPr(2, 1)).toBe(2);
  expect(nPr(2, 2)).toBe(2);
  expect(nPr(3, 1)).toBe(3);
  expect(nPr(3, 2)).toBe(6);
  expect(nPr(3, 3)).toBe(6);
  expect(nPr(4, 1)).toBe(4);
  expect(nPr(4, 2)).toBe(12);
});
