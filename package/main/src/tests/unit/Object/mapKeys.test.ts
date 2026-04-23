import { mapKeys } from "@/Object/mapKeys";

describe("mapKeys", () => {
  it("should transform keys using the provided function", () => {
    const result = mapKeys({ a: 1, b: 2, c: 3 }, (_value, key) =>
      key.toUpperCase(),
    );
    expect(result).toEqual({ A: 1, B: 2, C: 3 });
  });

  it("should pass value and key to the transformer", () => {
    const result = mapKeys({ x: 10, y: 20 }, (value, key) => `${key}_${value}`);
    expect(result).toEqual({ x_10: 10, y_20: 20 });
  });

  it("should handle an empty object", () => {
    const result = mapKeys({}, (_value, key) => key);
    expect(result).toEqual({});
  });

  it("should handle keys that map to the same value", () => {
    const result = mapKeys({ a: 1, b: 2 }, () => "same");
    expect(result).toEqual({ same: 2 });
  });

  it("should not modify the original object", () => {
    const original = { a: 1, b: 2 };
    mapKeys(original, (_value, key) => key.toUpperCase());
    expect(original).toEqual({ a: 1, b: 2 });
  });

  it("should prevent prototype pollution from source keys", () => {
    const malicious = JSON.parse('{"__proto__": 1, "a": 2}');
    const result = mapKeys(malicious, (_value, key) => key);
    expect(result).toEqual({ a: 2 });
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
  });

  it("should prevent prototype pollution from transformed keys", () => {
    const result = mapKeys({ a: 1 }, () => "__proto__");
    expect(result).toEqual({});
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
  });

  it("should preserve the order of the source's own keys", () => {
    const result = mapKeys({ c: 1, a: 2, b: 3 }, (_value, key) =>
      key.toUpperCase(),
    );
    expect(Object.keys(result)).toEqual(["C", "A", "B"]);
  });

  it("should not touch global Object.prototype when the payload contains __proto__", () => {
    const malicious = JSON.parse('{"__proto__":{"injected":"x"},"a":1}');
    mapKeys(malicious, (_value, key) => key);

    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (Object.prototype as any).injected,
    ).toBeUndefined();
    const innocent: { injected?: unknown } = {};
    expect(innocent.injected).toBeUndefined();
  });

  it("should demonstrate __proto__-primitive payload is harmlessly swallowed by the setter", () => {
    // When the value of the own `__proto__` key is a primitive, the __proto__
    // setter silently rejects it, so mapKeys produces a result without the
    // dangerous key and with the default Object.prototype.
    const malicious = JSON.parse('{"__proto__":1,"a":2}');

    const result = mapKeys(malicious, (_value, key) => key);

    expect(result).toEqual({ a: 2 });
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(Object.getPrototypeOf(result)).toBe(Object.prototype);
  });

  it("should locally replace the result's prototype when __proto__ value is an object (known limitation)", () => {
    // Documents the library's current behavior: mapKeys does not filter keys.
    // An own __proto__ key carrying an object value triggers the setter on
    // the freshly-allocated result, replacing its prototype locally. The
    // result's own keys remain safe; callers must still use removePrototype*
    // if the prototype chain must also be sanitized.
    const malicious = JSON.parse('{"__proto__":{"injected":"bad"},"a":1}');

    const result = mapKeys(malicious, (_value, key) => key);

    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(Object.hasOwn(result, "a")).toBe(true);
    expect(Object.getPrototypeOf(result)).toEqual({ injected: "bad" });
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).injected).toBe("bad");

    // Global Object.prototype is untouched.
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (Object.prototype as any).injected,
    ).toBeUndefined();
  });

  it("should not copy inherited enumerable properties from the source", () => {
    const parent = { inherited: 1 };
    const child = Object.create(parent);
    child.own = 2;

    const result = mapKeys(child as Record<string, unknown>, (_v, key) => key);

    expect(result).toEqual({ own: 2 });
  });

  it("should pass `constructor` and `prototype` through as regular own keys (only __proto__ primitives are dropped)", () => {
    // mapKeys only relies on JavaScript's __proto__ setter semantics for
    // protection. Own keys named `constructor` or `prototype` are retained
    // verbatim as regular own keys.
    const malicious = JSON.parse(
      '{"a":1,"constructor":2,"b":3,"prototype":4,"c":5,"__proto__":6}',
    );

    const result = mapKeys(malicious, (_value, key) => key);

    expect(result).toEqual({
      a: 1,
      b: 3,
      c: 5,
      constructor: 2,
      prototype: 4,
    });
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
  });

  it("should accept a transformer returning a numeric-looking key", () => {
    const result = mapKeys({ a: 1 }, () => "0");
    expect(result).toEqual({ "0": 1 });
  });
});
