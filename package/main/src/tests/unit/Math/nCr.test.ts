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
      expect(nCr(20, 10)).toBe(184_756); // 20C10 = 184,756
      expect(nCr(15, 7)).toBe(6435); // 15C7 = 6,435
    });
  });

  describe("edge cases", () => {
    it("should return 1 when r is 0 (choosing nothing)", () => {
      expect(nCr(5, 0)).toBe(1);
      expect(nCr(10, 0)).toBe(1);
    });

    it("should return NaN for invalid inputs", () => {
      expect(nCr(0, 5)).toBeNaN();
      expect(nCr(2, 5)).toBeNaN();
      expect(nCr(3, 4)).toBeNaN();
      expect(nCr(-1, 2)).toBeNaN();
      expect(nCr(5, -1)).toBeNaN();
    });

    it("should return 1 when n equals 0 and r equals 0", () => {
      expect(nCr(0, 0)).toBe(1);
    });
  });

  describe("large combinations", () => {
    it("should stay finite when the result fits in a double", () => {
      expect(nCr(1000, 500)).toBe(2.702_882_409_454_366_6e299);
      expect(nCr(400, 170)).toBe(1.137_233_204_157_519_8e117);
      expect(nCr(100, 97)).toBe(161_700);
    });

    it("should stay finite when an intermediate product would overflow", () => {
      expect(nCr(1029, 514)).toBe(1.429_820_686_498_903e308);
    });

    it("should return Infinity when the result exceeds Number.MAX_VALUE", () => {
      expect(nCr(1030, 515)).toBe(Number.POSITIVE_INFINITY);
      expect(nCr(1e10, 5e9)).toBe(Number.POSITIVE_INFINITY);
    });
  });
});
