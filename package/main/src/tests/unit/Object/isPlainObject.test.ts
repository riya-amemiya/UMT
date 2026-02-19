import { isPlainObject } from "@/Object/isPlainObject";

describe("isPlainObject", () => {
  it("should return true for object literals", () => {
    expect(isPlainObject({})).toBe(true);
    expect(isPlainObject({ a: 1 })).toBe(true);
  });

  it("should return true for Object.create(null)", () => {
    expect(isPlainObject(Object.create(null))).toBe(true);
  });

  it("should return true for objects with constructor property", () => {
    expect(isPlainObject({ constructor: 1 })).toBe(true);
    // @ts-expect-error
    expect(
      isPlainObject({
        constructor() {
          return 1;
        },
      }),
    ).toBe(true);
  });

  it("should return true for objects inheriting from plain objects", () => {
    const parent = { a: 1 };
    const child = Object.create(parent);
    expect(isPlainObject(child)).toBe(true);
  });

  it("should return false for non-objects", () => {
    expect(isPlainObject(null)).toBe(false);
    expect(isPlainObject(undefined)).toBe(false);
    expect(isPlainObject(1)).toBe(false);
    expect(isPlainObject("string")).toBe(false);
    expect(isPlainObject(true)).toBe(false);
  });

  it("should return false for arrays", () => {
    expect(isPlainObject([])).toBe(false);
    expect(isPlainObject([])).toBe(false);
  });

  it("should return false for Date objects", () => {
    expect(isPlainObject(new Date())).toBe(false);
  });

  it("should return false for Map/Set", () => {
    expect(isPlainObject(new Map())).toBe(false);
    expect(isPlainObject(new Set())).toBe(false);
  });

  it("should return false for custom classes", () => {
    class MyClass {}
    expect(isPlainObject(new MyClass())).toBe(false);
  });

  it("should return false for functions", () => {
    expect(
      isPlainObject(() => {
        return 1;
      }),
    ).toBe(false);
    expect(isPlainObject(() => 1)).toBe(false);
  });
});
