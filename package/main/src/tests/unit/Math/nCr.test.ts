import { nCr } from "@/Math/nCr";

describe("nCr function", () => {
  describe("valid combinations", () => {
    it("should calculate basic combinations correctly", () => {
      expect(nCr(5, 2)).toBe(10); // 5C2 = 10
      expect(nCr(10, 4)).toBe(210); // 10C4 = 210
    });

    it("should handle choosing all items", () => {
      expect(nCr(3, 3)).toBe(1); // 3C3 = 1
      expect(nCr(5, 5)).toBe(1); // 5C5 = 1
    });

    it("should handle choosing one item", () => {
      expect(nCr(5, 1)).toBe(5); // 5C1 = 5
      expect(nCr(10, 1)).toBe(10); // 10C1 = 10
    });

    it("should calculate larger combinations", () => {
      expect(nCr(20, 10)).toBe(184756); // 20C10 = 184,756
      expect(nCr(15, 7)).toBe(6435); // 15C7 = 6,435
    });
  });

  describe("edge cases", () => {
    it("should return NaN when n or r is 0", () => {
      expect(nCr(0, 5)).toBeNaN();
      expect(nCr(5, 0)).toBeNaN();
      expect(nCr(0, 0)).toBeNaN();
    });

    it("should return NaN when n is less than r", () => {
      expect(nCr(2, 5)).toBeNaN();
      expect(nCr(3, 4)).toBeNaN();
    });
  });
});
