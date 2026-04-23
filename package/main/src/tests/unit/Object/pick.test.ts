import { pick } from "@/Object/pick";
import { removePrototype } from "@/Object/removePrototype";

describe("pick function", () => {
  test("should select a single key", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: 1 });
  });

  test("should select multiple keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("should handle non-existent keys", () => {
    const obj = { a: 1, b: 2 };
    // @ts-expect-error
    const result = pick(obj, "c");
    expect(result).toEqual({});
  });

  test("should handle empty objects", () => {
    const obj = {};
    // @ts-expect-error
    const result = pick(obj, "a");
    expect(result).toEqual({});
  });

  test("should handle no keys specified", () => {
    const obj = { a: 1, b: 2 };
    const result = pick(obj);
    expect(result).toEqual({});
  });

  test("should select all keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "b", "c");
    expect(result).toEqual(obj);
  });

  test("should handle duplicate keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("should handle nested objects", () => {
    const obj = { a: { b: 1 }, c: 2 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: { b: 1 } });
  });

  test("should handle objects with null or undefined properties", () => {
    const obj = { a: null, b: undefined, c: 3 };
    const result = pick(obj, "a", "b", "c");
    expect(result).toEqual({ a: null, b: undefined, c: 3 });
  });

  test("should handle objects containing arrays", () => {
    const obj = { a: [1, 2, 3], b: 4 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: [1, 2, 3] });
  });

  it("should prevent prototype pollution via __proto__ when sanitized", () => {
    const malicious = JSON.parse(
      '{"__proto__": {"polluted": true}, "safe": 42}',
    );
    const result = pick(removePrototype(malicious), "safe");

    expect(result).toEqual({ safe: 42 });
    const clean = {} as Record<string, unknown>;
    expect("polluted" in clean).toBe(false);
  });

  it("should not pollute global Object.prototype when called with __proto__ key", () => {
    const malicious = JSON.parse('{"__proto__": {"polluted": true}}') as Record<
      string,
      unknown
    >;
    pick(malicious, "__proto__" as never);

    const clean = {} as Record<string, unknown>;
    expect("polluted" in clean).toBe(false);
  });

  it("should not pollute global Object.prototype when called with constructor and prototype keys", () => {
    const malicious = JSON.parse(
      '{"constructor": {"polluted": true}, "prototype": {"injected": true}}',
    ) as Record<string, unknown>;
    pick(malicious, "constructor" as never, "prototype" as never);

    const clean = {} as Record<string, unknown>;
    expect("polluted" in clean).toBe(false);
    expect("injected" in clean).toBe(false);
  });

  it("should preserve reference equality for selected nested values", () => {
    const nested = { inner: 1 };
    const object = { a: nested, b: 2 };

    const result = pick(object, "a");

    expect(result.a).toBe(nested);
  });

  it("should return a new object, not the original", () => {
    const object = { a: 1, b: 2 };
    const result = pick(object, "a", "b");

    expect(result).toEqual(object);
    expect(result).not.toBe(object);
  });

  it("should handle symbol keys", () => {
    const symbol = Symbol("k");
    const object = { [symbol]: 1, a: 2 };

    const result = pick(object, symbol);

    expect(Object.getOwnPropertySymbols(result)).toEqual([symbol]);
    expect((result as Record<symbol, unknown>)[symbol]).toBe(1);
  });

  it("should assign undefined for requested keys that are not own properties (bracket-access semantics)", () => {
    const object = { a: 1 } as Record<string, unknown>;
    const result = pick(object, "b" as keyof typeof object);

    // pick reads via bracket access, so non-existent keys yield undefined.
    // The resulting object still has the requested key as an own property.
    expect(Object.hasOwn(result, "b")).toBe(true);
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).b).toBeUndefined();
  });

  it("should demonstrate that pick(malicious, '__proto__') produces an unsafe own key without sanitization", () => {
    const malicious = JSON.parse('{"__proto__":{"polluted":true},"a":1}');

    const result = pick(malicious, "__proto__" as never) as Record<
      string,
      unknown
    >;

    // pick's naive bracket-assignment triggers the __proto__ setter on the
    // fresh result, so the result's own prototype is replaced.
    expect(Object.getPrototypeOf(result)).toEqual({ polluted: true });
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).polluted).toBe(true);

    // After sanitization, the dangerous key is simply absent.
    const sanitized = pick(removePrototype(malicious), "a");
    expect(sanitized).toEqual({ a: 1 });
    expect(Object.getPrototypeOf(sanitized)).toBe(Object.prototype);
  });
});
