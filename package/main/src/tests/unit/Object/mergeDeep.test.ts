import { mergeDeep } from "@/Object/mergeDeep";

describe("mergeDeep", () => {
  it("should deeply merge nested objects", () => {
    const target = {
      a: 1,
      b: {
        c: 2,
        d: 3,
      },
    };
    const source = {
      b: {
        d: 4,
        e: 5,
      },
      f: 6,
    };

    const result = mergeDeep(target, source);

    expect(result).toEqual({
      a: 1,
      b: {
        c: 2,
        d: 4,
        e: 5,
      },
      f: 6,
    });
  });

  it("should not modify original objects", () => {
    const target = { a: { b: 1 } };
    const source = { a: { c: 2 } };

    const result = mergeDeep(target, source);

    expect(target).toEqual({ a: { b: 1 } });
    expect(source).toEqual({ a: { c: 2 } });
    expect(result).toEqual({ a: { b: 1, c: 2 } });
  });

  it("should handle multiple levels of nesting", () => {
    const target = {
      level1: {
        level2: {
          level3: {
            a: 1,
          },
        },
      },
    };
    const source = {
      level1: {
        level2: {
          level3: {
            b: 2,
          },
          c: 3,
        },
      },
    };

    const result = mergeDeep(target, source);

    expect(result).toEqual({
      level1: {
        level2: {
          level3: {
            a: 1,
            b: 2,
          },
          c: 3,
        },
      },
    });
  });

  it("should handle non-object values", () => {
    const target = { a: { b: 1 } };
    const source = { a: "string" };

    const result = mergeDeep(target, source);

    expect(result).toEqual({ a: "string" });
  });

  it("should handle arrays as values", () => {
    const target = { a: [1, 2], b: { c: [3] } };
    const source = { a: [4, 5], b: { c: [6] } };

    const result = mergeDeep(target, source);

    expect(result).toEqual({ a: [4, 5], b: { c: [6] } });
  });

  it("should handle empty objects", () => {
    const target = {};
    const source = { a: { b: 1 } };

    const result = mergeDeep(target, source);

    expect(result).toEqual({ a: { b: 1 } });
  });

  it("should handle multiple sources", () => {
    const target = { a: { b: 1 } };
    const source1 = { a: { c: 2 } };
    const source2 = { a: { d: 3 } };

    const result = mergeDeep(target, source1, source2);
    expect(result).toEqual({ a: { b: 1, c: 2, d: 3 } });
  });

  it("should handle null and undefined", () => {
    const target = { a: { b: 1 } };
    const source = { a: null };

    const result = mergeDeep(target, source);

    expect(result).toEqual({ a: null });
  });

  it("should handle no sources provided", () => {
    const target = { a: 1, b: { c: 2 } };

    const result = mergeDeep(target);

    expect(result).toEqual({ a: 1, b: { c: 2 } });
  });

  it("should handle when target is not a plain object", () => {
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    const target = null as any;
    const source = { a: 1 };

    const result = mergeDeep(target, source);

    expect(result).toEqual({ a: 1 });
  });

  it("should exclude inherited properties from source objects", () => {
    const parent = { inherited: "value" };
    const child = Object.create(parent);
    child.own = "child";

    const target = { a: 1 };

    const result = mergeDeep(target, child);

    expect(result).toEqual({ a: 1, own: "child" });
    expect(result).not.toHaveProperty("inherited");
  });

  it("should prevent prototype pollution via __proto__", () => {
    const payload = JSON.parse('{"__proto__": {"polluted": true}}');
    const target = {};
    const result = mergeDeep(target, payload);

    // Should not have polluted property on the result (local pollution)
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).polluted).toBeUndefined();
    // Should not have __proto__ property set directly
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
  });

  it("should prevent prototype pollution via constructor", () => {
    const payload = JSON.parse(
      '{"constructor": {"prototype": {"polluted": true}}}',
    );
    const target = {};
    const result = mergeDeep(target, payload);

    // Should not have polluted property on Object.prototype (global pollution)
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((Object.prototype as any).polluted).toBeUndefined();
    // Should not overwrite constructor
    expect(result.constructor).toBe(Object);
  });

  it("should prevent prototype pollution via prototype", () => {
    const payload = JSON.parse('{"prototype": {"polluted": true}}');
    const target = {};
    const result = mergeDeep(target, payload);

    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).prototype).toBeUndefined();
  });
});
