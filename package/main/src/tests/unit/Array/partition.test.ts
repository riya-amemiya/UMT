import { partition } from "@/Array/partition";

describe("partition", () => {
  it("splits numbers into even and odd", () => {
    expect(partition([1, 2, 3, 4], (n) => n % 2 === 0)).toEqual([
      [2, 4],
      [1, 3],
    ]);
  });

  it("puts all elements in pass when predicate always true", () => {
    expect(partition([1, 2, 3], () => true)).toEqual([[1, 2, 3], []]);
  });

  it("puts all elements in fail when predicate always false", () => {
    expect(partition([1, 2, 3], () => false)).toEqual([[], [1, 2, 3]]);
  });

  it("returns empty tuples for empty array", () => {
    expect(partition([], () => true)).toEqual([[], []]);
  });

  it("provides index and array to the predicate", () => {
    const input = ["a", "b", "c"];
    const indices: number[] = [];
    const arrays: string[][] = [];
    partition(input, (_value, index, array) => {
      indices.push(index);
      arrays.push(array);
      return index % 2 === 0;
    });
    expect(indices).toEqual([0, 1, 2]);
    expect(arrays.every((array) => array === input)).toBe(true);
  });

  it("does not mutate the input array", () => {
    const input = [1, 2, 3, 4];
    partition(input, (n) => n > 2);
    expect(input).toEqual([1, 2, 3, 4]);
  });

  it("preserves object references", () => {
    const a = { id: 1 };
    const b = { id: 2 };
    const c = { id: 3 };
    const [pass, fail] = partition([a, b, c], (item) => item.id !== 2);
    expect(pass).toEqual([a, c]);
    expect(fail).toEqual([b]);
    expect(pass[0]).toBe(a);
    expect(fail[0]).toBe(b);
  });

  it("handles single element arrays", () => {
    expect(partition([1], (n) => n === 1)).toEqual([[1], []]);
    expect(partition([1], (n) => n === 0)).toEqual([[], [1]]);
  });

  it("handles mixed types", () => {
    const input = [0, "a", false, null, 1] as const;
    const [pass, fail] = partition([...input], (value) => Boolean(value));
    expect(pass).toEqual(["a", 1]);
    expect(fail).toEqual([0, false, null]);
  });
});
