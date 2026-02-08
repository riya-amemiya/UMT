import { sumPrecise } from "@/Math/sumPrecise";

describe("sumPrecise", () => {
  it("should sum simple integers", () => {
    expect(sumPrecise([1, 2, 3, 4, 5])).toBe(15);
  });

  it("should handle floating point precision", () => {
    expect(sumPrecise([0.1, 0.2, 0.3])).toBeCloseTo(0.6, 15);
  });

  it("should handle large and small number cancellation", () => {
    expect(sumPrecise([1e20, 1, -1e20])).toBe(1);
  });

  it("should return 0 for an empty array", () => {
    expect(sumPrecise([])).toBe(0);
  });

  it("should return the single element for a one-element array", () => {
    expect(sumPrecise([42])).toBe(42);
  });

  it("should handle all negative numbers", () => {
    expect(sumPrecise([-1, -2, -3])).toBe(-6);
  });

  it("should handle mixed positive and negative numbers", () => {
    expect(sumPrecise([10, -3, 5, -2])).toBe(10);
  });

  it("should handle many small floating point values", () => {
    const arr = Array.from({ length: 1000 }, () => 0.001);
    expect(sumPrecise(arr)).toBeCloseTo(1, 10);
  });
});
