import { nPr } from "@/Math/nPr";

describe("nPr function", () => {
  describe("valid permutations", () => {
    it("should calculate basic permutations correctly", () => {
      expect(nPr(5, 2)).toBe(20); // 5P2 = 20
      expect(nPr(10, 4)).toBe(5040); // 10P4 = 5040
    });

    it("should handle choosing one item", () => {
      expect(nPr(3, 1)).toBe(3); // 3P1 = 3
      expect(nPr(5, 1)).toBe(5); // 5P1 = 5
    });

    it("should handle choosing all items", () => {
      expect(nPr(3, 3)).toBe(6); // 3P3 = 6 (factorial of 3)
      expect(nPr(4, 4)).toBe(24); // 4P4 = 24 (factorial of 4)
    });

    it("should handle larger permutations", () => {
      expect(nPr(8, 3)).toBe(336); // 8P3 = 336
      expect(nPr(6, 4)).toBe(360); // 6P4 = 360
    });
  });

  describe("edge cases", () => {
    it("should return NaN when n or r is 0", () => {
      expect(nPr(0, 5)).toBeNaN();
      expect(nPr(5, 0)).toBeNaN();
      expect(nPr(0, 0)).toBeNaN();
    });

    it("should return NaN when n is less than r", () => {
      expect(nPr(2, 5)).toBeNaN();
      expect(nPr(3, 4)).toBeNaN();
    });

    it("should calculate correctly when n equals r", () => {
      expect(nPr(2, 2)).toBe(2); // 2P2 = 2
      expect(nPr(5, 5)).toBe(120); // 5P5 = 120
    });
  });
});
