import { nHr } from "@/Math/nHr";

describe("nHr function", () => {
  describe("valid combinations with repetition", () => {
    it("should calculate basic combinations with repetition correctly", () => {
      expect(nHr(5, 2)).toBe(15); // 5H2 = 15
      expect(nHr(3, 3)).toBe(10); // 3H3 = 10
    });

    it("should handle choosing one item", () => {
      expect(nHr(3, 1)).toBe(3); // 3H1 = 3
      expect(nHr(5, 1)).toBe(5); // 5H1 = 5
    });

    it("should handle choosing same as total items", () => {
      expect(nHr(2, 2)).toBe(3); // 2H2 = 3
      expect(nHr(4, 4)).toBe(35); // 4H4 = 35
    });

    it("should handle larger combinations", () => {
      expect(nHr(10, 3)).toBe(220); // 10H3 = 220
      expect(nHr(6, 4)).toBe(126); // 6H4 = 126
    });
  });

  describe("edge cases", () => {
    it("should return NaN when n or r is 0", () => {
      expect(nHr(0, 5)).toBeNaN();
      expect(nHr(5, 0)).toBeNaN();
      expect(nHr(0, 0)).toBeNaN();
    });

    it("should return NaN when n or r is negative", () => {
      expect(nHr(-1, 5)).toBeNaN();
      expect(nHr(5, -1)).toBeNaN();
      expect(nHr(-1, -1)).toBeNaN();
    });
  });
});
