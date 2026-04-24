import { removePrototypeMap } from "@/Object/removePrototypeMap";

describe("removePrototypeMap", () => {
  it("should remove prototype polluting properties from each object in the array", () => {
    const nested = { keep: true };
    const objects = [
      JSON.parse('{"__proto__":{"polluted":true},"a":1}'),
      {
        constructor: { prototype: { polluted: true } },
        b: 2,
        nested,
      },
      {
        prototype: { polluted: true },
        c: 3,
      },
    ];

    const result = removePrototypeMap(objects);

    expect(result).toEqual([{ a: 1 }, { b: 2, nested }, { c: 3 }]);
    expect(result).not.toBe(objects);
    expect(result[0]).not.toBe(objects[0]);
    // biome-ignore lint/complexity/useLiteralKeys: ignore
    expect(result[1]["nested"]).toBe(nested);
  });

  it("should return an empty array when given an empty array", () => {
    const result = removePrototypeMap([]);

    expect(result).toEqual([]);
  });

  it("should return a new array instance", () => {
    const objects = [{ a: 1 }];

    const result = removePrototypeMap(objects);

    expect(result).not.toBe(objects);
    expect(Array.isArray(result)).toBe(true);
  });

  it("should preserve element order", () => {
    const objects = [{ a: 1 }, { a: 2 }, { a: 3 }, { a: 4 }];

    const result = removePrototypeMap(objects);

    expect(result.map((item) => item.a)).toEqual([1, 2, 3, 4]);
  });

  it("should give every returned object a null prototype", () => {
    const objects = [
      { a: 1 },
      JSON.parse('{"__proto__":{"polluted":true},"b":2}'),
    ];

    const result = removePrototypeMap(objects);

    for (const item of result) {
      expect(Object.getPrototypeOf(item)).toBeNull();
    }
  });

  it("should only sanitize the top level of each element (shallow)", () => {
    const nestedMalicious = JSON.parse(
      '{"__proto__":{"polluted":true},"deep":1}',
    );
    const objects = [{ nested: nestedMalicious }];

    const result = removePrototypeMap(objects);

    expect(Object.hasOwn(result[0].nested, "__proto__")).toBe(true);
    expect(result[0].nested).toBe(nestedMalicious);
  });

  it("should demonstrate that naive loop assignments pollute and the map helper prevents it", () => {
    const payloads = [
      JSON.parse('{"__proto__":{"injected":"a"},"safe":1}'),
      JSON.parse('{"__proto__":{"injected":"b"},"safe":2}'),
    ];

    // A mutable loop assignment uses [[Set]], which triggers the __proto__
    // setter and replaces each target's prototype with the attacker's object.
    // (Spread syntax uses [[DefineOwnProperty]] and would NOT reproduce this.)
    const vulnerable = payloads.map((p) => {
      const out: { injected?: unknown } = {};
      for (const key of Object.keys(p)) {
        (out as Record<string, unknown>)[key] = (p as Record<string, unknown>)[
          key
        ];
      }
      return out;
    });
    expect(Object.getPrototypeOf(vulnerable[0])).toEqual({ injected: "a" });
    expect(Object.getPrototypeOf(vulnerable[1])).toEqual({ injected: "b" });
    expect(vulnerable[0].injected).toBe("a");
    expect(vulnerable[1].injected).toBe("b");

    const safe = removePrototypeMap(payloads) as { injected?: unknown }[];
    expect(safe).toEqual([{ safe: 1 }, { safe: 2 }]);
    for (const item of safe) {
      expect(Object.getPrototypeOf(item)).toBeNull();
      expect(item.injected).toBeUndefined();
    }
  });

  it("should not mutate any of the input objects", () => {
    const objects = [
      JSON.parse('{"__proto__":{"polluted":true},"a":1}'),
      JSON.parse('{"constructor":{"polluted":true},"b":2}'),
    ];

    removePrototypeMap(objects);

    expect(Object.hasOwn(objects[0], "__proto__")).toBe(true);
    expect(Object.hasOwn(objects[1], "constructor")).toBe(true);
  });

  it("should accept a readonly array", () => {
    const objects: readonly Record<string, unknown>[] = [
      JSON.parse('{"__proto__":{"polluted":true},"a":1}'),
    ];

    const result = removePrototypeMap(objects);

    expect(result).toEqual([{ a: 1 }]);
  });
});
