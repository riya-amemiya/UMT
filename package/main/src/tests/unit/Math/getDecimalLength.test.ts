import { getDecimalLength } from "@/Math/getDecimalLength";
describe("getDecimalLength function", () => {
  describe("integers", () => {
    it("should return 0 for whole numbers", () => {
      expect(getDecimalLength(1)).toBe(0);
      expect(getDecimalLength(100)).toBe(0);
      expect(getDecimalLength(-42)).toBe(0);
    });

    it("should return 0 for whole numbers with .0", () => {
      expect(getDecimalLength(1.0)).toBe(0);
      expect(getDecimalLength(42.0)).toBe(0);
      expect(getDecimalLength(-1.0)).toBe(0);
    });
  });

  describe("decimal numbers", () => {
    it("should count decimal places correctly", () => {
      expect(getDecimalLength(1.1)).toBe(1);
      expect(getDecimalLength(1.11)).toBe(2);
      expect(getDecimalLength(1.123)).toBe(3);
    });

    it("should handle leading zeros in decimals", () => {
      expect(getDecimalLength(1.01)).toBe(2);
      expect(getDecimalLength(1.001)).toBe(3);
      expect(getDecimalLength(1.000001)).toBe(6);
    });

    it("should handle negative numbers with decimals", () => {
      expect(getDecimalLength(-1.1)).toBe(1);
      expect(getDecimalLength(-0.01)).toBe(2);
    });
  });

  describe("edge cases", () => {
    it("should handle zero", () => {
      expect(getDecimalLength(0)).toBe(0);
      expect(getDecimalLength(0.0)).toBe(0);
      expect(getDecimalLength(-0)).toBe(0);
    });

    it("should handle large decimal numbers", () => {
      expect(getDecimalLength(123456.789)).toBe(3);
      expect(getDecimalLength(-987654.321)).toBe(3);
    });
  });
});
