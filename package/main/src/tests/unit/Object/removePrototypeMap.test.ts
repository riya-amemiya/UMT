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
});
