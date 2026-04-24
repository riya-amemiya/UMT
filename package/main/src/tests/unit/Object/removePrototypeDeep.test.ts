import { removePrototypeDeep } from "@/Object/removePrototypeDeep";

describe("removePrototypeDeep", () => {
  it("should remove prototype polluting properties recursively", () => {
    const object = JSON.parse(
      '{"safe":{"nested":{"__proto__":{"polluted":true},"value":1}},"list":[{"constructor":{"prototype":{"polluted":true}},"ok":1},{"prototype":{"polluted":true},"safe":2}]}',
    );

    const result = removePrototypeDeep(object);

    expect(result).toEqual({
      safe: {
        nested: {
          value: 1,
        },
      },
      list: [{ ok: 1 }, { safe: 2 }],
    });
    expect(Object.hasOwn(result.safe.nested, "__proto__")).toBe(false);
    expect(Object.hasOwn(result.list[0], "constructor")).toBe(false);
    expect(Object.hasOwn(result.list[1], "prototype")).toBe(false);
  });

  it("should handle arrays containing arrays and primitives", () => {
    const object = JSON.parse(
      '{"matrix":[[{"__proto__":{"polluted":true},"a":1}],["text",42,true,null]]}',
    );

    const result = removePrototypeDeep(object);

    expect(result).toEqual({
      matrix: [[{ a: 1 }], ["text", 42, true, null]],
    });
  });

  it("should not modify the original object", () => {
    const object = JSON.parse(
      '{"safe":{"__proto__":{"polluted":true},"value":1},"list":[{"prototype":{"polluted":true},"ok":1}]}',
    );

    const result = removePrototypeDeep(object);

    expect(result).toEqual({
      safe: { value: 1 },
      list: [{ ok: 1 }],
    });
    expect(Object.hasOwn(object.safe, "__proto__")).toBe(true);
    expect(Object.hasOwn(object.list[0], "prototype")).toBe(true);
  });

  it("should return a root object with a null prototype", () => {
    const object = { a: 1 };
    const result = removePrototypeDeep(object);

    expect(Object.getPrototypeOf(result)).toBeNull();
  });

  it("should give every nested plain object a null prototype", () => {
    const object = {
      a: { b: { c: { d: 1 } } },
      list: [{ x: 1 }, { y: { z: 2 } }],
    };

    const result = removePrototypeDeep(object);

    expect(Object.getPrototypeOf(result)).toBeNull();
    expect(Object.getPrototypeOf(result.a)).toBeNull();
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(Object.getPrototypeOf((result.a as any).b)).toBeNull();
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(Object.getPrototypeOf((result.a as any).b.c)).toBeNull();
    expect(Object.getPrototypeOf(result.list[0])).toBeNull();
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(Object.getPrototypeOf((result.list[1] as any).y)).toBeNull();
  });

  it("should handle an empty object", () => {
    const result = removePrototypeDeep({});

    expect(result).toEqual({});
    expect(Object.getPrototypeOf(result)).toBeNull();
  });

  it("should preserve primitive values verbatim", () => {
    const object = {
      a: 1,
      b: "text",
      c: true,
      d: null,
      e: undefined,
    };

    const result = removePrototypeDeep(object);

    expect(result).toEqual(object);
  });

  it("should pass non-plain objects through by reference", () => {
    const date = new Date("2024-01-01");
    const map = new Map([["k", "v"]]);
    const set = new Set([1, 2]);
    const regex = /foo/;
    const object = { date, map, set, regex };

    const result = removePrototypeDeep(object);

    expect(result.date).toBe(date);
    expect(result.map).toBe(map);
    expect(result.set).toBe(set);
    expect(result.regex).toBe(regex);
  });

  it("should sanitize dangerous keys discovered inside arrays", () => {
    const object = JSON.parse(
      '{"list":[{"__proto__":{"polluted":true},"value":1},{"constructor":{"polluted":true},"value":2}]}',
    );

    const result = removePrototypeDeep(object);

    expect(result.list[0]).toEqual({ value: 1 });
    expect(result.list[1]).toEqual({ value: 2 });
    expect(Object.hasOwn(result.list[0], "__proto__")).toBe(false);
    expect(Object.hasOwn(result.list[1], "constructor")).toBe(false);
  });

  it("should sanitize dangerous keys at multiple depths", () => {
    const object = JSON.parse(
      '{"__proto__":1,"a":{"__proto__":2,"b":{"__proto__":3,"c":4}}}',
    );

    const result = removePrototypeDeep(object);

    expect(result).toEqual({ a: { b: { c: 4 } } });
  });

  it("should return a new array (not the same reference)", () => {
    const inner = [1, 2, 3];
    const object = { list: inner };

    const result = removePrototypeDeep(object);

    expect(result.list).toEqual(inner);
    expect(result.list).not.toBe(inner);
  });

  it("should return new plain objects (not the same reference)", () => {
    const nested = { b: 1 };
    const object = { a: nested };

    const result = removePrototypeDeep(object);

    expect(result.a).toEqual(nested);
    expect(result.a).not.toBe(nested);
  });

  it("should demonstrate that a naive deep merge pollutes Object.prototype without sanitization", () => {
    const malicious = JSON.parse('{"__proto__":{"polluted":"bad"}}');

    const vulnerableDeepMerge = (
      target: Record<string, unknown>,
      source: Record<string, unknown>,
    ): void => {
      for (const key of Object.keys(source)) {
        const sourceValue = source[key];
        const targetValue = target[key];
        if (
          sourceValue !== null &&
          typeof sourceValue === "object" &&
          targetValue !== null &&
          typeof targetValue === "object"
        ) {
          vulnerableDeepMerge(
            targetValue as Record<string, unknown>,
            sourceValue as Record<string, unknown>,
          );
        } else {
          target[key] = sourceValue;
        }
      }
    };

    try {
      const victim: Record<string, unknown> = {};
      vulnerableDeepMerge(victim, malicious);

      const innocent: { polluted?: unknown } = {};
      expect(innocent.polluted).toBe("bad");

      const sanitized = removePrototypeDeep(malicious);
      const victim2: Record<string, unknown> = Object.create(null);
      vulnerableDeepMerge(victim2, sanitized);
      expect(Object.hasOwn(victim2, "polluted")).toBe(false);
    } finally {
      Reflect.deleteProperty(Object.prototype, "polluted");
    }
  });

  it("should recurse through arrays that contain arrays", () => {
    const object = JSON.parse(
      '{"matrix":[[[{"__proto__":{"polluted":true},"deep":1}]]]}',
    );

    const result = removePrototypeDeep(object);

    expect(result).toEqual({ matrix: [[[{ deep: 1 }]]] });
    // biome-ignore lint/suspicious/noExplicitAny: navigating dynamic shape
    expect(Object.hasOwn((result as any).matrix[0][0][0], "__proto__")).toBe(
      false,
    );
  });
});
