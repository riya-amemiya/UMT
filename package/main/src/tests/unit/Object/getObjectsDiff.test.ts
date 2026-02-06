import { getObjectsDiff } from "@/Object/getObjectsDiff";

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
});
