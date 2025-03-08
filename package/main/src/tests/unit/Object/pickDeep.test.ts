import { pickDeep } from "@/Object/pickDeep";

describe("pickDeep function", () => {
  test("should select simple keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pickDeep(obj, "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });

  test("should select nested keys", () => {
    const obj = { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 };
    const result = pickDeep(obj, "a.b.c", "f");
    expect(result).toEqual({ a: { b: { c: 1 } }, f: 4 });
  });

  test("should handle non-existent keys", () => {
    const obj = { a: 1, b: 2 };
    // @ts-expect-error
    const result = pickDeep(obj, "a", "c");
    expect(result).toEqual({ a: 1 });
  });

  test("should handle no keys specified", () => {
    const obj = { a: 1, b: 2 };
    const result = pickDeep(obj);
    expect(result).toEqual({});
  });

  test("should handle objects containing arrays", () => {
    const obj = { a: [{ b: 1 }, { c: 2 }], d: 3 };
    const result = pickDeep(obj, "a", "d");
    expect(result).toEqual(obj);
  });

  test("should handle objects with null or undefined properties", () => {
    const obj = { a: null, b: undefined, c: 3 };
    const result = pickDeep(obj, "a", "b", "c");
    expect(result).toEqual({ a: null, b: undefined, c: 3 });
  });

  test("should handle deeply nested keys", () => {
    const obj = {
      a: {
        b: {
          c: {
            d: {
              e: 5,
            },
          },
        },
      },
    };
    const result = pickDeep(obj, "a.b.c.d.e");
    expect(result).toEqual({
      a: {
        b: {
          c: {
            d: {
              e: 5,
            },
          },
        },
      },
    });
  });

  test("should handle keys that reference entire objects", () => {
    const obj = { a: { b: 1, c: 2 }, d: 3 };
    const result = pickDeep(obj, "a");
    expect(result).toEqual({ a: { b: 1, c: 2 } });
  });

  test("should handle keys without dots", () => {
    const obj = { a: { b: 1 }, c: 2 };
    const result = pickDeep(obj, "a", "c");
    expect(result).toEqual({ a: { b: 1 }, c: 2 });
  });

  test("should handle empty string keys", () => {
    const obj = { a: 1, b: 2 };
    // @ts-expect-error
    const result = pickDeep(obj, "");
    expect(result).toEqual({});
  });

  test("should handle keys containing numbers", () => {
    const obj = { a: { "1": 1, "2": 2 }, b: 3 };
    const result = pickDeep(obj, "a.1");
    expect(result).toEqual({ a: { "1": 1 } });
  });

  test("should handle non-existent nested paths", () => {
    const obj = { a: { b: 1 } };
    // @ts-expect-error
    const result = pickDeep(obj, "a.b.c");
    expect(result).toEqual({ a: { b: {} } });
  });

  test("should handle multiple nested keys simultaneously", () => {
    const obj = {
      user: { name: "Alice", address: { city: "Tokyo", zip: "12345" } },
      posts: [
        { id: 1, title: "Post 1" },
        { id: 2, title: "Post 2" },
      ],
    };
    const result = pickDeep(obj, "user.name", "user.address.city", "posts");
    expect(result).toEqual({
      user: { name: "Alice", address: { city: "Tokyo" } },
      posts: [
        { id: 1, title: "Post 1" },
        { id: 2, title: "Post 2" },
      ],
    });
  });

  test("should handle duplicate keys", () => {
    const obj = { a: 1, b: 2, c: 3 };
    const result = pickDeep(obj, "a", "a", "c");
    expect(result).toEqual({ a: 1, c: 3 });
  });
});
