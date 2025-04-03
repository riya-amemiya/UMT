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
});
