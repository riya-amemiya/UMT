import { mode } from "@/Math/mode";

describe("mode", () => {
  it("should return the most frequent value according to JSDoc examples", () => {
    expect(mode([1, 2, 2, 3, 3, 3])).toEqual([3]);
    expect(mode([1, 2, 2, 3, 3])).toEqual([2, 3]);
    expect(mode([1, 2, 3])).toEqual([1, 2, 3]);
  });

  it("should return empty array for empty input", () => {
    expect(mode([])).toEqual([]);
  });

  it("should handle single element", () => {
    expect(mode([42])).toEqual([42]);
  });

  it("should handle negative numbers", () => {
    expect(mode([-1, -2, -2, -3])).toEqual([-2]);
  });

  it("should handle decimal numbers", () => {
    expect(mode([1.5, 2.5, 2.5, 3.5])).toEqual([2.5]);
  });

  it("should sort modes in ascending order", () => {
    expect(mode([3, 1, 3, 1, 2, 2])).toEqual([1, 2, 3]);
  });
});
