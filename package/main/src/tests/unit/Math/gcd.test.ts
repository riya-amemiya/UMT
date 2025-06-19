import { gcd } from "@/Math/gcd";

describe("gcd function", () => {
  test("two positive integers", () => {
    expect(gcd(56, 48)).toBe(8);
    expect(gcd(12, 18)).toBe(6);
    expect(gcd(48, 18)).toBe(6);
  });

  test("more than two positive integers", () => {
    expect(gcd(56, 48, 32)).toBe(8);
    expect(gcd(56, 48, 32, 24)).toBe(8);
    expect(gcd(12, 18, 24)).toBe(6);
  });

  test("contains zero", () => {
    expect(gcd(56, 0)).toBe(0);
    expect(gcd(0, 56)).toBe(0);
    expect(gcd(0, 0)).toBe(0);
  });

  test("contains one", () => {
    expect(gcd(56, 1)).toBe(1);
    expect(gcd(1, 56)).toBe(1);
    expect(gcd(1, 1)).toBe(1);
  });

  test("contains negative numbers", () => {
    expect(gcd(-56, 48)).toBe(8);
    expect(gcd(56, -48)).toBe(8);
    expect(gcd(-56, -48)).toBe(8);
    expect(gcd(-56, 48, -32)).toBe(8);
  });

  test("handles decimal numbers", () => {
    expect(gcd(2.5, 1.5)).toBe(0.5);
    expect(gcd(3.6, 2.4)).toBe(1.2);
    expect(gcd(1.2, 0.8)).toBe(0.4);
  });

  test("handles large numbers", () => {
    expect(gcd(1_000_000, 500_000)).toBe(500_000);
    expect(gcd(123_456_789, 987_654_321)).toBe(9);
    expect(gcd(Number.MAX_SAFE_INTEGER, 999)).toBe(1);
  });

  test("handles edge cases", () => {
    expect(gcd(1, 2)).toBe(1);
    expect(gcd(2, 3)).toBe(1);
    expect(gcd(7, 11)).toBe(1);
  });

  test("handles same numbers", () => {
    expect(gcd(42, 42)).toBe(42);
    expect(gcd(100, 100, 100)).toBe(100);
  });

  test("handles small numbers", () => {
    expect(gcd(2, 4)).toBe(2);
    expect(gcd(3, 9)).toBe(3);
    expect(gcd(5, 25)).toBe(5);
  });
});
