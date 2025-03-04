import { average } from "@/Math/average";
describe("average function", () => {
  describe("basic scenarios", () => {
    it("should calculate average of integer numbers", () => {
      expect(average([1, 2])).toBe(1.5);
      expect(average([1, 2, 3, 4, 5])).toBe(3);
    });

    it("should calculate average of decimal numbers", () => {
      expect(average([1.1, 2.2])).toBeCloseTo(1.65);
      expect(average([1.1, 2.2, 3.3, 4.4, 5.5])).toBeCloseTo(3.3);
    });

    it("should handle single element arrays", () => {
      expect(average([5])).toBe(5);
      expect(average([3.14])).toBe(3.14);
    });
  });

  describe("special cases", () => {
    it("should handle arrays with zeros", () => {
      expect(average([0, 0, 0])).toBe(0);
      expect(average([1, 0, 2, 0, 3])).toBe(1.2);
    });

    it("should handle negative numbers", () => {
      expect(average([-1, -2])).toBe(-1.5);
      expect(average([-10, -20, -30])).toBe(-20);
    });

    it("should handle mixed positive and negative numbers", () => {
      expect(average([-1, 1])).toBe(0);
      expect(average([-10, 10, -5, 5])).toBe(0);
    });

    it("should return 0 for empty arrays", () => {
      expect(average([])).toBe(0);
    });
  });

  describe("edge cases", () => {
    it("should handle large numbers", () => {
      expect(average([Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER]))
        .toBe(Number.MAX_SAFE_INTEGER);
    });

    it("should handle small numbers", () => {
      expect(average([Number.MIN_SAFE_INTEGER, Number.MIN_SAFE_INTEGER]))
        .toBe(Number.MIN_SAFE_INTEGER);
    });

    it("should handle large arrays", () => {
      const largeArray = new Array(10000).fill(1);
      expect(average(largeArray)).toBe(1);
    });
  });
});
