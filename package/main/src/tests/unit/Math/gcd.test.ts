import { gcd } from "@/Math/gcd";
describe("gcd function", () => {
  // Test with two positive integers
  test("two positive integers", () => {
    expect(gcd(56, 48)).toBe(8);
  });

  // Test with more than two positive integers
  test("more than two positive integers", () => {
    expect(gcd(56, 48, 32)).toBe(8);
    expect(gcd(56, 48, 32, 24)).toBe(8);
  });

  // Test with zero
  test("contains zero", () => {
    expect(gcd(56, 0)).toBe(0);
    expect(gcd(0, 56)).toBe(0);
  });

  // Test with one
  test("contains one", () => {
    expect(gcd(56, 1)).toBe(1);
    expect(gcd(1, 56)).toBe(1);
  });

  // Test with negative numbers
  test("contains negative numbers", () => {
    expect(gcd(-56, 48)).toBe(8);
    expect(gcd(56, -48)).toBe(8);
    expect(gcd(-56, -48)).toBe(8);
    expect(gcd(-56, 48, -32)).toBe(8);
  });
});
