import { isPrimeNumberAsync } from "@/Async/Math/isPrimeNumberAsync";

describe("isPrimeNumberAsync", () => {
  test("returns false for numbers less than or equal to 1", async () => {
    expect(await isPrimeNumberAsync(0)).toBe(false);
    expect(await isPrimeNumberAsync(1)).toBe(false);
  });

  test("returns true for prime numbers", async () => {
    expect(await isPrimeNumberAsync(2)).toBe(true);
    expect(await isPrimeNumberAsync(3)).toBe(true);
    expect(await isPrimeNumberAsync(5)).toBe(true);
    expect(await isPrimeNumberAsync(7)).toBe(true);
    expect(await isPrimeNumberAsync(11)).toBe(true);
    expect(await isPrimeNumberAsync(13)).toBe(true);
    expect(await isPrimeNumberAsync(17)).toBe(true);
    expect(await isPrimeNumberAsync(19)).toBe(true);
    expect(await isPrimeNumberAsync(23)).toBe(true);
    expect(await isPrimeNumberAsync(29)).toBe(true);
  });

  test("returns false for composite numbers", async () => {
    expect(await isPrimeNumberAsync(4)).toBe(false);
    expect(await isPrimeNumberAsync(6)).toBe(false);
    expect(await isPrimeNumberAsync(8)).toBe(false);
    expect(await isPrimeNumberAsync(9)).toBe(false);
    expect(await isPrimeNumberAsync(10)).toBe(false);
    expect(await isPrimeNumberAsync(12)).toBe(false);
    expect(await isPrimeNumberAsync(14)).toBe(false);
    expect(await isPrimeNumberAsync(15)).toBe(false);
    expect(await isPrimeNumberAsync(16)).toBe(false);
    expect(await isPrimeNumberAsync(18)).toBe(false);
  });

  test("should return false for large non-prime number", async () => {
    const largeNonPrimeNumber = 10000000000000;
    expect(await isPrimeNumberAsync(largeNonPrimeNumber)).toBe(false);
  });

  test("should return true for large prime number", async () => {
    const largePrimeNumber = 982451653; // This is a known large prime number
    expect(await isPrimeNumberAsync(largePrimeNumber)).toBe(true);
  });
});
