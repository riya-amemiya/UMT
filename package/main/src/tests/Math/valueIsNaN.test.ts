import { valueIsNaN } from "@/Math/valueIsNaN";
test("{valueIsNaN}", () => {
  expect(valueIsNaN(0)).toBe(false);
  expect(valueIsNaN(0, true)).toBe(false);
  expect(valueIsNaN(NaN)).toBe(true);
  expect(valueIsNaN(NaN, true)).toBe(true);
  expect(valueIsNaN("NaN")).toBe(false);
  expect(valueIsNaN("NaN", true)).toBe(true);
  expect(valueIsNaN("0")).toBe(false);
  expect(valueIsNaN("0", true)).toBe(false);
  expect(valueIsNaN("a")).toBe(false);
  expect(valueIsNaN("a", true)).toBe(true);
});
