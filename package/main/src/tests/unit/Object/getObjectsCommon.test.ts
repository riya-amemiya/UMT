import { getObjectsCommon } from "@/Object/getObjectsCommon";

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
});
