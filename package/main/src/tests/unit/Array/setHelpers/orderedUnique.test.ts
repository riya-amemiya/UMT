import { orderedUnique } from "@/Array/setHelpers/orderedUnique";

describe("orderedUnique", () => {
  test("keeps the first occurrence of each value in order", () => {
    expect(orderedUnique([[3, 1, 3, 2, 1]])).toEqual([3, 1, 2]);
  });

  test("scans every source in order", () => {
    expect(orderedUnique([[1, 2], new Set([2, 3]), [3, 4]])).toEqual([
      1, 2, 3, 4,
    ]);
  });

  test("returns an empty array for no sources or empty sources", () => {
    expect(orderedUnique([])).toEqual([]);
    expect(orderedUnique([[], []])).toEqual([]);
  });

  test("uses SameValueZero and keeps the first value unchanged", () => {
    const result = orderedUnique([[-0, 0, Number.NaN, Number.NaN]]);
    expect(result).toHaveLength(2);
    expect(Object.is(result[0], -0)).toBe(true);
    expect(result[1]).toBeNaN();
  });

  test("uses the key selector when given", () => {
    expect(
      orderedUnique([["apple", "avocado", "banana"]], (value) => value[0]),
    ).toEqual(["apple", "banana"]);
  });

  test("reports every duplicate value", () => {
    const duplicates: number[] = [];
    const result = orderedUnique(
      [
        [1, 2, 1],
        [2, 1],
      ],
      undefined,
      (value) => {
        duplicates.push(value);
      },
    );
    expect(result).toEqual([1, 2]);
    expect(duplicates).toEqual([1, 2, 1]);
  });

  test("reports duplicates by key", () => {
    const duplicates: number[] = [];
    orderedUnique(
      [[1.1, 1.9, 2.5]],
      (value) => Math.floor(value),
      (value) => {
        duplicates.push(value);
      },
    );
    expect(duplicates).toEqual([1.9]);
  });
});
