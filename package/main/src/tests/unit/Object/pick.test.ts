import { pick } from "@/Object/pick";

describe("pick function", () => {
  test("should select a single key", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: 1 });
  });

  test("should select multiple keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("should handle non-existent keys", () => {
    const obj = { a: 1, b: 2 };
    // @ts-expect-error
    const result = pick(obj, "c");
    expect(result).toEqual({});
  });

  test("should handle empty objects", () => {
    const obj = {};
    // @ts-expect-error
    const result = pick(obj, "a");
    expect(result).toEqual({});
  });

  test("should handle no keys specified", () => {
    const obj = { a: 1, b: 2 };
    const result = pick(obj);
    expect(result).toEqual({});
  });

  test("should select all keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "b", "c");
    expect(result).toEqual(obj);
  });

  test("should handle duplicate keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pick(obj, "a", "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("should handle nested objects", () => {
    const obj = { a: { b: 1 }, c: 2 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: { b: 1 } });
  });

  test("should handle objects with null or undefined properties", () => {
    const obj = { a: null, b: undefined, c: 3 };
    const result = pick(obj, "a", "b", "c");
    expect(result).toEqual({ a: null, b: undefined, c: 3 });
  });

  test("should handle objects containing arrays", () => {
    const obj = { a: [1, 2, 3], b: 4 };
    const result = pick(obj, "a");
    expect(result).toEqual({ a: [1, 2, 3] });
  });
});
