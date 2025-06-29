import { factorial } from "@/Math/factorial";

describe("factorial", () => {
  test("should return 1 for factorial of 0", () => {
    expect(factorial(0)).toBe(1);
  });

  test("should return 1 for factorial of 1", () => {
    expect(factorial(1)).toBe(1);
  });

  test("should correctly calculate factorial of positive integers", () => {
    expect(factorial(2)).toBe(2);
    expect(factorial(3)).toBe(6);
    expect(factorial(4)).toBe(24);
    expect(factorial(5)).toBe(120);
    expect(factorial(6)).toBe(720);
    expect(factorial(7)).toBe(5040);
    expect(factorial(8)).toBe(40_320);
    expect(factorial(9)).toBe(362_880);
    expect(factorial(10)).toBe(3_628_800);
  });

  test("should return 1 for negative numbers", () => {
    expect(factorial(-1)).toBe(1);
    expect(factorial(-5)).toBe(1);
    expect(factorial(-10)).toBe(1);
    expect(factorial(-100)).toBe(1);
  });

  test("should handle larger numbers", () => {
    expect(factorial(11)).toBe(39_916_800);
    expect(factorial(12)).toBe(479_001_600);
    expect(factorial(13)).toBe(6_227_020_800);
    expect(factorial(13)).toBe(6_227_020_800);
    expect(factorial(14)).toBe(87_178_291_200);
  });

  test("should handle decimal numbers by using Math.max logic", () => {
    expect(factorial(2.5)).toBe(2);
    expect(factorial(3.7)).toBe(6);
    expect(factorial(5.9)).toBe(120);
    expect(factorial(0.5)).toBe(1);
  });

  test("should handle edge cases", () => {
    expect(factorial(0.1)).toBe(1);
    expect(factorial(0.9)).toBe(1);
    expect(factorial(1.1)).toBe(1);
    expect(factorial(1.9)).toBe(1);
  });
});
