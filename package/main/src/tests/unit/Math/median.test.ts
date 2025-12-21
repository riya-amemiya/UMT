import { median } from "@/Math/median";

describe("median", () => {
  it("should calculate median of array with even number of elements", () => {
    expect(median([1, 2, 3, 4])).toBe(2.5);
  });

  it("should calculate median of array with odd number of elements", () => {
    expect(median([1, 3, 3, 6, 7, 8, 9])).toBe(6);
  });

  it("should return NaN for empty array", () => {
    expect(median([])).toBeNaN();
  });

  it("should calculate median of unsorted array", () => {
    expect(median([9, 1, 5, 3, 6])).toBe(5);
  });

  it("should handle single element array", () => {
    expect(median([5])).toBe(5);
    expect(median([-10])).toBe(-10);
  });

  it("should handle two element array", () => {
    expect(median([1, 3])).toBe(2);
    expect(median([10, 20])).toBe(15);
  });

  describe("special number values", () => {
    it("should handle Infinity in array", () => {
      expect(median([1, 2, Number.POSITIVE_INFINITY])).toBe(2);
      expect(median([Number.NEGATIVE_INFINITY, 0, 1])).toBe(0);
    });

    it("should handle array with all same values", () => {
      expect(median([5, 5, 5, 5])).toBe(5);
    });

    it("should handle negative numbers", () => {
      expect(median([-5, -3, -1, 0, 2])).toBe(-1);
    });
  });
});
