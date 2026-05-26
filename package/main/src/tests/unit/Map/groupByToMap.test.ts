import { groupByToMap } from "@/Map/groupByToMap";

describe("groupByToMap", () => {
  it("should group numbers into odd and even", () => {
    const array = [1, 2, 3, 4, 5];
    const result = groupByToMap(array, (num) =>
      num % 2 === 0 ? "even" : "odd",
    );
    expect(result).toEqual(
      new Map<string, number[]>([
        ["odd", [1, 3, 5]],
        ["even", [2, 4]],
      ]),
    );
  });

  it("should preserve insertion order of keys", () => {
    const array = [3, 1, 2, 1, 3, 2];
    const result = groupByToMap(array, (num) => num);
    expect([...result.keys()]).toEqual([3, 1, 2]);
  });

  it("should accept object keys", () => {
    const a = { id: "a" };
    const b = { id: "b" };
    const array = [
      { tag: a, value: 1 },
      { tag: b, value: 2 },
      { tag: a, value: 3 },
    ];
    const result = groupByToMap(array, (item) => item.tag);
    expect(result.get(a)).toEqual([
      { tag: a, value: 1 },
      { tag: a, value: 3 },
    ]);
    expect(result.get(b)).toEqual([{ tag: b, value: 2 }]);
  });

  it("should group strings based on their length", () => {
    const array = ["one", "two", "three", "four", "five"];
    const result = groupByToMap(array, (str) => str.length);
    expect(result).toEqual(
      new Map<number, string[]>([
        [3, ["one", "two"]],
        [5, ["three"]],
        [4, ["four", "five"]],
      ]),
    );
  });

  it("should pass index and array to the iteratee", () => {
    const array = ["a", "b", "c", "d"];
    const receivedIndices: number[] = [];
    groupByToMap(array, (_value, index, source) => {
      expect(source).toBe(array);
      receivedIndices.push(index);
      return index % 2;
    });
    expect(receivedIndices).toEqual([0, 1, 2, 3]);
  });

  it("should return an empty Map for an empty array", () => {
    const result = groupByToMap<number, string>([], (num) =>
      num % 2 === 0 ? "even" : "odd",
    );
    expect(result.size).toBe(0);
  });
});
