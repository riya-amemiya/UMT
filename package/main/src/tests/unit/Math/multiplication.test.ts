import { multiplication } from "@/Math/multiplication";

describe("multiplication function", () => {
  describe("basic integer multiplication", () => {
    it("should multiply two positive integers", () => {
      expect(multiplication(2, 3)).toBe(6);
      expect(multiplication(10, 20)).toBe(200);
    });

    it("should multiply multiple positive integers", () => {
      expect(multiplication(2, 3, 4)).toBe(24);
      expect(multiplication(1, 2, 3, 4, 5)).toBe(120);
    });
  });

  describe("decimal number multiplication", () => {
    it("should handle decimal multiplication without floating point errors", () => {
      expect(multiplication(0.1, 0.2)).toBeCloseTo(0.02);
      expect(multiplication(0.1, 0.2, 0.3)).toBeCloseTo(0.006);
      expect(multiplication(1.23, 4.56)).toBeCloseTo(5.6088);
    });

    it("should handle mixed integer and decimal multiplication", () => {
      expect(multiplication(0.5, 2)).toBe(1);
      expect(multiplication(0.1, 10)).toBe(1);
      expect(multiplication(1.5, 2, 3)).toBe(9);
    });
  });

  describe("edge cases", () => {
    it("should handle negative numbers", () => {
      expect(multiplication(-2, 3)).toBe(-6);
      expect(multiplication(2, -3)).toBe(-6);
      expect(multiplication(-2, -3)).toBe(6);
    });

    it("should handle zero", () => {
      expect(multiplication(0, 5)).toBe(0);
      expect(multiplication(5, 0)).toBe(0);
      expect(multiplication(0, 0)).toBe(0);
    });

    it("should handle multiplication by one", () => {
      expect(multiplication(1, 5)).toBe(5);
      expect(multiplication(5, 1)).toBe(5);
      expect(multiplication(1, 1)).toBe(1);
    });

    it("should handle very small numbers", () => {
      expect(multiplication(0.0001, 0.0001)).toBeCloseTo(0.00000001);
      expect(multiplication(0.00001, 0.00001)).toBeCloseTo(0.0000000001);
    });
  });
});
