import { isDictionaryObject } from "@/Validate/isDictionaryObject";

describe("isDictionaryObject", () => {
  it("should return true for an empty object", () => {
    expect(isDictionaryObject({})).toBe(true);
  });

  it("should return true for an object with string properties", () => {
    expect(isDictionaryObject({ foo: "bar", baz: "qux" })).toBe(true);
  });

  it("should work as a type guard", () => {
    const value: unknown = { x: 1, y: 2 };
    if (isDictionaryObject<{ x: number; y: number }>(value)) {
      // TypeScript should recognize value as { x: number; y: number }
      const sum = value.x + value.y;
      expect(sum).toBe(3);
    }
  });

  it("should return true for an object with number properties", () => {
    expect(isDictionaryObject({ foo: 1, bar: 2 })).toBe(true);
  });

  it("should return true for an object with boolean properties", () => {
    expect(isDictionaryObject({ foo: true, bar: false })).toBe(true);
  });

  it("should return true for an object with mixed properties", () => {
    expect(isDictionaryObject({ foo: "bar", baz: 1, qux: true })).toBe(true);
  });

  it("should return false for an array", () => {
    expect(isDictionaryObject([1, 2, 3])).toBe(false);
  });

  it("should return false for a string", () => {
    expect(isDictionaryObject("foo")).toBe(false);
  });

  it("should return false for a number", () => {
    expect(isDictionaryObject(42)).toBe(false);
  });

  it("should return false for a boolean", () => {
    expect(isDictionaryObject(true)).toBe(false);
  });

  it("should return false for null", () => {
    expect(isDictionaryObject(null)).toBe(false);
  });

  it("should return false for undefined", () => {
    expect(isDictionaryObject(undefined)).toBe(false);
  });
});
