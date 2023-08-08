import { multiplication } from "@/Math/multiplication";
test("{multiplication}", () => {
  expect(multiplication(1, 1)).toBe(1);
  expect(multiplication(1, 2)).toBe(2);
  expect(multiplication(1.1, 2.2)).toBe(2.42);
});
