import { isObject } from "@/Validate/isObject";

describe("isObj", () => {
  it("should return true for an object with string properties", () => {
    expect(isObject({ foo: "bar", baz: "qux" })).toBe(true);
  });

  it("should return true for an object with number properties", () => {
    expect(isObject({ foo: 1, bar: 2 })).toBe(true);
  });

  it("should return true for an object with boolean properties", () => {
    expect(isObject({ foo: true, bar: false })).toBe(true);
  });

  it("should return true for an object with mixed properties", () => {
    expect(isObject({ foo: "bar", baz: 1, qux: true })).toBe(true);
  });

  it("should return false for an array", () => {
    expect(isObject([1, 2, 3])).toBe(false);
  });

  it("should return false for a string", () => {
    expect(isObject("foo")).toBe(false);
  });

  it("should return false for a number", () => {
    expect(isObject(42)).toBe(false);
  });

  it("should return false for a boolean", () => {
    expect(isObject(true)).toBe(false);
  });

  it("should return false for null", () => {
    expect(isObject(null)).toBe(false);
  });

  it("should return false for undefined", () => {
    expect(isObject(undefined)).toBe(false);
  });
});
