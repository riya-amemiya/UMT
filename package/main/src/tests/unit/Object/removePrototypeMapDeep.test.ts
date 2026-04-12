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
});
