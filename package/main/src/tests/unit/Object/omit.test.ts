import { omit } from "@/Object/omit";

describe("omit", () => {
  it("should omit specified keys", () => {
    const object = { a: 1, b: 2, c: 3, d: 4 };

    const result = omit(object, "b", "d");

    expect(result).toEqual({ a: 1, c: 3 });
  });

  it("should not modify original object", () => {
    const object = { a: 1, b: 2, c: 3 };

    const result = omit(object, "b");

    expect(object).toEqual({ a: 1, b: 2, c: 3 });
    expect(result).toEqual({ a: 1, c: 3 });
  });

  it("should handle non-existent keys", () => {
    const object = { a: 1, b: 2 };

    const result = omit(object, "c" as keyof typeof object);

    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should handle empty object", () => {
    const object = {};

    const result = omit(object, "a" as keyof typeof object);

    expect(result).toEqual({});
  });

  it("should handle no keys to omit", () => {
    const object = { a: 1, b: 2, c: 3 };

    const result = omit(object);

    expect(result).toEqual({ a: 1, b: 2, c: 3 });
    expect(result).not.toBe(object);
  });

  it("should handle omitting all keys", () => {
    const object = { a: 1, b: 2 };

    const result = omit(object, "a", "b");

    expect(result).toEqual({});
  });

  it("should handle various value types", () => {
    const object = {
      string: "test",
      number: 42,
      boolean: true,
      array: [1, 2, 3],
      object: { nested: true },
      null: null,
      undefined,
    };

    const result = omit(object, "string", "array", "null");

    expect(result).toEqual({
      number: 42,
      boolean: true,
      object: { nested: true },
      undefined,
    });
  });

  it("should preserve type information", () => {
    interface TestObject {
      a: number;
      b: string;
      c: boolean;
    }

    const object: TestObject = { a: 1, b: "test", c: true };

    const result = omit(object, "b");

    expect(result).toEqual({ a: 1, c: true });
    // TypeScript should infer the result type as { a: number; c: boolean; }
  });

  it("should preserve reference equality for retained nested values", () => {
    const nested = { inner: 1 };
    const object = { a: nested, b: 2 };

    const result = omit(object, "b") as unknown as { a: typeof nested };

    expect(result.a).toBe(nested);
  });

  it("should return a new object, not the original", () => {
    const object = { a: 1 };
    const result = omit(object);

    expect(result).not.toBe(object);
  });

  it("should handle symbol keys", () => {
    const symbol = Symbol("k");
    const other = Symbol("other");
    const object = { [symbol]: 1, [other]: 2, a: 3 };

    const result = omit(object, symbol);

    expect(result).toEqual({ [other]: 2, a: 3 });
    expect(Object.getOwnPropertySymbols(result)).toEqual([other]);
  });

  it("should handle duplicate keys passed to omit", () => {
    const object = { a: 1, b: 2, c: 3 };

    const result = omit(object, "b", "b", "b");

    expect(result).toEqual({ a: 1, c: 3 });
  });

  it("should demonstrate prototype-pollution safety by dropping __proto__ when requested", () => {
    // omit uses spread + delete, which is safe even with malicious payloads
    // because delete on __proto__ via bracket notation does not walk the
    // prototype chain.
    const malicious = JSON.parse('{"__proto__":{"polluted":true},"a":1}');

    const result = omit(malicious, "__proto__" as never) as { a?: number };

    // __proto__ own property was removed; Object.prototype is untouched.
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (Object.prototype as any).polluted,
    ).toBeUndefined();
    expect(result.a).toBe(1);
  });

  it("should preserve own __proto__ when it is not explicitly omitted", () => {
    const malicious = JSON.parse('{"__proto__":{"polluted":true},"a":1}');

    const result = omit(malicious, "a") as Record<string, unknown>;

    // Spread preserves the own __proto__ key, since omit only spreads + deletes
    // the explicitly requested keys. This is why callers must still use
    // removePrototype for full sanitization.
    expect(Object.hasOwn(result, "__proto__")).toBe(true);
  });

  it("should not copy inherited enumerable properties from the source", () => {
    const parent = { inherited: 1 };
    const child: { own?: number } = Object.create(parent);
    child.own = 2;

    const result = omit(child, "own");

    expect(Object.hasOwn(result, "own")).toBe(false);
    expect(Object.hasOwn(result, "inherited")).toBe(false);
  });
});
