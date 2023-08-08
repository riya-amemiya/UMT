import { division } from "@/Math/division";
test("{division}", () => {
  expect(division(100, 50)).toBe(2);
  expect(division(10.1, 2.2)).toBe(4.590909090909091);
});
