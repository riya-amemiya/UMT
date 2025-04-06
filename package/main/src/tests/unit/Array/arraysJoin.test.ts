import { arraysJoin } from "@/Array/arraysJoin";

describe("arraysJoin", () => {
  test("should join arrays without duplicates", () => {
    // Basic array joining
    expect(arraysJoin([1, 2, 3], [4, 5, 6])).toEqual([1, 2, 3, 4, 5, 6]);

    // Multiple arrays joining
    expect(arraysJoin([1, 2, 3], [4, 5, 6], [7, 8, 9])).toEqual([
      1, 2, 3, 4, 5, 6, 7, 8, 9,
    ]);

    // Arrays with duplicates
    expect(arraysJoin([1, 2, 3], [2, 3, 4], [3, 4, 5])).toEqual([
      1, 2, 3, 4, 5,
    ]);

    // Empty array cases
    expect(arraysJoin([], [1, 2, 3])).toEqual([1, 2, 3]);
    expect(arraysJoin([1, 2, 3], [])).toEqual([1, 2, 3]);
  });
});
