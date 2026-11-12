import { maxBy } from "@/Array/maxBy";

describe("maxBy", () => {
  test("returns the element with the largest selected value", () => {
    expect(maxBy([{ n: 1 }, { n: 3 }, { n: 2 }], (item) => item.n)).toEqual({
      n: 3,
    });
  });

  test("returns undefined for an empty array", () => {
    expect(maxBy([], (n: number) => n)).toBeUndefined();
  });

  test("returns the only element of a single-element array", () => {
    expect(maxBy([7], (n) => n)).toBe(7);
  });

  test("keeps the first element on ties", () => {
    const first = { n: 2, id: "first" };
    const second = { n: 2, id: "second" };
    expect(maxBy([{ n: 1, id: "x" }, first, second], (item) => item.n)).toBe(
      first,
    );
  });

  test("compares strings", () => {
    expect(maxBy(["pear", "apple", "fig"], (s) => s)).toBe("pear");
  });

  test("never replaces the current maximum with a NaN key", () => {
    expect(maxBy([1, Number.NaN, 2], (n) => n)).toBe(2);
    expect(maxBy([Number.NaN, 1], (n) => n)).toBeNaN();
  });
});
