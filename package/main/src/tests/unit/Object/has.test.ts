import { has } from "@/Object/has";

describe("has", () => {
  it("should return true for existing nested path (string)", () => {
    expect(has({ a: { b: 1 } }, "a.b")).toBe(true);
  });

  it("should return true for existing nested path (array)", () => {
    expect(has({ a: { b: 1 } }, ["a", "b"])).toBe(true);
  });

  it("should return false for non-existing nested path (string)", () => {
    expect(has({ a: { b: 1 } }, "a.c")).toBe(false);
  });

  it("should return false for non-existing nested path (array)", () => {
    expect(has({ a: { b: 1 } }, ["a", "c"])).toBe(false);
  });

  it("should return false for non-existing top-level path", () => {
    expect(has({ a: { b: 1 } }, "b")).toBe(false);
  });

  it("should handle empty path (string)", () => {
    expect(has({ a: { b: 1 } }, "")).toBe(false);
  });

  it("should handle empty path (array)", () => {
    expect(has({ a: { b: 1 } }, [])).toBe(true);
  });

  it("should handle null or undefined object", () => {
    // @ts-expect-error
    expect(has(null, "a.b")).toBe(false);
    // @ts-expect-error
    expect(has(undefined, "a.b")).toBe(false);
  });
});
