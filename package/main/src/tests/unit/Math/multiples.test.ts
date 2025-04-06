import { multiples } from "@/Math/multiples";

describe("multiples function", () => {
  it("should generate multiples of 1", () => {
    expect(multiples(1, 1)).toEqual([1]);
    expect(multiples(1, 3)).toEqual([1, 2, 3]);
    expect(multiples(1, 5)).toEqual([1, 2, 3, 4, 5]);
  });

  it("should generate multiples of 2", () => {
    expect(multiples(2, 1)).toEqual([2]);
    expect(multiples(2, 3)).toEqual([2, 4, 6]);
    expect(multiples(2, 5)).toEqual([2, 4, 6, 8, 10]);
  });

  it("should generate multiples of negative numbers", () => {
    expect(multiples(-2, 3)).toEqual([-2, -4, -6]);
    expect(multiples(-3, 4)).toEqual([-3, -6, -9, -12]);
  });

  it("should handle zero as base number", () => {
    expect(multiples(0, 3)).toEqual([0, 0, 0]);
  });

  it("should return empty array for zero count", () => {
    expect(multiples(2, 0)).toEqual([]);
    expect(multiples(-2, 0)).toEqual([]);
  });

  it("should handle decimal numbers", () => {
    expect(multiples(0.5, 3)).toEqual([0.5, 1, 1.5]);
    expect(multiples(1.5, 3)).toEqual([1.5, 3, 4.5]);
  });
});
