import { isValueNaN } from "@/Math/isValueNaN";
test("{isValueNaN}", () => {
  expect(isValueNaN(0)).toBe(false);
  expect(isValueNaN(0, true)).toBe(false);
  expect(isValueNaN(NaN)).toBe(true);
  expect(isValueNaN(NaN, true)).toBe(true);
  expect(isValueNaN("NaN")).toBe(false);
  expect(isValueNaN("NaN", true)).toBe(true);
  expect(isValueNaN("0")).toBe(false);
  expect(isValueNaN("0", true)).toBe(false);
  expect(isValueNaN("a")).toBe(false);
  expect(isValueNaN("a", true)).toBe(true);
});
