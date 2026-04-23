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

  it("should return true for JSON.parse output (which has an own __proto__ key)", () => {
    // JSON.parse produces plain objects with Object.prototype chain.
    // Malicious payloads rely on this being recognized as a plain object.
    expect(isPlainObject(JSON.parse("{}"))).toBe(true);
    expect(isPlainObject(JSON.parse('{"a":1}'))).toBe(true);
    expect(isPlainObject(JSON.parse('{"__proto__":{"polluted":true}}'))).toBe(
      true,
    );
  });

  it("should return true after Object.setPrototypeOf(obj, null)", () => {
    const object = { a: 1 };
    Object.setPrototypeOf(object, null);
    expect(isPlainObject(object)).toBe(true);
  });

  it("should return true for deep plain-object chains", () => {
    const grandparent = { level: 1 };
    const parent = Object.create(grandparent);
    const child = Object.create(parent);

    expect(isPlainObject(grandparent)).toBe(true);
    expect(isPlainObject(parent)).toBe(true);
    expect(isPlainObject(child)).toBe(true);
  });

  it("should return false for Error instances", () => {
    expect(isPlainObject(new Error("boom"))).toBe(false);
    expect(isPlainObject(new TypeError("boom"))).toBe(false);
  });

  it("should return false for Promises", () => {
    const promise = Promise.resolve(1);
    expect(isPlainObject(promise)).toBe(false);
  });

  it("should return false for typed arrays and ArrayBuffer", () => {
    expect(isPlainObject(new Uint8Array())).toBe(false);
    expect(isPlainObject(new Int32Array())).toBe(false);
    expect(isPlainObject(new ArrayBuffer(0))).toBe(false);
  });

  it("should return false for boxed primitive wrappers", () => {
    // biome-ignore lint/style/useConsistentBuiltinInstantiation: testing boxed wrappers
    expect(isPlainObject(new Number(1))).toBe(false);
    // biome-ignore lint/style/useConsistentBuiltinInstantiation: testing boxed wrappers
    expect(isPlainObject(new String("x"))).toBe(false);
    // biome-ignore lint/style/useConsistentBuiltinInstantiation: testing boxed wrappers
    expect(isPlainObject(new Boolean(true))).toBe(false);
  });

  it("should return false for WeakMap and WeakSet", () => {
    expect(isPlainObject(new WeakMap())).toBe(false);
    expect(isPlainObject(new WeakSet())).toBe(false);
  });

  it("should return true for an object whose constructor property shadows Object", () => {
    // Own constructor property should NOT confuse isPlainObject — the check
    // looks at the prototype's constructor, not the own key.
    const object = { constructor: "not a function" };
    expect(isPlainObject(object)).toBe(true);
  });

  it("should return true for Object.create(Object.prototype)", () => {
    expect(isPlainObject(Object.create(Object.prototype))).toBe(true);
  });

  it("should return false for generator functions and their instances", () => {
    function* gen() {
      yield 1;
    }
    expect(isPlainObject(gen)).toBe(false);
    expect(isPlainObject(gen())).toBe(false);
  });

  it("should return false for objects created via class inheriting from Object", () => {
    class Extended extends Object {
      x = 1;
    }
    expect(isPlainObject(new Extended())).toBe(false);
  });
});
