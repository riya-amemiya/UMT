import { removePrototypeMapDeep } from "@/Object/removePrototypeMapDeep";

describe("removePrototypeMapDeep", () => {
  it("should remove prototype polluting properties recursively from each object in the array", () => {
    const objects = [
      JSON.parse('{"safe":{"__proto__":{"polluted":true},"value":1}}'),
      JSON.parse(
        '{"list":[{"prototype":{"polluted":true},"ok":1}],"meta":{"constructor":{"prototype":{"polluted":true}},"keep":2}}',
      ),
    ];

    const result = removePrototypeMapDeep(objects);

    expect(result).toEqual([
      { safe: { value: 1 } },
      { list: [{ ok: 1 }], meta: { keep: 2 } },
    ]);
    expect(Object.hasOwn(result[0].safe, "__proto__")).toBe(false);
    expect(Object.hasOwn(result[1].list[0], "prototype")).toBe(false);
    expect(Object.hasOwn(result[1].meta, "constructor")).toBe(false);
  });

  it("should return an empty array when given an empty array", () => {
    const result = removePrototypeMapDeep([]);

    expect(result).toEqual([]);
  });

  it("should return a new array instance", () => {
    const objects = [{ a: 1 }];

    const result = removePrototypeMapDeep(objects);

    expect(result).not.toBe(objects);
    expect(Array.isArray(result)).toBe(true);
  });

  it("should preserve element order", () => {
    const objects = [
      { id: 1, value: { x: 1 } },
      { id: 2, value: { x: 2 } },
      { id: 3, value: { x: 3 } },
    ];

    const result = removePrototypeMapDeep(objects);

    expect(result.map((item) => item.id)).toEqual([1, 2, 3]);
  });

  it("should give every nested plain object a null prototype", () => {
    const objects = [
      { a: { b: 1 } },
      JSON.parse('{"__proto__":{"polluted":true},"nested":{"deep":{"x":1}}}'),
    ];

    const result = removePrototypeMapDeep(objects);

    for (const item of result) {
      expect(Object.getPrototypeOf(item)).toBeNull();
    }
    expect(Object.getPrototypeOf(result[0].a)).toBeNull();
    // biome-ignore lint/suspicious/noExplicitAny: dynamic shape
    expect(Object.getPrototypeOf((result[1] as any).nested)).toBeNull();
    // biome-ignore lint/suspicious/noExplicitAny: dynamic shape
    expect(Object.getPrototypeOf((result[1] as any).nested.deep)).toBeNull();
  });

  it("should not mutate any of the input objects", () => {
    const objects = [
      JSON.parse('{"outer":{"__proto__":{"polluted":true},"ok":1}}'),
    ];
    const snapshot = JSON.parse(JSON.stringify(objects));

    removePrototypeMapDeep(objects);

    // Note: JSON roundtrip drops __proto__, but the original still has it
    expect(Object.hasOwn(objects[0].outer, "__proto__")).toBe(true);
    expect(objects[0].outer.ok).toBe(snapshot[0].outer.ok);
  });

  it("should sanitize dangerous keys nested inside arrays", () => {
    const objects = [
      JSON.parse(
        '{"items":[{"__proto__":{"polluted":true},"a":1},{"constructor":{"polluted":true},"b":2}]}',
      ),
    ];

    const result = removePrototypeMapDeep(objects);

    expect(result[0].items).toEqual([{ a: 1 }, { b: 2 }]);
    expect(Object.hasOwn(result[0].items[0], "__proto__")).toBe(false);
    expect(Object.hasOwn(result[0].items[1], "constructor")).toBe(false);
  });

  it("should demonstrate that a naive recursive clone locally pollutes nested prototypes, while the deep map helper prevents it", () => {
    const payloads = [JSON.parse('{"a":{"__proto__":{"injected":"deep"}}}')];

    const vulnerableClone = (value: unknown): unknown => {
      if (value === null || typeof value !== "object") {
        return value;
      }
      if (Array.isArray(value)) {
        return value.map((item) => vulnerableClone(item));
      }
      const result: Record<string, unknown> = {};
      for (const key of Object.keys(value as Record<string, unknown>)) {
        result[key] = vulnerableClone((value as Record<string, unknown>)[key]);
      }
      return result;
    };

    // The naive clone hits the __proto__ setter when assigning the cloned
    // value back, so the nested child inherits from the injected object.
    const cloned = vulnerableClone(payloads[0]) as {
      a: { injected?: unknown };
    };
    expect(Object.getPrototypeOf(cloned.a)).toEqual({ injected: "deep" });
    expect(cloned.a.injected).toBe("deep");

    // After sanitization the nested object no longer carries the injected
    // prototype.
    const safe = removePrototypeMapDeep(payloads) as {
      a: { injected?: unknown };
    }[];
    expect(safe).toEqual([{ a: {} }]);
    const safeInner = safe[0].a;
    expect(Object.getPrototypeOf(safeInner)).toBeNull();
    expect(safeInner.injected).toBeUndefined();
  });

  it("should accept a readonly array", () => {
    const objects: readonly Record<string, unknown>[] = [
      JSON.parse('{"outer":{"__proto__":{"polluted":true},"ok":1}}'),
    ];

    const result = removePrototypeMapDeep(objects);

    expect(result).toEqual([{ outer: { ok: 1 } }]);
  });

  it("should handle arrays that mix sanitized and already-safe objects", () => {
    const objects = [
      { clean: { a: 1 } },
      JSON.parse('{"dirty":{"__proto__":{"polluted":true},"b":2}}'),
    ];

    const result = removePrototypeMapDeep(objects);

    expect(result).toEqual([{ clean: { a: 1 } }, { dirty: { b: 2 } }]);
  });
});
