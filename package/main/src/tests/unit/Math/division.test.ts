import { division } from "@/Math/division";

describe("division function", () => {
  describe("basic integer division", () => {
    it("should correctly divide integers", () => {
      expect(division(10, 2)).toBe(5);
      expect(division(30, 5)).toBe(6);
    });

    it("should handle large number divided by small number", () => {
      expect(division(1000, 10)).toBe(100);
      expect(division(10_000, 100)).toBe(100);
    });

    it("should handle small number divided by large number", () => {
      expect(division(10, 100)).toBe(0.1);
      expect(division(1, 1000)).toBe(0.001);
    });
  });

  describe("division with remainder", () => {
    it("should handle division with remainder when isFloor is true", () => {
      expect(division(7, 2, true)).toBe(3.5);
      expect(division(3, 5, true)).toBe(0.6);
    });

    it("should return quotient and remainder when isFloor is false", () => {
      expect(division(10, 3, false)).toEqual([3, 1]);
      expect(division(7, 2, false)).toEqual([3, 1]);
      expect(division(3, 5, false)).toEqual([0, 3]);
    });

    it("should handle large numbers with remainder", () => {
      expect(division(1000, 3, false)).toEqual([333, 1]);
      expect(division(10_000, 7, false)).toEqual([1428, 4]);
    });
  });

  describe("decimal numbers", () => {
    it("should handle decimal numbers with different lengths", () => {
      expect(division(10.5, 2.1)).toBe(5);
      expect(division(0.1, 0.2)).toBe(0.5);
      expect(division(0.001, 0.1)).toBe(0.01);
      expect(division(0.1, 0.001)).toBe(100);
      expect(division(0.0001, 0.01)).toBe(0.01);
    });

    it("should handle decimal numbers with same length", () => {
      expect(division(1.1, 2.2)).toBe(0.5);
      expect(division(0.01, 0.02)).toBe(0.5);
    });

    it("should handle complex decimal combinations", () => {
      expect(division(1.234_56, 2.1)).toBeCloseTo(0.587_885_714);
      expect(division(0.0001, 0.0003)).toBeCloseTo(0.333_333_333);
      expect(division(123.456, 0.789)).toBeCloseTo(156.4715);
    });

    it("should handle decimal numbers with remainder", () => {
      expect(division(1.5, 0.7, false)).toEqual([2, 1]);
      expect(division(0.123, 0.0456, false)).toEqual([2, 123]);
    });
  });

  describe("edge cases", () => {
    it("should return NaN for division by zero", () => {
      expect(division(1, 0)).toBe(Number.NaN);
      expect(division(-1, 0)).toBe(Number.NaN);
      expect(division(-1, 0, false)).toEqual([Number.NaN, Number.NaN]);
      expect(division(0, 0)).toBe(Number.NaN);
      expect(division(0, 0, false)).toEqual([Number.NaN, Number.NaN]);
    });

    it("should handle zero dividend", () => {
      expect(division(0, 1)).toBe(0);
      expect(division(0, -1)).toBe(-0);
      expect(division(0, 2, false)).toEqual([0, 0]);
      expect(division(0, -2, false)).toEqual([-0, 0]);
    });

    it("should handle negative numbers", () => {
      expect(division(-10, 2)).toBe(-5);
      expect(division(10, -2)).toBe(-5);
      expect(division(-10, -2)).toBe(5);
    });

    it("should handle negative decimals", () => {
      expect(division(-0.16, 0.2)).toBe(-0.8);
      expect(division(0.16, -0.2)).toBe(-0.8);
      expect(division(-0.16, -0.2)).toBe(0.8);
    });
  });
});
