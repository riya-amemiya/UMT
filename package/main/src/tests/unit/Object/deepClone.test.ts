import { deepClone } from "@/Object/deepClone";

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
});
