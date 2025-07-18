import { isPrimeNumber } from "@/Validate/isPrimeNumber";

describe("isPrimeNumber", () => {
  test("returns false for numbers less than or equal to 1", () => {
    expect(isPrimeNumber(0)).toBe(false);
    expect(isPrimeNumber(1)).toBe(false);
  });

  test("returns true for prime numbers", () => {
    expect(isPrimeNumber(2)).toBe(true);
    expect(isPrimeNumber(3)).toBe(true);
    expect(isPrimeNumber(5)).toBe(true);
    expect(isPrimeNumber(7)).toBe(true);
    expect(isPrimeNumber(11)).toBe(true);
    expect(isPrimeNumber(13)).toBe(true);
    expect(isPrimeNumber(17)).toBe(true);
    expect(isPrimeNumber(19)).toBe(true);
    expect(isPrimeNumber(23)).toBe(true);
    expect(isPrimeNumber(29)).toBe(true);
  });

  test("returns false for composite numbers", () => {
    expect(isPrimeNumber(4)).toBe(false);
    expect(isPrimeNumber(6)).toBe(false);
    expect(isPrimeNumber(8)).toBe(false);
    expect(isPrimeNumber(9)).toBe(false);
    expect(isPrimeNumber(10)).toBe(false);
    expect(isPrimeNumber(12)).toBe(false);
    expect(isPrimeNumber(14)).toBe(false);
    expect(isPrimeNumber(15)).toBe(false);
    expect(isPrimeNumber(16)).toBe(false);
    expect(isPrimeNumber(18)).toBe(false);
  });
  test("should return false for negative numbers", () => {
    expect(isPrimeNumber(-2)).toBe(false);
    expect(isPrimeNumber(-7)).toBe(false);
    expect(isPrimeNumber(-11)).toBe(false);
  });

  test("should return false for non-integer numbers", () => {
    expect(isPrimeNumber(2.5)).toBe(false);
    expect(isPrimeNumber(3.14)).toBe(false);
    expect(isPrimeNumber(7.0)).toBe(true); // 7.0 is treated as integer 7
  });

  test("should handle large non-prime numbers", () => {
    const largeNonPrimeNumber = 10_000_000_000_000; // 10^13
    expect(isPrimeNumber(largeNonPrimeNumber)).toBe(false);
  });

  test("should handle large prime numbers", () => {
    const largePrimeNumber = 982_451_653; // 982,451,653 is a known prime number
    expect(isPrimeNumber(largePrimeNumber)).toBe(true);
  });
});
