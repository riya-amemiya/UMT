import { average } from "@/Math/average";
describe("average function", () => {
  test("Basic scenarios", () => {
    expect(average([1, 2])).toBe(1.5);
    expect(average([1.1, 2.2])).toBeCloseTo(1.65);
    expect(average([1.1, 2.2, 3.3, 4.4, 5.5])).toBeCloseTo(3.3);
    expect(average([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])).toBe(5.5);
  });

  test("Array with single element", () => {
    expect(average([5])).toBe(5);
  });

  test("Array with zero", () => {
    expect(average([0, 0, 0])).toBe(0);
  });

  test("Negative numbers", () => {
    expect(average([-1, -2])).toBe(-1.5);
  });

  test("Mixed positive and negative numbers", () => {
    expect(average([-1, 1])).toBe(0);
  });

  test("Empty array", () => {
    expect(average([])).toBe(0);
  });
});
