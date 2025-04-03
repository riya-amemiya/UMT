import { quotient } from "@/Math/quotient";

describe("quotient function", () => {
  describe("basic division", () => {
    it("should handle exact division (no remainder)", () => {
      expect(quotient(4, 2)).toEqual([2, 0]);
      expect(quotient(10, 5)).toEqual([2, 0]);
      expect(quotient(100, 10)).toEqual([10, 0]);
    });

    it("should handle division with remainder", () => {
      expect(quotient(5, 2)).toEqual([2, 1]);
      expect(quotient(7, 3)).toEqual([2, 1]);
      expect(quotient(10, 3)).toEqual([3, 1]);
    });
  });

  describe("edge cases", () => {
    it("should handle division by 1", () => {
      expect(quotient(5, 1)).toEqual([5, 0]);
      expect(quotient(0, 1)).toEqual([0, 0]);
    });

    it("should handle when dividend is smaller than divisor", () => {
      expect(quotient(2, 5)).toEqual([0, 2]);
      expect(quotient(1, 10)).toEqual([0, 1]);
    });

    it("should handle negative numbers", () => {
      expect(quotient(-5, 2)).toEqual([-2, -1]);
      expect(quotient(5, -2)).toEqual([-2, 1]);
      expect(quotient(-5, -2)).toEqual([2, -1]);
    });

    it("should handle large numbers", () => {
      expect(quotient(1000, 3)).toEqual([333, 1]);
      expect(quotient(9999, 10)).toEqual([999, 9]);
    });
  });
});
