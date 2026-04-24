import { deepClone } from "@/Object/deepClone";
import { removePrototype } from "@/Object/removePrototype";

describe("deepClone", () => {
  it("should deep clone a simple object", () => {
    const original = { a: 1, b: "hello" };
    const cloned = deepClone(original);

    expect(cloned).toEqual(original);
    expect(cloned).not.toBe(original);
  });

  it("should deep clone nested objects", () => {
    const original = { a: { b: { c: 3 } } };
    const cloned = deepClone(original);

    expect(cloned).toEqual(original);
    expect(cloned.a).not.toBe(original.a);
    expect(cloned.a.b).not.toBe(original.a.b);
  });

  it("should deep clone arrays", () => {
    const original = [1, [2, [3]]];
    const cloned = deepClone(original);

    expect(cloned).toEqual(original);
    expect(cloned).not.toBe(original);
    expect(cloned[1]).not.toBe(original[1]);
  });

  it("should deep clone Date objects", () => {
    const original = { date: new Date("2024-01-01") };
    const cloned = deepClone(original);

    expect(cloned.date.getTime()).toBe(original.date.getTime());
    expect(cloned.date).not.toBe(original.date);
  });

  it("should deep clone Map and Set", () => {
    const original = {
      map: new Map([["key", "value"]]),
      set: new Set([1, 2, 3]),
    };
    const cloned = deepClone(original);

    expect(cloned.map.get("key")).toBe("value");
    expect(cloned.set.has(2)).toBe(true);
    expect(cloned.map).not.toBe(original.map);
    expect(cloned.set).not.toBe(original.set);
  });

  it("should clone primitive values", () => {
    expect(deepClone(42)).toBe(42);
    expect(deepClone("hello")).toBe("hello");
    expect(deepClone(true)).toBe(true);
    expect(deepClone(null)).toBe(null);
  });

  it("should not affect the original when modifying the clone", () => {
    const original = { a: 1, b: { c: 2 } };
    const cloned = deepClone(original);

    cloned.b.c = 99;
    expect(original.b.c).toBe(2);
  });

  it("should deep clone RegExp objects", () => {
    const original = { pattern: /test/gi };
    const cloned = deepClone(original);

    expect(cloned.pattern.source).toBe("test");
    expect(cloned.pattern.flags).toBe("gi");
    expect(cloned.pattern).not.toBe(original.pattern);
  });

  it("should prevent prototype pollution via __proto__", () => {
    const payload = JSON.parse('{"__proto__": {"polluted": true}}');
    const cloned = deepClone(removePrototype(payload));

    // Should not have __proto__ property set directly
    expect(Object.hasOwn(cloned, "__proto__")).toBe(false);
  });

  it("should prevent prototype pollution via constructor", () => {
    const payload = JSON.parse(
      '{"constructor": {"prototype": {"polluted": true}}}',
    );
    const cloned = deepClone(removePrototype(payload));

    // Should not overwrite constructor
    expect(cloned.constructor).toBe(Object);
  });

  it("should demonstrate that cloning an unsanitized payload locally pollutes the clone", () => {
    const payload = JSON.parse('{"__proto__":{"injected":"local"},"safe":1}');

    const polluted = deepClone(payload) as {
      injected?: unknown;
      safe?: unknown;
    };

    // JSON.parse keeps __proto__ as an own property; deepClone uses Object.keys
    // and assigns via bracket notation, which triggers the __proto__ setter on
    // the freshly-created result, replacing its prototype locally.
    expect(Object.getPrototypeOf(polluted)).toEqual({ injected: "local" });
    expect(polluted.injected).toBe("local");
    expect(polluted.safe).toBe(1);

    // Global Object.prototype remains clean.
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (Object.prototype as any).injected,
    ).toBeUndefined();

    // After sanitization the clone is clean and has no injected prototype.
    const safe = deepClone(removePrototype(payload)) as { injected?: unknown };
    expect(Object.getPrototypeOf(safe)).toBe(Object.prototype);
    expect(safe.injected).toBeUndefined();
  });

  it("should deep clone deeply nested arrays and objects together", () => {
    const original = {
      list: [{ tags: ["a", "b"] }, { tags: ["c", "d"] }],
    };

    const cloned = deepClone(original);

    expect(cloned).toEqual(original);
    expect(cloned.list).not.toBe(original.list);
    expect(cloned.list[0]).not.toBe(original.list[0]);
    expect(cloned.list[0].tags).not.toBe(original.list[0].tags);
    cloned.list[0].tags.push("z");
    expect(original.list[0].tags).toEqual(["a", "b"]);
  });

  it("should clone Map entries recursively", () => {
    const nested = { inner: 1 };
    const original = new Map<string, unknown>([["a", nested]]);

    const cloned = deepClone(original);

    const clonedNested = cloned.get("a") as { inner: number };
    expect(clonedNested).toEqual(nested);
    expect(clonedNested).not.toBe(nested);
  });

  it("should clone Set members recursively", () => {
    const nested = { inner: 1 };
    const original = new Set<unknown>([nested]);

    const cloned = deepClone(original);

    const [clonedNested] = [...cloned] as { inner: number }[];
    expect(clonedNested).toEqual(nested);
    expect(clonedNested).not.toBe(nested);
  });

  it("should preserve the RegExp lastIndex-independent state (new instance)", () => {
    const pattern = /x/g;
    pattern.lastIndex = 5;
    const cloned = deepClone({ pattern });

    expect(cloned.pattern.source).toBe("x");
    expect(cloned.pattern.flags).toBe("g");
    expect(cloned.pattern.lastIndex).toBe(0);
  });

  it("should produce independent clones for multiple references to the same nested object", () => {
    const shared = { x: 1 };
    const original = { a: shared, b: shared };

    const cloned = deepClone(original);

    expect(cloned.a).toEqual({ x: 1 });
    expect(cloned.b).toEqual({ x: 1 });
    // deepClone does not deduplicate references — each occurrence becomes a
    // distinct copy. Callers should be aware of this trade-off.
    expect(cloned.a).not.toBe(cloned.b);
    cloned.a.x = 99;
    expect(cloned.b.x).toBe(1);
  });

  it("should not copy symbol-keyed properties", () => {
    const symbol = Symbol("k");
    const original = { a: 1, [symbol]: "value" } as Record<string, unknown>;

    const cloned = deepClone(original);

    expect(Object.getOwnPropertySymbols(cloned)).toHaveLength(0);
    expect((cloned as Record<symbol, unknown>)[symbol]).toBeUndefined();
  });

  it("should not copy inherited enumerable properties", () => {
    const parent = { inherited: "value" };
    const child = Object.create(parent);
    child.own = 1;

    const cloned = deepClone(child) as Record<string, unknown>;

    expect(Object.hasOwn(cloned, "own")).toBe(true);
    expect(Object.hasOwn(cloned, "inherited")).toBe(false);
  });

  it("should handle undefined input", () => {
    expect(deepClone(undefined)).toBeUndefined();
  });

  it("should clone an empty object and empty array", () => {
    expect(deepClone({})).toEqual({});
    expect(deepClone([])).toEqual([]);
  });

  it("should clone Date values that hold sub-properties by producing a Date (sub-props not copied)", () => {
    const date = new Date("2024-01-01") as Date & { tag?: string };
    date.tag = "extra";

    const cloned = deepClone({ date }).date as Date & { tag?: string };

    expect(cloned).toBeInstanceOf(Date);
    expect(cloned.getTime()).toBe(date.getTime());
    expect(cloned).not.toBe(date);
    // Ad-hoc properties on Date are not preserved — documents the behavior.
    expect(cloned.tag).toBeUndefined();
  });
});
