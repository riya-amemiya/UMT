import { set } from "@/Object/set";

describe("set", () => {
  it("sets a top-level property", () => {
    const target: Record<string, unknown> = {};
    set(target, "a", 1);
    expect(target).toEqual({ a: 1 });
  });

  it("creates nested objects when missing", () => {
    const target: Record<string, unknown> = {};
    set(target, "a.b.c", 1);
    expect(target).toEqual({ a: { b: { c: 1 } } });
  });

  it("respects array path input", () => {
    const target: Record<string, unknown> = {};
    set(target, ["a", "b"], 2);
    expect(target).toEqual({ a: { b: 2 } });
  });

  it("overwrites existing leaf values", () => {
    const target: Record<string, unknown> = { a: { b: 1 } };
    set(target, "a.b", 2);
    expect(target).toEqual({ a: { b: 2 } });
  });

  it("replaces non-object intermediate values", () => {
    const target: Record<string, unknown> = { a: 5 };
    set(target, "a.b", 1);
    expect(target).toEqual({ a: { b: 1 } });
  });

  it("returns the same object reference", () => {
    const target = {};
    expect(set(target, "a", 1)).toBe(target);
  });

  it("returns the object unchanged when path is empty", () => {
    const target = { a: 1 };
    set(target, [], 2);
    expect(target).toEqual({ a: 1 });
  });
});
