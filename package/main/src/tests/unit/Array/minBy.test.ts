import { minBy } from "@/Array/minBy";

describe("minBy", () => {
  test("returns the element with the smallest selected value", () => {
    expect(minBy([{ n: 3 }, { n: 1 }, { n: 2 }], (item) => item.n)).toEqual({
      n: 1,
    });
  });

  test("returns undefined for an empty array", () => {
    expect(minBy([], (n: number) => n)).toBeUndefined();
  });

  test("returns the only element of a single-element array", () => {
    expect(minBy([7], (n) => n)).toBe(7);
  });

  test("keeps the first element on ties", () => {
    const first = { n: 1, id: "first" };
    const second = { n: 1, id: "second" };
    expect(minBy([{ n: 2, id: "x" }, first, second], (item) => item.n)).toBe(
      first,
    );
  });

  test("compares strings", () => {
    expect(minBy(["pear", "apple", "fig"], (s) => s)).toBe("apple");
  });

  test("never replaces the current minimum with a NaN key", () => {
    expect(minBy([2, Number.NaN, 1], (n) => n)).toBe(1);
    expect(minBy([Number.NaN, 1], (n) => n)).toBeNaN();
  });
});
