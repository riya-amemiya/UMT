import { getObjectsDiff } from "@/Object/getObjectsDiff";
import { removePrototype } from "@/Object/removePrototype";

describe("getObjectsDiff", () => {
  test("should find diff key-value pairs between two objects", () => {
    expect(getObjectsDiff({ a: 1, b: 2 }, { b: 2, c: 3 })).toEqual({
      a: 1,
      c: 3,
    });
  });

  test("should find diff key-value pairs among three objects", () => {
    expect(
      getObjectsDiff(
        { a: 1, b: 2, c: 3 },
        { b: 2, c: 3, d: 4 },
        { c: 3, d: 4, e: 5 },
      ),
    ).toEqual({ a: 1, e: 5 });
  });

  test("should return a shallow copy for a single object", () => {
    const obj = { a: 1, b: 2 };
    const result = getObjectsDiff(obj);
    expect(result).toEqual({ a: 1, b: 2 });
    expect(result).not.toBe(obj);
  });

  test("should return empty object when objects are identical", () => {
    expect(getObjectsDiff({ a: 1, b: 2 }, { a: 1, b: 2 })).toEqual({});
  });

  test("should return all pairs when no keys overlap", () => {
    expect(getObjectsDiff({ a: 1 }, { b: 2 }, { c: 3 })).toEqual({
      a: 1,
      b: 2,
      c: 3,
    });
  });

  test("should handle empty objects", () => {
    expect(getObjectsDiff({}, { a: 1 })).toEqual({ a: 1 });
    expect(getObjectsDiff({ a: 1 }, {})).toEqual({ a: 1 });
    expect(getObjectsDiff({}, {})).toEqual({});
  });

  test("should use last value when same key has different unique values", () => {
    expect(getObjectsDiff({ a: 1 }, { a: 2 })).toEqual({ a: 2 });
  });

  test("should handle same key with shared and unique values across three objects", () => {
    expect(getObjectsDiff({ a: 1 }, { a: 1 }, { a: 2 })).toEqual({ a: 2 });
  });

  test("should handle falsy values correctly", () => {
    expect(getObjectsDiff({ a: null, b: 0 }, { a: null, b: 0 })).toEqual({});
  });

  test("should handle different falsy values", () => {
    expect(getObjectsDiff({ a: null }, { a: undefined })).toEqual({
      a: undefined,
    });
  });

  test("should find diff in nested objects recursively", () => {
    expect(
      getObjectsDiff({ a: { b: 1, c: 2 }, d: 3 }, { a: { b: 1, d: 4 }, d: 3 }),
    ).toEqual({ a: { c: 2, d: 4 } });
  });

  test("should exclude key when nested diff is empty", () => {
    expect(getObjectsDiff({ a: { b: 1 } }, { a: { b: 1 } })).toEqual({});
  });

  test("should handle deeply nested objects", () => {
    expect(
      getObjectsDiff(
        { a: { b: { c: { d: 1, e: 2 } } } },
        { a: { b: { c: { d: 1, f: 3 } } } },
      ),
    ).toEqual({ a: { b: { c: { e: 2, f: 3 } } } });
  });

  test("should handle nested objects among three objects", () => {
    expect(
      getObjectsDiff(
        { a: { b: 1, c: 2 } },
        { a: { b: 1, d: 3 } },
        { a: { b: 1, e: 4 } },
      ),
    ).toEqual({ a: { c: 2, d: 3, e: 4 } });
  });

  test("should handle nested shared values across three objects", () => {
    expect(
      getObjectsDiff({ a: { b: 1 } }, { a: { b: 1 } }, { a: { b: 2 } }),
    ).toEqual({ a: { b: 2 } });
  });

  test("should handle mixed nested and primitive values", () => {
    expect(getObjectsDiff({ a: { b: 1 } }, { a: "hello" })).toEqual({
      a: "hello",
    });
  });

  test("should compare arrays by reference", () => {
    const arr = [1, 2, 3];
    expect(getObjectsDiff({ a: arr }, { a: arr })).toEqual({});
    expect(getObjectsDiff({ a: [1, 2, 3] }, { a: [1, 2, 3] })).toEqual({
      a: [1, 2, 3],
    });
  });

  test("should not mutate original objects", () => {
    const obj1 = { a: 1, b: { c: 2 } };
    const obj2 = { a: 1, b: { c: 2, d: 3 } };
    const obj1Copy = JSON.parse(JSON.stringify(obj1));
    const obj2Copy = JSON.parse(JSON.stringify(obj2));

    getObjectsDiff(obj1, obj2);

    expect(obj1).toEqual(obj1Copy);
    expect(obj2).toEqual(obj2Copy);
  });

  it("should prevent prototype pollution via __proto__", () => {
    const obj1 = JSON.parse('{"__proto__": {"polluted": true}}');
    const obj2 = { a: 1 };

    const result = getObjectsDiff(removePrototype(obj1), removePrototype(obj2));

    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).polluted).toBeUndefined();
  });

  test("should compare using strict equality (no type coercion)", () => {
    expect(
      getObjectsDiff(
        { a: 1 } as Record<string, unknown>,
        { a: "1" } as Record<string, unknown>,
      ),
    ).toEqual({ a: "1" });
    expect(
      getObjectsDiff(
        { a: true } as Record<string, unknown>,
        { a: 1 } as Record<string, unknown>,
      ),
    ).toEqual({ a: 1 });
  });

  test("should handle NaN with SameValueZero semantics (NaN equals NaN in the internal Map)", () => {
    // Map keys use SameValueZero, which equates NaN with NaN. As a result,
    // two identical NaN entries count as shared rather than unique.
    expect(getObjectsDiff({ a: Number.NaN }, { a: Number.NaN })).toEqual({});
  });

  test("should include keys present in only some of the objects", () => {
    expect(
      getObjectsDiff(
        { a: 1, only1: "x" },
        { a: 1, only2: "y" },
        { a: 1, only3: "z" },
      ),
    ).toEqual({ only1: "x", only2: "y", only3: "z" });
  });

  test("should keep the last unique value when 4+ objects have conflicting values", () => {
    expect(getObjectsDiff({ x: 1 }, { x: 2 }, { x: 3 }, { x: 4 })).toEqual({
      x: 4,
    });
  });

  test("should not include a key when all values are identical across many objects", () => {
    expect(getObjectsDiff({ x: 1 }, { x: 1 }, { x: 1 }, { x: 1 })).toEqual({});
  });

  test("should not include a key when exactly one value is shared and one is unique (the unique wins)", () => {
    expect(getObjectsDiff({ x: 1 }, { x: 1 }, { x: 2 })).toEqual({ x: 2 });
  });

  test("should pair shared values correctly across objects even when the shared value is not first", () => {
    expect(getObjectsDiff({ x: 3 }, { x: 1 }, { x: 1 })).toEqual({ x: 3 });
  });

  test("should handle keys that exist in only a subset of the inputs", () => {
    expect(getObjectsDiff({ a: 1 }, { b: 2 }, { a: 1, b: 2 })).toEqual({});
    expect(getObjectsDiff({ a: 1 }, { b: 2 }, { a: 3, b: 4 })).toEqual({
      a: 3,
      b: 4,
    });
  });

  test("should iterate own enumerable keys only (skip inherited)", () => {
    const parent = { inherited: 1 };
    const child: { own?: number } = Object.create(parent);
    child.own = 2;

    expect(
      getObjectsDiff(child as Record<string, unknown>, { own: 2, extra: 9 }),
    ).toEqual({
      extra: 9,
    });
  });

  test("should return a new object, not the original", () => {
    const source = { a: 1 };
    const result = getObjectsDiff(source);

    expect(result).toEqual(source);
    expect(result).not.toBe(source);
  });

  test("should pass through unsanitized __proto__ as own key (diff does not filter)", () => {
    // Documents the library's scope: getObjectsDiff does not filter dangerous
    // keys itself. Callers should sanitize with removePrototype* first.
    const obj1 = JSON.parse('{"__proto__":{"polluted":true},"a":1}');
    const obj2 = { a: 1 };

    const result = getObjectsDiff(obj1, obj2) as Record<string, unknown>;

    // __proto__ is present in obj1 only, so it becomes a unique value. The
    // assignment goes through the setter though, so it does not appear as an
    // own key of the result.
    expect(Object.hasOwn(result, "__proto__")).toBe(false);
  });
});
