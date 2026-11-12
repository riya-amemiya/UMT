import { takeWhile } from "@/Array/takeWhile";

describe("takeWhile", () => {
  test("takes leading elements while the predicate holds", () => {
    expect(takeWhile([1, 2, 3, 4, 1], (n) => n < 3)).toEqual([1, 2]);
  });

  test("returns a copy of the whole array when every element passes", () => {
    const source = [1, 2, 3];
    const result = takeWhile(source, () => true);
    expect(result).toEqual([1, 2, 3]);
    expect(result).not.toBe(source);
  });

  test("returns an empty array when the first element fails", () => {
    expect(takeWhile([5, 1, 2], (n) => n < 3)).toEqual([]);
  });

  test("returns an empty array for an empty input", () => {
    expect(takeWhile([], () => true)).toEqual([]);
  });

  test("passes value, index and array to the predicate", () => {
    const source = ["a", "b", "c"];
    const calls: [string, number, string[]][] = [];
    takeWhile(source, (value, index, array) => {
      calls.push([value, index, array]);
      return index < 1;
    });
    expect(calls).toEqual([
      ["a", 0, source],
      ["b", 1, source],
    ]);
    expect(calls[0][2]).toBe(source);
  });

  test("does not mutate the input", () => {
    const source = [1, 2, 3];
    takeWhile(source, (n) => n < 2);
    expect(source).toEqual([1, 2, 3]);
  });
});
