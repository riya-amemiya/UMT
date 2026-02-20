import { percentile } from "@/Math/percentile";

describe("percentile", () => {
  it("should calculate percentiles according to JSDoc examples", () => {
    expect(percentile([1, 2, 3, 4, 5], 50)).toBe(3);
    expect(percentile([1, 2, 3, 4, 5], 25)).toBe(2);
    expect(percentile([1, 2, 3, 4, 5], 75)).toBe(4);
  });

  it("should calculate 0th and 100th percentiles", () => {
    expect(percentile([1, 2, 3, 4, 5], 0)).toBe(1);
    expect(percentile([1, 2, 3, 4, 5], 100)).toBe(5);
  });

  it("should handle single element array", () => {
    expect(percentile([42], 50)).toBe(42);
    expect(percentile([42], 0)).toBe(42);
    expect(percentile([42], 100)).toBe(42);
  });

  it("should return NaN for empty array", () => {
    expect(percentile([], 50)).toBeNaN();
  });

  it("should handle unsorted arrays", () => {
    expect(percentile([5, 1, 3, 2, 4], 50)).toBe(3);
  });

  it("should handle negative numbers", () => {
    expect(percentile([-5, -3, -1, 1, 3], 50)).toBe(-1);
  });

  it("should handle duplicate values", () => {
    expect(percentile([1, 1, 2, 2, 3, 3], 50)).toBe(2);
  });
});
