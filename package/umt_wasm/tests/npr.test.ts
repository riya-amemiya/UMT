import { npr } from "../src/umt_plugin_wasm";

describe("npr function", () => {
  describe("valid permutations", () => {
    it("should calculate basic permutations correctly", () => {
      expect(npr(5, 2)).toBe(20); // 5P2 = 20
      expect(npr(10, 4)).toBe(5040); // 10P4 = 5040
    });

    it("should handle choosing one item", () => {
      expect(npr(3, 1)).toBe(3); // 3P1 = 3
      expect(npr(5, 1)).toBe(5); // 5P1 = 5
    });

    it("should handle choosing all items", () => {
      expect(npr(3, 3)).toBe(6); // 3P3 = 6 (factorial of 3)
      expect(npr(4, 4)).toBe(24); // 4P4 = 24 (factorial of 4)
    });

    it("should handle larger permutations", () => {
      expect(npr(8, 3)).toBe(336); // 8P3 = 336
      expect(npr(6, 4)).toBe(360); // 6P4 = 360
    });
  });

  describe("edge cases", () => {
    it("should return 1 when r is 0 (arranging nothing)", () => {
      expect(npr(5, 0)).toBe(1);
      expect(npr(10, 0)).toBe(1);
      expect(npr(0, 0)).toBe(1);
    });

    it("should return NaN for invalid inputs", () => {
      expect(npr(0, 5)).toBeNaN();
      expect(npr(2, 5)).toBeNaN();
      expect(npr(3, 4)).toBeNaN();
      expect(npr(-1, 2)).toBeNaN();
      expect(npr(5, -1)).toBeNaN();
    });

    it("should calculate correctly when n equals r", () => {
      expect(npr(2, 2)).toBe(2); // 2P2 = 2
      expect(npr(5, 5)).toBe(120); // 5P5 = 120
    });
  });
});
