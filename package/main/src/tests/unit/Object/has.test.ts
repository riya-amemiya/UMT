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

  it("should block __proto__ key to prevent prototype pollution", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, "__proto__")).toBe(false);
    expect(has(obj, "__proto__.polluted")).toBe(false);
    expect(has(obj, "a.__proto__")).toBe(false);
  });

  it("should block constructor key to prevent prototype pollution", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, "constructor")).toBe(false);
    expect(has(obj, "constructor.prototype")).toBe(false);
  });

  it("should block prototype key to prevent prototype pollution", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, "prototype")).toBe(false);
  });

  it("should block dangerous keys passed as array path", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, ["__proto__"])).toBe(false);
    expect(has(obj, ["constructor"])).toBe(false);
    expect(has(obj, ["prototype"])).toBe(false);
    expect(has(obj, ["a", "__proto__"])).toBe(false);
  });

  it("should return true when the value at the path is falsy but the key exists", () => {
    expect(has({ a: null }, "a")).toBe(true);
    expect(has({ a: undefined }, "a")).toBe(true);
    expect(has({ a: 0 }, "a")).toBe(true);
    expect(has({ a: false }, "a")).toBe(true);
    expect(has({ a: "" }, "a")).toBe(true);
  });

  it("should return false when traversing through null or undefined", () => {
    expect(has({ a: null }, "a.b")).toBe(false);
    expect(has({ a: undefined }, "a.b")).toBe(false);
  });

  it("should distinguish own properties from inherited ones", () => {
    const parent = { inherited: 1 };
    const child = Object.create(parent);
    child.own = 2;

    expect(has(child, "own")).toBe(true);
    // Object.hasOwn — inherited keys must NOT be reported.
    expect(has(child, "inherited")).toBe(false);
  });

  it("should handle deeply nested paths", () => {
    const object = { a: { b: { c: { d: { e: 1 } } } } };
    expect(has(object, "a.b.c.d.e")).toBe(true);
    expect(has(object, "a.b.c.d.f")).toBe(false);
    expect(has(object, "a.b.c.d.e.f")).toBe(false);
  });

  it("should demonstrate that plain `in` operator would incorrectly report inherited keys", () => {
    const object = { a: 1 } as Record<string, unknown>;

    // Using `in` walks the prototype chain — hasOwn-based `has` must not.
    expect("toString" in object).toBe(true);
    expect(has(object, "toString")).toBe(false);
    expect(has(object, ["toString"])).toBe(false);
  });

  it("should report __proto__ as present only when it is an own property (JSON.parse)", () => {
    // Documents the actual semantic: `has` uses Object.hasOwn, so a
    // JSON.parse-created __proto__ own key is visible. Callers sanitizing
    // untrusted input should use removePrototype* before calling has.
    const malicious = JSON.parse('{"__proto__":{"polluted":true}}');
    expect(has(malicious, "__proto__")).toBe(true);

    // A plain object has __proto__ only via the prototype chain, not as own.
    expect(has({ a: 1 }, "__proto__")).toBe(false);
  });

  it("should handle a path that steps off a primitive", () => {
    const object = { a: 1 } as Record<string, unknown>;
    expect(has(object, "a.b")).toBe(false);
    expect(has(object, ["a", "b"])).toBe(false);
  });

  it("should accept an object as the last segment", () => {
    const object = { a: { b: { c: 1 } } };
    expect(has(object, "a.b")).toBe(true);
    expect(has(object, ["a", "b"])).toBe(true);
  });

  it("should not match numeric indices as property names unless they are own keys", () => {
    const array = [10, 20, 30];
    const wrapper = { list: array } as Record<string, unknown>;
    expect(has(wrapper, "list.0")).toBe(true);
    expect(has(wrapper, "list.3")).toBe(false);
    // Inherited Array prototype methods must not be reported.
    expect(has(wrapper, "list.push")).toBe(false);
  });
});
