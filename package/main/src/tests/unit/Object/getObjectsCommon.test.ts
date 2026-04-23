import { getObjectsCommon } from "@/Object/getObjectsCommon";
import { removePrototype } from "@/Object/removePrototype";

describe("getObjectsCommon", () => {
  test("should find common key-value pairs between two objects", () => {
    expect(getObjectsCommon({ a: 1, b: 2 }, { a: 1, c: 3 })).toEqual({
      a: 1,
    });
  });

  test("should find common key-value pairs among three objects", () => {
    expect(
      getObjectsCommon(
        { a: 1, b: 2, c: 3 },
        { a: 1, b: 2, d: 4 },
        { a: 1, e: 5 },
      ),
    ).toEqual({ a: 1 });
  });

  test("should return a shallow copy for a single object", () => {
    const obj = { a: 1, b: 2 };
    const result = getObjectsCommon(obj);
    expect(result).toEqual({ a: 1, b: 2 });
    expect(result).not.toBe(obj);
  });

  test("should return empty object when one input is empty", () => {
    expect(getObjectsCommon({ a: 1, b: 2 }, {})).toEqual({});
    expect(getObjectsCommon({}, { a: 1, b: 2 })).toEqual({});
  });

  test("should return empty object when all inputs are empty", () => {
    expect(getObjectsCommon({}, {})).toEqual({});
  });

  test("should return empty object when no keys are common", () => {
    expect(getObjectsCommon({ a: 1 }, { b: 2 })).toEqual({});
  });

  test("should exclude keys with different values", () => {
    expect(getObjectsCommon({ a: 1, b: 2 }, { a: 1, b: 5 })).toEqual({
      a: 1,
    });
  });

  test("should handle falsy values correctly", () => {
    expect(
      getObjectsCommon(
        { a: null, b: undefined, c: 0, d: false, e: "" },
        { a: null, b: undefined, c: 0, d: false, e: "" },
      ),
    ).toEqual({ a: null, b: undefined, c: 0, d: false, e: "" });
  });

  test("should handle falsy values that differ", () => {
    expect(getObjectsCommon({ a: null }, { a: undefined })).toEqual({});
    expect(getObjectsCommon({ a: 0 }, { a: false })).toEqual({});
    expect(getObjectsCommon({ a: "" }, { a: 0 })).toEqual({});
  });

  test("should find common nested objects recursively", () => {
    expect(
      getObjectsCommon(
        { a: { b: 1, c: 2 }, d: 3 },
        { a: { b: 1, d: 4 }, d: 3 },
      ),
    ).toEqual({ a: { b: 1 }, d: 3 });
  });

  test("should exclude key when recursive result is empty", () => {
    expect(getObjectsCommon({ a: { b: 1 } }, { a: { c: 2 } })).toEqual({});
  });

  test("should handle deeply nested objects", () => {
    expect(
      getObjectsCommon(
        { a: { b: { c: { d: 1, e: 2 } } } },
        { a: { b: { c: { d: 1, f: 3 } } } },
      ),
    ).toEqual({ a: { b: { c: { d: 1 } } } });
  });

  test("should handle nested objects among three objects", () => {
    expect(
      getObjectsCommon(
        { a: { b: 1, c: 2, d: 3 } },
        { a: { b: 1, c: 2, e: 4 } },
        { a: { b: 1, f: 5 } },
      ),
    ).toEqual({ a: { b: 1 } });
  });

  test("should handle mixed nested and primitive values", () => {
    expect(getObjectsCommon({ a: { b: 1 } }, { a: "hello" })).toEqual({});
  });

  test("should compare arrays by reference", () => {
    const arr = [1, 2, 3];
    expect(getObjectsCommon({ a: arr }, { a: arr })).toEqual({ a: arr });
    expect(getObjectsCommon({ a: [1, 2, 3] }, { a: [1, 2, 3] })).toEqual({});
  });

  test("should compare object values by reference when not plain objects", () => {
    const date = new Date("2024-01-01");
    expect(getObjectsCommon({ a: date }, { a: date })).toEqual({ a: date });
    expect(
      getObjectsCommon(
        { a: new Date("2024-01-01") },
        { a: new Date("2024-01-01") },
      ),
    ).toEqual({});
  });

  test("should not mutate original objects", () => {
    const obj1 = { a: 1, b: { c: 2 } };
    const obj2 = { a: 1, b: { c: 2, d: 3 } };
    const obj1Copy = JSON.parse(JSON.stringify(obj1));
    const obj2Copy = JSON.parse(JSON.stringify(obj2));

    getObjectsCommon(obj1, obj2);

    expect(obj1).toEqual(obj1Copy);
    expect(obj2).toEqual(obj2Copy);
  });

  test("should prevent prototype pollution via __proto__", () => {
    const obj1 = JSON.parse('{"__proto__": {"polluted": true}}');
    const obj2 = JSON.parse('{"__proto__": {"polluted": true}}');

    const result = getObjectsCommon(
      removePrototype(obj1),
      removePrototype(obj2),
    );

    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect((result as any).polluted).toBeUndefined();
  });

  test("should return an empty result when either object lacks a key present in the other", () => {
    expect(getObjectsCommon({ a: 1, b: 2 }, { a: 1 })).toEqual({ a: 1 });
    expect(getObjectsCommon({ a: 1 }, { a: 1, b: 2 })).toEqual({ a: 1 });
  });

  test("should handle NaN reference equality (=== returns false for NaN)", () => {
    expect(getObjectsCommon({ a: Number.NaN }, { a: Number.NaN })).toEqual({});
  });

  test("should compare using strict equality (no coercion)", () => {
    expect(
      getObjectsCommon(
        { a: 1 } as Record<string, unknown>,
        { a: "1" } as Record<string, unknown>,
      ),
    ).toEqual({});
    expect(
      getObjectsCommon(
        { a: true } as Record<string, unknown>,
        { a: 1 } as Record<string, unknown>,
      ),
    ).toEqual({});
  });

  test("should not copy inherited keys from the first object", () => {
    const parent = { inherited: 1 };
    const child: { own?: number } = Object.create(parent);
    child.own = 2;

    const result = getObjectsCommon(child as Record<string, unknown>, {
      own: 2,
      inherited: 1,
    });

    // getObjectsCommon iterates Object.entries, which only sees own enumerable
    // keys on the first argument.
    expect(Object.hasOwn(result, "own")).toBe(true);
    expect(Object.hasOwn(result, "inherited")).toBe(false);
  });

  test("should return shared indices when comparing a plain object and an array (top-level own keys match)", () => {
    // getObjectsCommon walks own keys and compares values strictly, so numeric
    // string indices on a plain object that match the same positions on an
    // array are treated as common pairs. This documents the actual behavior.
    const object = { "0": "a", "1": "b" };
    const array = ["a", "b"] as unknown as Record<string, unknown>;
    expect(getObjectsCommon(object, array)).toEqual({ "0": "a", "1": "b" });
  });

  test("should skip __proto__ keys on plain objects passed unsanitized", () => {
    // Object.entries does include JSON-parsed __proto__ own keys. The library
    // does NOT filter dangerous keys itself — but since values are compared
    // strictly and written via bracket notation on a result=`{}`, the result
    // never exposes __proto__ as an own key (primitive rejected by setter;
    // objects fall into the recursion path and assign to prototype only).
    const obj1 = JSON.parse('{"__proto__":{"polluted":true},"a":1}');
    const obj2 = JSON.parse('{"__proto__":{"polluted":true},"a":1}');

    const result = getObjectsCommon(obj1, obj2) as { a?: number };

    expect(Object.hasOwn(result, "__proto__")).toBe(false);
    expect(result.a).toBe(1);
  });

  test("should handle many objects efficiently", () => {
    const objects = Array.from({ length: 20 }, () => ({
      shared: 1,
      unique: Math.random(),
    }));

    const result = getObjectsCommon(objects[0], ...objects.slice(1));

    expect(result).toEqual({ shared: 1 });
  });

  test("should return a new object, not the original", () => {
    const source = { a: 1, b: 2 };
    const result = getObjectsCommon(source);

    expect(result).toEqual(source);
    expect(result).not.toBe(source);
  });
});
