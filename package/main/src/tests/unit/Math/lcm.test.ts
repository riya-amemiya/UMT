import { lcm } from "@/Math/lcm";

describe("lcm function", () => {
  // Test with positive numbers
  test("should handle positive numbers", () => {
    expect(lcm(4, 5)).toBe(20);
    expect(lcm(6, 8)).toBe(24);
    expect(lcm(10, 15)).toBe(30);
    expect(lcm(7, 11)).toBe(77);
  });

  // Test with zero
  test("should handle zero values", () => {
    expect(lcm(0, 5)).toBe(0);
    expect(lcm(4, 0)).toBe(0);
    expect(lcm(0, 0)).toBe(0);
    expect(lcm(0, 10)).toBe(0);
    expect(lcm(7, 0)).toBe(0);
  });

  // Test with one
  test("should handle values with one", () => {
    expect(lcm(1, 5)).toBe(5);
    expect(lcm(4, 1)).toBe(4);
    expect(lcm(1, 1)).toBe(1);
    expect(lcm(1, 10)).toBe(10);
    expect(lcm(7, 1)).toBe(7);
  });

  // Test with large numbers
  test("should handle large numbers", () => {
    expect(lcm(1000, 2000)).toBe(2000);
    expect(lcm(12345, 67890)).toBe(55873470);
  });

  // Test with negative numbers
  test("should handle negative numbers", () => {
    expect(lcm(-4, 6)).toBe(12);
    expect(lcm(4, -6)).toBe(12);
    expect(lcm(-4, -6)).toBe(12);
  });
});
