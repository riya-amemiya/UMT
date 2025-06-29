import { reduce } from "@/Math/reduce";

describe("reduce function", () => {
  describe("basic fraction reduction", () => {
    it("should reduce simple fractions", () => {
      expect(reduce(2, 4)).toEqual({ x: 1, y: 2, gcd: 2 });
      expect(reduce(3, 6)).toEqual({ x: 1, y: 2, gcd: 3 });
      expect(reduce(8, 12)).toEqual({ x: 2, y: 3, gcd: 4 });
    });

    it("should handle already reduced fractions", () => {
      expect(reduce(3, 4)).toEqual({ x: 3, y: 4, gcd: 1 });
      expect(reduce(7, 9)).toEqual({ x: 7, y: 9, gcd: 1 });
    });

    it("should handle fractions with 1", () => {
      expect(reduce(5, 1)).toEqual({ x: 5, y: 1, gcd: 1 });
      expect(reduce(1, 7)).toEqual({ x: 1, y: 7, gcd: 1 });
    });
  });

  describe("edge cases", () => {
    it("should handle zero in numerator or denominator", () => {
      expect(reduce(0, 5)).toEqual({ x: Number.NaN, y: Number.NaN });
      expect(reduce(5, 0)).toEqual({ x: Number.NaN, y: Number.NaN });
      expect(reduce(0, 0)).toEqual({ x: Number.NaN, y: Number.NaN });
    });

    it("should handle negative numbers", () => {
      expect(reduce(-6, 8)).toEqual({ x: -3, y: 4, gcd: 2 });
      expect(reduce(6, -8)).toEqual({ x: -3, y: 4, gcd: 2 });
      expect(reduce(-6, -8)).toEqual({ x: 3, y: 4, gcd: 2 });
    });

    it("should handle larger numbers", () => {
      expect(reduce(1000, 2500)).toEqual({ x: 2, y: 5, gcd: 500 });
      expect(reduce(24_680, 12_340)).toEqual({ x: 2, y: 1, gcd: 12_340 });
    });
  });
});
