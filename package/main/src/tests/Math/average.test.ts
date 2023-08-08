import { average } from "@/Math/average";
test("{average}", () => {
  expect(average([1, 2])).toBe(1.5);
  expect(average([1.1, 2.2])).toBeCloseTo(1.65);
  expect(average([1.1, 2.2, 3.3])).toBe(2.2);
  expect(average([1.1, 2.2, 3.3, 4.4])).toBe(2.75);
  expect(average([1.1, 2.2, 3.3, 4.4, 5.5])).toBe(3.3);
  expect(average([1.1, 2.2, 3.3, 4.4, 5.5, 6.6])).toBe(3.85);
});
