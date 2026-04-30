import { get } from "@/Object/get";

describe("get", () => {
  it("reads a top-level property", () => {
    expect(get({ a: 1 }, "a")).toBe(1);
  });

  it("reads a nested property by dotted path", () => {
    expect(get({ a: { b: { c: 42 } } }, "a.b.c")).toBe(42);
  });

  it("reads a nested property by array path", () => {
    expect(get({ a: { b: { c: 42 } } }, ["a", "b", "c"])).toBe(42);
  });

  it("returns the default when a segment is missing", () => {
    expect(get({ a: { b: 1 } }, "a.x", "fallback")).toBe("fallback");
  });

  it("returns the default when traversing through null", () => {
    expect(get({ a: null }, "a.b", "fallback")).toBe("fallback");
  });

  it("returns undefined when no default and missing", () => {
    expect(get({ a: 1 }, "x")).toBeUndefined();
  });

  it("returns the default when the resolved value is undefined", () => {
    expect(get({ a: undefined }, "a", 0)).toBe(0);
  });

  it("returns null when the value is explicitly null", () => {
    expect(get({ a: null }, "a", "fallback")).toBeNull();
  });

  it("handles object root being null", () => {
    expect(get(null, "a.b", 7)).toBe(7);
  });
});
