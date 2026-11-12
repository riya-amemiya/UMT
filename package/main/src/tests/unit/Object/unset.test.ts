import { unset } from "@/Object/unset";

describe("unset", () => {
  it("removes a top-level property", () => {
    expect(unset({ a: 1, b: 2 }, "a")).toEqual({ b: 2 });
  });

  it("removes a nested property", () => {
    expect(unset({ a: { b: 1, c: 2 } }, "a.b")).toEqual({ a: { c: 2 } });
  });

  it("accepts an array path", () => {
    expect(unset({ a: { b: 1, c: 2 } }, ["a", "c"])).toEqual({ a: { b: 1 } });
  });

  it("does not mutate the input", () => {
    const source = { a: { b: 1, c: 2 }, d: { e: 3 } };
    const result = unset(source, "a.b");
    expect(source).toEqual({ a: { b: 1, c: 2 }, d: { e: 3 } });
    expect(result).not.toBe(source);
    expect(result.a).not.toBe(source.a);
    expect(result.d).toBe(source.d);
  });

  it("returns a shallow copy when the path is missing", () => {
    const source = { a: { b: 1 } };
    const result = unset(source, "a.x.y");
    expect(result).toEqual(source);
    expect(result).not.toBe(source);
  });

  it("returns a shallow copy when the path array is empty", () => {
    const source = { a: 1 };
    const result = unset(source, []);
    expect(result).toEqual({ a: 1 });
    expect(result).not.toBe(source);
  });

  it("removes an array element and shifts the rest", () => {
    const source = { a: [1, 2, 3] };
    expect(unset(source, "a.1")).toEqual({ a: [1, 3] });
    expect(source.a).toEqual([1, 2, 3]);
  });

  it("traverses through array elements", () => {
    expect(unset({ a: [{ b: 1, c: 2 }] }, "a.0.b")).toEqual({
      a: [{ c: 2 }],
    });
  });

  it("ignores non-canonical array indexes", () => {
    expect(unset({ a: [1, 2] }, "a.01")).toEqual({ a: [1, 2] });
    expect(unset({ a: [1, 2] }, "a.length")).toEqual({ a: [1, 2] });
  });

  it("ignores out-of-range array indexes", () => {
    expect(unset({ a: [1, 2] }, "a.5")).toEqual({ a: [1, 2] });
  });

  it("stops at primitive values", () => {
    expect(unset({ a: "text" }, "a.length")).toEqual({ a: "text" });
  });

  it("stops at non-plain objects", () => {
    const date = new Date(0);
    const result = unset({ a: date }, "a.getTime");
    expect(result.a).toBe(date);
  });

  it("returns a non-plain root unchanged", () => {
    const map = new Map([["a", 1]]);
    expect(unset(map, "a")).toBe(map);
  });

  it("removes a top-level element of an array root", () => {
    expect(unset([1, 2, 3], "0")).toEqual([2, 3]);
  });

  it("does not inherit keys from the prototype", () => {
    expect(unset({ a: 1 }, "toString")).toEqual({ a: 1 });
  });
});
