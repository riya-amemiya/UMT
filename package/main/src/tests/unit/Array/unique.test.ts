import { unique } from "@/Array/unique";

describe("unique", () => {
  test("should remove duplicate values from array", () => {
    expect(unique([1, 2, 2, 3, 3, 3, 4])).toEqual([1, 2, 3, 4]);
  });

  test("should handle empty array", () => {
    expect(unique([])).toEqual([]);
  });

  test("should handle array with no duplicates", () => {
    expect(unique([1, 2, 3, 4])).toEqual([1, 2, 3, 4]);
  });

  test("should work with strings", () => {
    expect(unique(["a", "b", "b", "c", "a"])).toEqual(["a", "b", "c"]);
  });

  test("should handle array with all same values", () => {
    expect(unique([1, 1, 1, 1])).toEqual([1]);
  });

  test("should preserve order of first occurrence", () => {
    expect(unique([3, 1, 2, 1, 3, 2])).toEqual([3, 1, 2]);
  });

  test("should work with mixed types", () => {
    expect(unique([1, "a", 1, "a", true, true, null, null])).toEqual([
      1,
      "a",
      true,
      null,
    ]);
  });

  test("should handle boolean values", () => {
    expect(unique([true, false, true, false, true])).toEqual([true, false]);
  });

  test("should handle special values", () => {
    expect(unique([0, null, undefined, "", false, 0, null])).toEqual([
      0,
      null,
      undefined,
      "",
      false,
    ]);
  });
});
