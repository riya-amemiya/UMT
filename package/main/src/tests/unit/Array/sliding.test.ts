import { sliding } from "@/Array/sliding";

describe("sliding", () => {
  it("returns overlapping windows with default step of 1", () => {
    expect(sliding([1, 2, 3, 4, 5], 3)).toEqual([
      [1, 2, 3],
      [2, 3, 4],
      [3, 4, 5],
    ]);
  });

  it("respects a custom step", () => {
    expect(sliding([1, 2, 3, 4, 5], 3, 2)).toEqual([
      [1, 2, 3],
      [3, 4, 5],
    ]);
  });

  it("behaves like non-overlapping chunks when step equals size", () => {
    expect(sliding([1, 2, 3, 4, 5, 6], 2, 2)).toEqual([
      [1, 2],
      [3, 4],
      [5, 6],
    ]);
  });

  it("omits incomplete trailing windows", () => {
    expect(sliding([1, 2, 3, 4, 5], 3, 3)).toEqual([[1, 2, 3]]);
  });

  it("returns empty array when size is larger than length", () => {
    expect(sliding([1, 2], 5)).toEqual([]);
  });

  it("returns empty array for empty input", () => {
    expect(sliding([], 2)).toEqual([]);
  });

  it("handles size of 1", () => {
    expect(sliding([1, 2, 3], 1)).toEqual([[1], [2], [3]]);
  });

  it("handles size equal to array length", () => {
    expect(sliding([1, 2, 3], 3)).toEqual([[1, 2, 3]]);
  });

  it("does not mutate the input array", () => {
    const input = [1, 2, 3, 4];
    sliding(input, 2);
    expect(input).toEqual([1, 2, 3, 4]);
  });

  it("preserves object references", () => {
    const a = { id: 1 };
    const b = { id: 2 };
    const c = { id: 3 };
    const result = sliding([a, b, c], 2);
    expect(result).toEqual([
      [a, b],
      [b, c],
    ]);
    expect(result[0][0]).toBe(a);
    expect(result[1][1]).toBe(c);
  });

  it("handles string arrays", () => {
    expect(sliding(["a", "b", "c", "d"], 2, 1)).toEqual([
      ["a", "b"],
      ["b", "c"],
      ["c", "d"],
    ]);
  });

  it("handles single element with size 1", () => {
    expect(sliding([42], 1)).toEqual([[42]]);
  });
});
