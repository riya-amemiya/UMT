import { generateNumberArray } from "@/Array/generateNumberArray";

describe("generateNumberArray", () => {
  it("should generate an array with the specified length", () => {
    const result = generateNumberArray(5);
    expect(result).toEqual([0, 1, 2, 3, 4]);
  });

  it("should generate an array with specified min and max values", () => {
    const result = generateNumberArray(5, 10, 14);
    expect(result).toEqual([10, 11, 12, 13, 14]);
  });

  it("should return an empty array when length is 0", () => {
    const result = generateNumberArray(0);
    expect(result).toEqual([]);
  });

  it("should return min value when length is 1", () => {
    const result = generateNumberArray(1, 10, 20);
    expect(result).toEqual([10]);
  });

  it("should generate an array with random values", () => {
    const result = generateNumberArray(5, 10, 14, true);
    expect(result).toHaveLength(5);
    for (const item of result) {
      expect(item).toBeGreaterThanOrEqual(10);
      expect(item).toBeLessThanOrEqual(14);
    }
  });

  it("should handle negative length gracefully", () => {
    const result = generateNumberArray(-5);
    expect(result).toEqual([]);

    const result2 = generateNumberArray(-1, 10, 20);
    expect(result2).toEqual([]);
  });

  it("should handle fractional lengths by truncating", () => {
    const result = generateNumberArray(3.7);
    expect(result).toEqual([0, 1, 2]);

    const result2 = generateNumberArray(2.9, 5, 10);
    expect(result2).toEqual([5, 10]);
  });

  it("should handle negative min and max values", () => {
    const result = generateNumberArray(5, -10, -6);
    expect(result).toEqual([-10, -9, -8, -7, -6]);

    const result2 = generateNumberArray(3, -5, 5);
    expect(result2).toEqual([-5, 0, 5]);
  });

  it("should handle equal min and max values", () => {
    const result = generateNumberArray(5, 10, 10);
    expect(result).toEqual([10, 10, 10, 10, 10]);

    const result2 = generateNumberArray(1, 7, 7);
    expect(result2).toEqual([7]);
  });

  it("should handle decimal min and max values", () => {
    const result = generateNumberArray(3, 1.5, 2.5);
    expect(result).toEqual([1.5, 2, 2.5]);

    const result2 = generateNumberArray(5, 0.1, 0.9);
    expect(result2).toEqual([0.1, 0.3, 0.5, 0.7, 0.9]);
  });

  it("should handle large arrays efficiently", () => {
    const result = generateNumberArray(10_000);
    expect(result).toHaveLength(10_000);
    expect(result[0]).toBe(0);
    expect(result[9999]).toBe(9999);

    const result2 = generateNumberArray(10_000, 1000, 2000);
    expect(result2).toHaveLength(10_000);
    expect(result2[0]).toBe(1000);
    expect(result2[9999]).toBe(2000);
  });

  it("should handle very small ranges", () => {
    const result = generateNumberArray(1000, 0, 0.001);
    expect(result).toHaveLength(1000);
    expect(result[0]).toBe(0);
    expect(result[999]).toBeCloseTo(0.001, 6);
  });

  it("should handle random generation with edge cases", () => {
    const result = generateNumberArray(5, 10, 10, true);
    expect(result).toEqual([10, 10, 10, 10, 10]);

    const result2 = generateNumberArray(1, 5, 15, true);
    expect(result2).toEqual([5]);

    const result3 = generateNumberArray(100, 1, 100, true);
    const result4 = generateNumberArray(100, 1, 100, true);

    expect(result3).not.toEqual(result4);

    for (const value of result3) {
      expect(value).toBeGreaterThanOrEqual(1);
      expect(value).toBeLessThanOrEqual(100);
      expect(Number.isInteger(value)).toBe(true);
    }
  });

  it("should handle extreme ranges", () => {
    const result = generateNumberArray(
      3,
      Number.MIN_SAFE_INTEGER,
      Number.MAX_SAFE_INTEGER,
    );
    expect(result).toHaveLength(3);
    expect(result[0]).toBe(Number.MIN_SAFE_INTEGER);
    expect(result[2]).toBe(Number.MAX_SAFE_INTEGER);

    const result2 = generateNumberArray(2, 1e15, 2e15);
    expect(result2).toEqual([1e15, 2e15]);
  });

  it("should handle zero as min or max", () => {
    const result = generateNumberArray(5, 0, 4);
    expect(result).toEqual([0, 1, 2, 3, 4]);

    const result2 = generateNumberArray(3, -2, 0);
    expect(result2).toEqual([-2, -1, 0]);
  });

  it("should maintain precision with decimal steps", () => {
    const result = generateNumberArray(11, 0, 1);
    expect(result).toEqual([0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1]);

    expect(result[5]).toBeCloseTo(0.5, 10);
  });

  it("should handle non-integer lengths in random mode", () => {
    const result = generateNumberArray(3.8, 1, 10, true);
    expect(result).toHaveLength(3);

    for (const value of result) {
      expect(value).toBeGreaterThanOrEqual(1);
      expect(value).toBeLessThanOrEqual(10);
    }
  });

  it("should handle special numeric edge cases", () => {
    const result = generateNumberArray(
      2,
      Number.MIN_VALUE,
      Number.MIN_VALUE * 2,
    );
    expect(result).toHaveLength(2);
    expect(result[0]).toBe(Number.MIN_VALUE);

    expect(generateNumberArray(0, -100, 100)).toEqual([]);
    expect(generateNumberArray(0, 0, 0, true)).toEqual([]);
  });

  it("should generate correct step sizes for various ranges", () => {
    const result = generateNumberArray(4, 0, 10);
    expect(result).toEqual([0, 10 / 3, 20 / 3, 10]);

    const result2 = generateNumberArray(3, 1, 7);
    expect(result2).toEqual([1, 4, 7]);
  });

  it("should handle Math.random edge cases in random mode", () => {
    const originalRandom = Math.random;

    Math.random = () => 0;
    const result1 = generateNumberArray(3, 5, 10, true);
    expect(result1).toEqual([5, 5, 5]);

    Math.random = () => 0.999_999_9;
    const result2 = generateNumberArray(3, 5, 10, true);
    expect(result2).toEqual([10, 10, 10]);

    Math.random = originalRandom;
  });

  it("should handle default parameter values correctly", () => {
    const result1 = generateNumberArray(5);
    expect(result1).toEqual([0, 1, 2, 3, 4]);

    const result2 = generateNumberArray(3, 10, 20);
    expect(result2).toEqual([10, 15, 20]);
  });

  it("should handle random generation with decimal min/max", () => {
    const result = generateNumberArray(5, 0.5, 1.5, true);
    expect(result).toHaveLength(5);
    for (const item of result) {
      expect(item).toBeGreaterThanOrEqual(0.5);
      expect(item).toBeLessThanOrEqual(1.5);
    }
  });
});
