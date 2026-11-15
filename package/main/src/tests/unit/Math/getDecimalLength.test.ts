import { getDecimalLength } from "@/Math/getDecimalLength";

const previousGetDecimalLength = (value: number) => {
  const string_ = value.toString();
  const x = string_.split(".", 2)[1];
  return x !== undefined && x.length > 0 ? x.length : 0;
};

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
      expect(getDecimalLength(1.000_001)).toBe(6);
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
      expect(getDecimalLength(123_456.789)).toBe(3);
      expect(getDecimalLength(-987_654.321)).toBe(3);
    });

    it("should count characters after the first dot in scientific notation", () => {
      expect(getDecimalLength(1.23e21)).toBe(6);
      expect(getDecimalLength(1e-7)).toBe(0);
      expect(getDecimalLength(1.23e-7)).toBe(5);
      expect(getDecimalLength(Number.MIN_VALUE)).toBe(0);
      expect(getDecimalLength(Number.MAX_VALUE)).toBe(21);
    });

    it("should return 0 for Infinity and NaN", () => {
      expect(getDecimalLength(Number.POSITIVE_INFINITY)).toBe(0);
      expect(getDecimalLength(Number.NEGATIVE_INFINITY)).toBe(0);
      expect(getDecimalLength(Number.NaN)).toBe(0);
    });

    it("matches the previous toString + split path", () => {
      const samples = [
        0,
        -0,
        1,
        -42,
        1.0,
        1.1,
        1.11,
        1.123,
        1.01,
        1.001,
        1.000_001,
        -1.1,
        -0.01,
        123_456.789,
        -987_654.321,
        0.1,
        0.2,
        Math.PI,
        1e-7,
        1.23e21,
        1.23e-7,
        Number.MIN_VALUE,
        Number.MAX_VALUE,
        Number.POSITIVE_INFINITY,
        Number.NEGATIVE_INFINITY,
        Number.NaN,
      ];
      for (const value of samples) {
        expect(getDecimalLength(value)).toBe(previousGetDecimalLength(value));
      }
      for (let index = 0; index < 10_000; index++) {
        const value = (index - 5000) / 1000;
        expect(getDecimalLength(value)).toBe(previousGetDecimalLength(value));
      }
    });
  });
});
