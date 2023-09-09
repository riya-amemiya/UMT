import { isPrimeNumber } from "@/Math/isPrimeNumber";

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
});
