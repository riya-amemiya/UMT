import { countBy } from "@/Array/countBy";

describe("countBy", () => {
  it("counts numbers by floor", () => {
    expect(countBy([6.1, 4.2, 6.3], Math.floor)).toEqual({ 4: 1, 6: 2 });
  });

  it("counts strings by length", () => {
    expect(countBy(["one", "two", "three"], (str) => str.length)).toEqual({
      3: 2,
      5: 1,
    });
  });

  it("counts objects by property", () => {
    const array: { type: "fruit" | "vegetable" }[] = [
      { type: "fruit" },
      { type: "vegetable" },
      { type: "fruit" },
    ];
    expect(countBy(array, (item) => item.type)).toEqual({
      fruit: 2,
      vegetable: 1,
    });
  });

  it("returns empty object for empty array", () => {
    expect(countBy([], (n: number) => n)).toEqual({});
  });

  it("counts all under one key when iteratee is constant", () => {
    expect(countBy([1, 2, 3], () => "all")).toEqual({ all: 3 });
  });

  it("provides index and array to the iteratee", () => {
    const input = ["a", "b", "c"];
    const indices: number[] = [];
    countBy(input, (_value, index, array) => {
      indices.push(index);
      expect(array).toBe(input);
      return index;
    });
    expect(indices).toEqual([0, 1, 2]);
  });

  it("handles numeric keys", () => {
    expect(countBy([1, 2, 1, 3, 2, 1], (n) => n)).toEqual({
      1: 3,
      2: 2,
      3: 1,
    });
  });

  it("handles boolean-derived string keys", () => {
    expect(
      countBy([true, false, true], (bool) => (bool ? "yes" : "no")),
    ).toEqual({ yes: 2, no: 1 });
  });

  it("handles single element array", () => {
    expect(countBy(["x"], (s) => s)).toEqual({ x: 1 });
  });
});
