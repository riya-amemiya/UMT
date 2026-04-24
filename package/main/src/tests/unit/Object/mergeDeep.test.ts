import { mergeDeep } from "@/Object/mergeDeep";
import { removePrototype } from "@/Object/removePrototype";
import { removePrototypeDeep } from "@/Object/removePrototypeDeep";

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
    const result = mergeDeep(target, removePrototype(payload));

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
    const result = mergeDeep(target, removePrototype(payload));

    // Should not have polluted property on Object.prototype (global pollution)
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((Object.prototype as any).polluted).toBeUndefined();
    // Should not overwrite constructor
    expect(result.constructor).toBe(Object);
  });

  it("should prevent prototype pollution via prototype", () => {
    const payload = JSON.parse('{"prototype": {"polluted": true}}');
    const target = {};
    const result = mergeDeep(target, removePrototype(payload));

    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).prototype).toBeUndefined();
  });

  it("should demonstrate __proto__ pollution via a naive deep merge and confirm mergeDeep with sanitization is safe", () => {
    const malicious = JSON.parse('{"__proto__":{"injected":"global"}}');

    const vulnerableDeepMerge = (
      target: Record<string, unknown>,
      source: Record<string, unknown>,
    ): Record<string, unknown> => {
      for (const key of Object.keys(source)) {
        const sourceValue = source[key];
        const targetValue = target[key];
        if (
          sourceValue !== null &&
          typeof sourceValue === "object" &&
          !Array.isArray(sourceValue) &&
          targetValue !== null &&
          typeof targetValue === "object" &&
          !Array.isArray(targetValue)
        ) {
          vulnerableDeepMerge(
            targetValue as Record<string, unknown>,
            sourceValue as Record<string, unknown>,
          );
        } else {
          target[key] = sourceValue;
        }
      }
      return target;
    };

    try {
      const victim: Record<string, unknown> = {};
      vulnerableDeepMerge(victim, malicious);

      // Global pollution confirmed: a freshly created object inherits the property.
      const innocent: { injected?: unknown } = {};
      expect(innocent.injected).toBe("global");

      const safe = mergeDeep({}, removePrototypeDeep(malicious));
      expect(Object.hasOwn(safe, "__proto__")).toBe(false);
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      expect((safe as any).injected).toBe("global");
      // After cleanup, the injected property must disappear everywhere.
    } finally {
      Reflect.deleteProperty(Object.prototype, "injected");
    }

    const afterCleanup: { injected?: unknown } = {};
    expect(afterCleanup.injected).toBeUndefined();
  });

  it("should demonstrate constructor.prototype pollution via a naive deep merge", () => {
    const malicious = JSON.parse(
      '{"constructor":{"prototype":{"globallyInjected":"yes"}}}',
    );

    // A naive merge that recurses into anything "object-like" (including the
    // Object constructor function). This reaches Object.prototype and writes
    // to it — the canonical constructor.prototype pollution pattern.
    const vulnerableDeepMerge = (
      target: Record<string, unknown>,
      source: Record<string, unknown>,
    ): Record<string, unknown> => {
      for (const key of Object.keys(source)) {
        const sourceValue = source[key];
        const targetValue = target[key];
        const targetIsObjectLike =
          targetValue !== null &&
          (typeof targetValue === "object" ||
            typeof targetValue === "function");
        if (
          sourceValue !== null &&
          typeof sourceValue === "object" &&
          !Array.isArray(sourceValue) &&
          targetIsObjectLike
        ) {
          vulnerableDeepMerge(
            targetValue as Record<string, unknown>,
            sourceValue as Record<string, unknown>,
          );
        } else {
          target[key] = sourceValue;
        }
      }
      return target;
    };

    try {
      const victim: Record<string, unknown> = {};
      vulnerableDeepMerge(victim, malicious);

      const innocent: { globallyInjected?: unknown } = {};
      expect(innocent.globallyInjected).toBe("yes");

      const safe = mergeDeep({}, removePrototypeDeep(malicious));
      expect(Object.hasOwn(safe, "constructor")).toBe(false);
    } finally {
      Reflect.deleteProperty(Object.prototype, "globallyInjected");
    }
  });

  it("should leave Object.prototype untouched even when __proto__ appears in the source", () => {
    const payload = JSON.parse('{"__proto__":{"leak":"oops"},"safe":1}');

    mergeDeep({}, payload);

    const clean: { leak?: unknown } = {};
    expect(clean.leak).toBeUndefined();
    expect(
      // biome-ignore lint/suspicious/noExplicitAny: ignore
      (Object.prototype as any).leak,
    ).toBeUndefined();
  });

  it("should handle deeply nested merges with many sources", () => {
    const sources = Array.from({ length: 10 }, (_, i) => ({
      level: { [`key${i}`]: i },
    }));

    const result = mergeDeep({ level: { initial: true } }, ...sources);

    expect(result.level.initial).toBe(true);
    for (let index = 0; index < 10; index++) {
      // biome-ignore lint/suspicious/noExplicitAny: dynamic shape
      expect((result.level as any)[`key${index}`]).toBe(index);
    }
  });

  it("should not mutate any of the sources with deeply nested data", () => {
    const source = { a: { b: { c: 1 } } };
    const sourceCopy = JSON.parse(JSON.stringify(source));

    mergeDeep({ a: { b: { d: 2 } } }, source);

    expect(source).toEqual(sourceCopy);
  });

  it("should treat arrays as atomic values and not merge them element-wise", () => {
    const target = { list: [1, 2, 3] };
    const source = { list: [9] };

    const result = mergeDeep(target, source);

    expect(result.list).toEqual([9]);
  });

  it("should return a new object, not the target reference", () => {
    const target = { a: 1 };
    const source = { b: 2 };

    const result = mergeDeep(target, source);

    expect(result).not.toBe(target);
  });
});
