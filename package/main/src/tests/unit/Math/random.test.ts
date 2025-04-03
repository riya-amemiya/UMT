import { random } from "@/Math/random";

describe("random function", () => {
  describe("basic random generation", () => {
    it("should generate random numbers within specified range", () => {
      const min = 1;
      const max = 10;
      for (let i = 0; i < 100; i++) {
        const result = random(max, min);
        expect(result).toBeGreaterThanOrEqual(min);
        expect(result).toBeLessThanOrEqual(max);
        expect(Number.isInteger(result)).toBe(true);
      }
    });

    it("should use 0 as default minimum value", () => {
      const max = 5;
      for (let i = 0; i < 100; i++) {
        const result = random(max);
        expect(result).toBeGreaterThanOrEqual(0);
        expect(result).toBeLessThanOrEqual(max);
        expect(Number.isInteger(result)).toBe(true);
      }
    });
  });

  describe("edge cases", () => {
    it("should return same value when max equals min", () => {
      const value = 7;
      const result = random(value, value);
      expect(result).toBe(value);
    });

    it("should handle negative numbers", () => {
      const min = -10;
      const max = -5;
      for (let i = 0; i < 100; i++) {
        const result = random(max, min);
        expect(result).toBeGreaterThanOrEqual(min);
        expect(result).toBeLessThanOrEqual(max);
        expect(Number.isInteger(result)).toBe(true);
      }
    });

    it("should handle mixed positive and negative range", () => {
      const min = -5;
      const max = 5;
      for (let i = 0; i < 100; i++) {
        const result = random(max, min);
        expect(result).toBeGreaterThanOrEqual(min);
        expect(result).toBeLessThanOrEqual(max);
        expect(Number.isInteger(result)).toBe(true);
      }
    });

    it("should handle when min is greater than max", () => {
      const min = 10;
      const max = 5;
      const result = random(max, min);
      expect(result).toBeGreaterThanOrEqual(max);
      expect(result).toBeLessThanOrEqual(min);
      expect(Number.isInteger(result)).toBe(true);
    });
  });
});
