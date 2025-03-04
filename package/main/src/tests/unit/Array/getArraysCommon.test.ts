import { getArraysCommon } from "@/Array/getArraysCommon";
describe("getArraysCommon", () => {
  it("should find common elements between arrays", () => {
    // Test with two arrays
    expect(getArraysCommon([1, 2, 3], [2, 3, 4])).toEqual([2, 3]);

    // Test with three arrays
    expect(getArraysCommon([1, 2, 3], [2, 3, 9], [3, 4, 5])).toEqual([3]);

    // Test with four arrays (including duplicates)
    expect(getArraysCommon([1, 2, 3], [2, 3, 9], [3, 4, 5], [3, 4, 5])).toEqual(
      [3],
    );
  });
});
