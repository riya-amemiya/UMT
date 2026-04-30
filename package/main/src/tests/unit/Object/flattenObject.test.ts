import { flattenObject } from "@/Object/flattenObject";

describe("flattenObject", () => {
  it("flattens a nested object using dot separator", () => {
    expect(flattenObject({ a: { b: { c: 1 } } })).toEqual({ "a.b.c": 1 });
  });

  it("preserves top-level keys", () => {
    expect(flattenObject({ a: 1, b: 2 })).toEqual({ a: 1, b: 2 });
  });

  it("supports custom separator", () => {
    expect(flattenObject({ a: { b: 1 } }, "/")).toEqual({ "a/b": 1 });
  });

  it("keeps arrays as values", () => {
    expect(flattenObject({ a: { b: [1, 2] } })).toEqual({ "a.b": [1, 2] });
  });

  it("treats empty plain objects as leaf values", () => {
    expect(flattenObject({ a: {} })).toEqual({ a: {} });
  });

  it("keeps non-plain objects as values", () => {
    const map = new Map();
    expect(flattenObject({ a: { b: map } })).toEqual({ "a.b": map });
  });

  it("returns empty for empty input", () => {
    expect(flattenObject({})).toEqual({});
  });
});
