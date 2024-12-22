import { max } from "@/Math/max";
test("{max}", () => {
  expect(max(1, 1)).toBe(1);
  expect(max(1, 2)).toBe(2);
});
