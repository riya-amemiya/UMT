import { deviationValue } from "@/Math/deviationValue";

// Tests for standard score calculation
describe("deviationValue", () => {
  // Test with normal input values
  test("should handle normal input values", () => {
    expect(deviationValue(100, 50, 10)).toBe(100);
  });

  // Test with value equal to mean
  test("should handle value equal to mean", () => {
    expect(deviationValue(50, 50, 10)).toBe(50);
  });

  // Test with zero standard deviation
  test("should handle zero standard deviation", () => {
    expect(deviationValue(100, 50, 0)).toBe(Number.POSITIVE_INFINITY);
  });

  // Test with negative values
  test("should handle negative values", () => {
    expect(deviationValue(-20, 0, 20)).toBe(40);
  });

  // Test with floating point numbers
  test("should handle floating point numbers", () => {
    expect(deviationValue(55.5, 50, 10)).toBeCloseTo(55.5);
  });
});
