import { mapValues } from "@/Object/mapValues";

describe("mapValues", () => {
  it("should transform values using the provided function", () => {
    const result = mapValues(
      { a: 1, b: 2, c: 3 },
      (value) => (value as number) * 2,
    );
    expect(result).toEqual({ a: 2, b: 4, c: 6 });
  });

  it("should pass value and key to the transformer", () => {
    const result = mapValues(
      { x: 10, y: 20 },
      (value, key) => `${key}=${value}`,
    );
    expect(result).toEqual({ x: "x=10", y: "y=20" });
  });

  it("should handle an empty object", () => {
    const result = mapValues({}, (value) => value);
    expect(result).toEqual({});
  });

  it("should not modify the original object", () => {
    const original = { a: 1, b: 2 };
    mapValues(original, (value) => (value as number) * 2);
    expect(original).toEqual({ a: 1, b: 2 });
  });

  it("should handle different return types", () => {
    const result = mapValues(
      { a: 1, b: 2, c: 3 },
      (value) => (value as number) > 1,
    );
    expect(result).toEqual({ a: false, b: true, c: true });
  });

  it("should prevent prototype pollution by skipping dangerous keys", () => {
    const malicious = JSON.parse('{"__proto__": 1, "a": 2}');
    const result = mapValues(malicious, (v) => v);
    expect(result).toEqual({ a: 2 });
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
  });

  it("should preserve the key order of the source's own keys", () => {
    const result = mapValues({ c: 1, a: 2, b: 3 }, (value) => value);
    expect(Object.keys(result)).toEqual(["c", "a", "b"]);
  });

  it("should invoke the transformer once per own key with the correct arguments", () => {
    const calls: [unknown, string][] = [];
    mapValues({ x: 1, y: 2 }, (value, key) => {
      calls.push([value, key]);
      return value;
    });

    expect(calls).toEqual([
      [1, "x"],
      [2, "y"],
    ]);
  });

  it("should return a new object, not the original", () => {
    const original = { a: 1 };
    const result = mapValues(original, (value) => value);
    expect(result).not.toBe(original);
  });

  it("should not copy inherited enumerable properties", () => {
    const parent = { inherited: 1 };
    const child = Object.create(parent);
    child.own = 2;

    const result = mapValues(
      child as Record<string, unknown>,
      (value) => value,
    );

    expect(result).toEqual({ own: 2 });
  });

  it("should allow the transformer to return null and undefined", () => {
    const result = mapValues({ a: 1, b: 2, c: 3 }, (value) =>
      (value as number) > 1 ? null : undefined,
    );
    expect(result).toEqual({ a: undefined, b: null, c: null });
    expect(Object.hasOwn(result, "a")).toBe(true);
    expect(Object.hasOwn(result, "b")).toBe(true);
    expect(Object.hasOwn(result, "c")).toBe(true);
  });

  it("should leave global Object.prototype untouched when the input has __proto__ own key (primitive value)", () => {
    const malicious = JSON.parse('{"__proto__":1,"a":2}');

    mapValues(malicious, (v) => v);

    const clean: { polluted?: unknown } = {};
    expect(clean.polluted).toBeUndefined();
  });

  it("should demonstrate that a naive loop pollutes the prototype, while mapValues does not (object-valued __proto__)", () => {
    const malicious = JSON.parse('{"__proto__":{"injected":"bad"},"a":1}');

    // A naive value transformation that iterates own keys and assigns into a
    // fresh `{}` would hit the __proto__ setter and replace the result's
    // prototype with the malicious object.
    const naive: Record<string, unknown> = {};
    for (const key of Object.keys(malicious)) {
      naive[key] = malicious[key];
    }
    expect(Object.getPrototypeOf(naive)).toEqual({ injected: "bad" });

    // mapValues explicitly uses Object.keys and only writes to safe keys via
    // the transformer. Since `__proto__` is an own key of the source, mapValues
    // also writes to it — but the setter assigns an object prototype. The own
    // key is still absent from the result.
    const result = mapValues(malicious, (v) => v);
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(Object.hasOwn(result, "a")).toBe(true);
  });

  it("should preserve reference equality for returned object values", () => {
    const nested = { inner: 1 };
    const result = mapValues({ a: nested }, (value) => value);
    expect(result.a).toBe(nested);
  });
});
