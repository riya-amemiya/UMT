import { merge } from "@/Object/merge";
import { removePrototype } from "@/Object/removePrototype";

describe("merge", () => {
  it("should merge multiple objects", () => {
    const target = { a: 1, b: 2 };
    const source1 = { b: 3, c: 4 };
    const source2 = { c: 5, d: 6 };

    const result = merge(target, source1, source2);
    expect(result).toEqual({ a: 1, b: 3, c: 5, d: 6 });
  });

  it("should not modify original objects", () => {
    const target = { a: 1, b: 2 };
    const source = { b: 3, c: 4 };

    const result = merge(target, source);

    expect(target).toEqual({ a: 1, b: 2 });
    expect(source).toEqual({ b: 3, c: 4 });
    expect(result).toEqual({ a: 1, b: 3, c: 4 });
  });

  it("should handle empty target", () => {
    const target = {};
    const source = { a: 1, b: 2 };

    const result = merge(target, source);

    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should handle no sources", () => {
    const target = { a: 1, b: 2 };

    const result = merge(target);

    expect(result).toEqual({ a: 1, b: 2 });
    expect(result).not.toBe(target);
  });

  it("should handle empty sources", () => {
    const target = { a: 1, b: 2 };
    const source = {};

    const result = merge(target, source);

    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should override properties with later sources", () => {
    const target = { a: 1 };
    const source1 = { a: 2 };
    const source2 = { a: 3 };

    const result = merge(target, source1, source2);

    expect(result).toEqual({ a: 3 });
  });

  it("should handle null and undefined values", () => {
    const target = { a: 1, b: null };
    const source = { b: undefined, c: null };

    const result = merge(target, source);

    expect(result).toEqual({ a: 1, b: undefined, c: null });
  });

  it("should prevent prototype pollution via __proto__", () => {
    const malicious = JSON.parse('{"__proto__": {"polluted": true}}');
    merge({}, malicious);

    // Verify the global Object prototype was not polluted
    const clean = {};
    expect("polluted" in clean).toBe(false);
  });

  it("should prevent prototype pollution via constructor and prototype", () => {
    const malicious = JSON.parse(
      '{"constructor": {"polluted": true}, "prototype": {"injected": true}}',
    );
    const result = merge({}, removePrototype(malicious));

    expect(Object.keys(result)).not.toContain("constructor");
    expect(Object.keys(result)).not.toContain("prototype");
  });

  it("should demonstrate local prototype pollution when __proto__ payload is merged without sanitization", () => {
    const malicious = JSON.parse('{"__proto__":{"injected":"local"}}');

    const polluted = merge({}, malicious) as Record<string, unknown>;

    // The result's own prototype was replaced by the injected object.
    expect(Object.getPrototypeOf(polluted)).toEqual({ injected: "local" });
    expect((polluted as { injected?: unknown }).injected).toBe("local");

    // Global Object.prototype must remain untouched.
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (Object.prototype as any).injected,
    ).toBeUndefined();

    // After sanitization, the result is clean.
    const safe = merge({}, removePrototype(malicious)) as {
      injected?: unknown;
    };
    expect(Object.getPrototypeOf(safe)).toBe(Object.prototype);
    expect(safe.injected).toBeUndefined();
  });

  it("should preserve own constructor key on the result when not sanitized", () => {
    const malicious = JSON.parse('{"constructor":{"polluted":true},"a":1}');

    const result = merge({}, malicious) as {
      constructor: { polluted?: unknown };
    };

    // Without sanitization the key flows through — demonstrating the need for removePrototype.
    expect(Object.hasOwn(result, "constructor")).toBe(true);
    expect(result.constructor.polluted).toBe(true);

    const safe = merge({}, removePrototype(malicious));
    expect(Object.hasOwn(safe, "constructor")).toBe(false);
  });

  it("should merge many sources in left-to-right precedence order", () => {
    const result = merge({ a: 1 }, { a: 2 }, { a: 3 }, { a: 4 }, { a: 5 });

    expect(result.a).toBe(5);
  });

  it("should only copy own enumerable string keys of each source", () => {
    const parent = { inherited: "parent" };
    const child = Object.create(parent);
    child.own = "child";

    const result = merge({ a: 1 }, child as Record<string, unknown>);

    expect(result).toEqual({ a: 1, own: "child" });
    expect(Object.hasOwn(result, "inherited")).toBe(false);
  });

  it("should not copy symbol keys from sources", () => {
    const symbol = Symbol("key");
    const source = { a: 1, [symbol]: "value" } as Record<string, unknown>;

    const result = merge({}, source) as Record<string | symbol, unknown>;

    expect(Object.getOwnPropertySymbols(result)).toHaveLength(0);
    expect(result[symbol]).toBeUndefined();
  });

  it("should return a new object even when there are no sources", () => {
    const target = { a: 1 };

    const result = merge(target);

    expect(result).toEqual(target);
    expect(result).not.toBe(target);
  });

  it("should preserve reference equality for nested values (shallow merge)", () => {
    const nested = { x: 1 };
    const source = { nested };

    const result = merge({}, source);

    expect(result.nested).toBe(nested);
  });

  it("should overwrite nested objects instead of merging them (shallow)", () => {
    const target = { a: { b: 1, c: 2 } };
    const source = { a: { d: 3 } };

    const result = merge(target, source);

    expect(result.a).toEqual({ d: 3 });
    expect(result.a).not.toHaveProperty("b");
  });
});
