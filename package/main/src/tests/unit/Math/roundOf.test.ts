import { roundOf } from "@/Math/roundOf";

describe("roundOf function", () => {
  describe("basic rounding", () => {
    it("should round to default precision (0)", () => {
      expect(roundOf(1.111111111111)).toBe(1);
      expect(roundOf(1.555555555555)).toBe(2);
      expect(roundOf(1.499999999999)).toBe(1);
    });

    it("should round to specified decimal places", () => {
      expect(roundOf(1.111111111111, 2)).toBe(1.11);
      expect(roundOf(1.555555555555, 2)).toBe(1.56);
      expect(roundOf(1.499999999999, 2)).toBe(1.5);
    });

    it("should handle increasing precision", () => {
      const value = 1.111111111111;
      expect(roundOf(value, 1)).toBe(1.1);
      expect(roundOf(value, 2)).toBe(1.11);
      expect(roundOf(value, 3)).toBe(1.111);
      expect(roundOf(value, 4)).toBe(1.1111);
      expect(roundOf(value, 5)).toBe(1.11111);
    });
  });

  describe("edge cases", () => {
    it("should handle negative numbers", () => {
      expect(roundOf(-1.234, 2)).toBe(-1.23);
      expect(roundOf(-1.235, 2)).toBe(-1.24);
    });

    it("should handle zero with different precisions", () => {
      expect(roundOf(0, 0)).toBe(0);
      expect(roundOf(0, 2)).toBe(0);
      expect(roundOf(0.0, 4)).toBe(0);
    });

    it("should handle negative precision", () => {
      expect(roundOf(1234.5678, -1)).toBe(1230);
      expect(roundOf(1234.5678, -2)).toBe(1200);
    });
  });
});
