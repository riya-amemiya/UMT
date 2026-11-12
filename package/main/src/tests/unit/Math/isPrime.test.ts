import { isPrime } from "@/Math/isPrime";

describe("isPrime function", () => {
  it("should return false for numbers below 2", () => {
    expect(isPrime(1)).toBe(false);
    expect(isPrime(0)).toBe(false);
    expect(isPrime(-7)).toBe(false);
  });

  it("should return true for 2 and 3", () => {
    expect(isPrime(2)).toBe(true);
    expect(isPrime(3)).toBe(true);
  });

  it("should return false for multiples of 2 or 3", () => {
    expect(isPrime(4)).toBe(false);
    expect(isPrime(9)).toBe(false);
    expect(isPrime(100)).toBe(false);
  });

  it("should return true for primes found by the 6k +/- 1 wheel", () => {
    expect(isPrime(5)).toBe(true);
    expect(isPrime(7)).toBe(true);
    expect(isPrime(29)).toBe(true);
    expect(isPrime(97)).toBe(true);
    expect(isPrime(7919)).toBe(true);
    expect(isPrime(2_147_483_647)).toBe(true);
  });

  it("should return false for composites divisible by 6k - 1", () => {
    expect(isPrime(25)).toBe(false);
    expect(isPrime(121)).toBe(false);
  });

  it("should return false for composites divisible by 6k + 1", () => {
    expect(isPrime(49)).toBe(false);
    expect(isPrime(91)).toBe(false);
    expect(isPrime(169)).toBe(false);
  });

  it("should return false for non-integers", () => {
    expect(isPrime(7.5)).toBe(false);
    expect(isPrime(2.000_001)).toBe(false);
  });

  it("should return false for NaN and Infinity", () => {
    expect(isPrime(Number.NaN)).toBe(false);
    expect(isPrime(Number.POSITIVE_INFINITY)).toBe(false);
    expect(isPrime(Number.NEGATIVE_INFINITY)).toBe(false);
  });

  it("should return false for integers above 2^53", () => {
    expect(isPrime(2 ** 53 + 2)).toBe(false);
  });
});
