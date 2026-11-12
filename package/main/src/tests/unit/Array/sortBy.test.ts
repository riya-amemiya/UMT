import { sortBy } from "@/Array/sortBy";

describe("sortBy", () => {
  test("sorts by a single selector", () => {
    expect(
      sortBy([{ age: 30 }, { age: 20 }, { age: 25 }], (user) => user.age),
    ).toEqual([{ age: 20 }, { age: 25 }, { age: 30 }]);
  });

  test("uses later selectors to break ties", () => {
    expect(
      sortBy(
        ["bb", "a", "ab", "c"],
        (s) => s.length,
        (s) => s,
      ),
    ).toEqual(["a", "c", "ab", "bb"]);
  });

  test("is stable for equal keys", () => {
    const items = [
      { key: 1, id: "a" },
      { key: 0, id: "b" },
      { key: 1, id: "c" },
      { key: 0, id: "d" },
    ];
    expect(sortBy(items, (item) => item.key).map((item) => item.id)).toEqual([
      "b",
      "d",
      "a",
      "c",
    ]);
  });

  test("is stable for large inputs that need merging", () => {
    const items = Array.from({ length: 200 }, (_, index) => ({
      key: index % 3,
      index,
    }));
    const result = sortBy(items, (item) => item.key);
    for (let index = 1; index < result.length; index += 1) {
      const previous = result[index - 1];
      const current = result[index];
      expect(
        previous.key < current.key ||
          (previous.key === current.key && previous.index < current.index),
      ).toBe(true);
    }
  });

  test("returns an unchanged copy without selectors", () => {
    const source = [3, 1, 2];
    const result = sortBy(source);
    expect(result).toEqual([3, 1, 2]);
    expect(result).not.toBe(source);
  });

  test("handles empty and single-element arrays", () => {
    expect(sortBy([], (n: number) => n)).toEqual([]);
    expect(sortBy([1], (n) => n)).toEqual([1]);
  });

  test("calls each selector once per element", () => {
    let calls = 0;
    sortBy([5, 3, 4, 1, 2], (n) => {
      calls += 1;
      return n;
    });
    expect(calls).toBe(5);
  });

  test("does not mutate the input", () => {
    const source = [3, 1, 2];
    sortBy(source, (n) => n);
    expect(source).toEqual([3, 1, 2]);
  });

  test("supports descending order through a negated key", () => {
    expect(sortBy([1, 3, 2], (n) => -n)).toEqual([3, 2, 1]);
  });
});
