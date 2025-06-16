import { lcm } from "@/Math/lcm";

describe("lcm function", () => {
  test("should handle positive numbers", () => {
    expect(lcm(4, 5)).toBe(20);
    expect(lcm(6, 8)).toBe(24);
    expect(lcm(10, 15)).toBe(30);
    expect(lcm(7, 11)).toBe(77);
    expect(lcm(2, 3)).toBe(6);
    expect(lcm(12, 18)).toBe(36);
  });

  test("should handle zero values", () => {
    expect(lcm(0, 5)).toBe(0);
    expect(lcm(4, 0)).toBe(0);
    expect(lcm(0, 0)).toBe(0);
    expect(lcm(0, 10)).toBe(0);
    expect(lcm(7, 0)).toBe(0);
  });

  test("should handle values with one", () => {
    expect(lcm(1, 5)).toBe(5);
    expect(lcm(4, 1)).toBe(4);
    expect(lcm(1, 1)).toBe(1);
    expect(lcm(1, 10)).toBe(10);
    expect(lcm(7, 1)).toBe(7);
  });

  test("should handle large numbers", () => {
    expect(lcm(1000, 2000)).toBe(2000);
    expect(lcm(12345, 67890)).toBe(55873470);
    expect(lcm(999, 1001)).toBe(999999);
  });

  test("should handle negative numbers", () => {
    expect(lcm(-4, 6)).toBe(12);
    expect(lcm(4, -6)).toBe(12);
    expect(lcm(-4, -6)).toBe(12);
    expect(lcm(-10, -15)).toBe(30);
  });

  test("should handle same numbers", () => {
    expect(lcm(5, 5)).toBe(5);
    expect(lcm(42, 42)).toBe(42);
    expect(lcm(1, 1)).toBe(1);
  });

  test("should handle coprime numbers", () => {
    expect(lcm(3, 5)).toBe(15);
    expect(lcm(7, 11)).toBe(77);
    expect(lcm(13, 17)).toBe(221);
  });

  test("should handle multiples", () => {
    expect(lcm(6, 12)).toBe(12);
    expect(lcm(5, 25)).toBe(25);
    expect(lcm(4, 20)).toBe(20);
  });

  test("should handle decimal numbers", () => {
    expect(lcm(2.5, 5)).toBe(5);
    expect(lcm(1.5, 3)).toBe(3);
    expect(lcm(0.5, 1.5)).toBe(1.5);
  });

  test("should handle edge cases", () => {
    expect(lcm(2, 2)).toBe(2);
    expect(lcm(100, 200)).toBe(200);
    expect(lcm(17, 1)).toBe(17);
  });
});
