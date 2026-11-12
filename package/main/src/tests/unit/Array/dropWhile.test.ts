import { dropWhile } from "@/Array/dropWhile";

describe("dropWhile", () => {
  test("drops leading elements while the predicate holds", () => {
    expect(dropWhile([1, 2, 3, 4, 1], (n) => n < 3)).toEqual([3, 4, 1]);
  });

  test("returns an empty array when every element passes", () => {
    expect(dropWhile([1, 2, 3], () => true)).toEqual([]);
  });

  test("returns a copy of the whole array when the first element fails", () => {
    const source = [5, 1, 2];
    const result = dropWhile(source, (n) => n < 3);
    expect(result).toEqual([5, 1, 2]);
    expect(result).not.toBe(source);
  });

  test("returns an empty array for an empty input", () => {
    expect(dropWhile([], () => false)).toEqual([]);
  });

  test("passes value, index and array to the predicate", () => {
    const source = ["a", "b", "c"];
    const calls: [string, number, string[]][] = [];
    dropWhile(source, (value, index, array) => {
      calls.push([value, index, array]);
      return index < 1;
    });
    expect(calls).toEqual([
      ["a", 0, source],
      ["b", 1, source],
    ]);
    expect(calls[1][2]).toBe(source);
  });

  test("does not mutate the input", () => {
    const source = [1, 2, 3];
    dropWhile(source, (n) => n < 2);
    expect(source).toEqual([1, 2, 3]);
  });
});
