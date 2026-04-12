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
});
