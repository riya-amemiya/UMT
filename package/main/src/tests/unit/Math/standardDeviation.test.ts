import { standardDeviation } from "@/Math/standardDeviation";

describe("standardDeviation function", () => {
  it("should calculate standard deviation of sequential integers", () => {
    expect(standardDeviation([1, 2, 3, 4, 5])).toBe(Math.SQRT2);
    expect(standardDeviation([1, 2, 3])).toBe(0.816_496_580_927_726);
  });

  it("should calculate standard deviation of non-sequential values", () => {
    expect(standardDeviation([10, 12, 23, 23, 16, 23, 21, 16])).toBe(
      4.898_979_485_566_356,
    );
    expect(standardDeviation([5, 10, 15, 20, 25])).toBe(
      7.071_067_811_865_475_5,
    );
  });

  it("should handle edge cases", () => {
    // All same values should have standard deviation of 0
    expect(standardDeviation([5, 5, 5, 5])).toBe(0);

    // Single value should have standard deviation of 0
    expect(standardDeviation([42])).toBe(0);

    // Empty array should return 0 (as average function returns 0 for empty arrays)
    expect(standardDeviation([])).toBe(0);
  });
});
