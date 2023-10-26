import { isObj } from "@/Tool/isObj";

describe("isObj", () => {
  it("should return true for an object with string properties", () => {
    expect(isObj({ foo: "bar", baz: "qux" })).toBe(true);
  });

  it("should return true for an object with number properties", () => {
    expect(isObj({ foo: 1, bar: 2 })).toBe(true);
  });

  it("should return true for an object with boolean properties", () => {
    expect(isObj({ foo: true, bar: false })).toBe(true);
  });

  it("should return true for an object with mixed properties", () => {
    expect(isObj({ foo: "bar", baz: 1, qux: true })).toBe(true);
  });

  it("should return false for an array", () => {
    expect(isObj([1, 2, 3])).toBe(false);
  });

  it("should return false for a string", () => {
    expect(isObj("foo")).toBe(false);
  });

  it("should return false for a number", () => {
    expect(isObj(42)).toBe(false);
  });

  it("should return false for a boolean", () => {
    expect(isObj(true)).toBe(false);
  });

  it("should return false for null", () => {
    expect(isObj(null)).toBe(false);
  });

  it("should return false for undefined", () => {
    expect(isObj(undefined)).toBe(false);
  });
});
