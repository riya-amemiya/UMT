import { omit } from "@/Object/omit";

describe("omit", () => {
  it("should omit specified keys", () => {
    const object = { a: 1, b: 2, c: 3, d: 4 };

    const result = omit(object, "b", "d");

    expect(result).toEqual({ a: 1, c: 3 });
  });

  it("should not modify original object", () => {
    const object = { a: 1, b: 2, c: 3 };

    const result = omit(object, "b");

    expect(object).toEqual({ a: 1, b: 2, c: 3 });
    expect(result).toEqual({ a: 1, c: 3 });
  });

  it("should handle non-existent keys", () => {
    const object = { a: 1, b: 2 };

    const result = omit(object, "c" as keyof typeof object);

    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should handle empty object", () => {
    const object = {};

    const result = omit(object, "a" as keyof typeof object);

    expect(result).toEqual({});
  });

  it("should handle no keys to omit", () => {
    const object = { a: 1, b: 2, c: 3 };

    const result = omit(object);

    expect(result).toEqual({ a: 1, b: 2, c: 3 });
    expect(result).not.toBe(object);
  });

  it("should handle omitting all keys", () => {
    const object = { a: 1, b: 2 };

    const result = omit(object, "a", "b");

    expect(result).toEqual({});
  });

  it("should handle various value types", () => {
    const object = {
      string: "test",
      number: 42,
      boolean: true,
      array: [1, 2, 3],
      object: { nested: true },
      null: null,
      undefined: undefined,
    };

    const result = omit(object, "string", "array", "null");

    expect(result).toEqual({
      number: 42,
      boolean: true,
      object: { nested: true },
      undefined: undefined,
    });
  });

  it("should preserve type information", () => {
    interface TestObject {
      a: number;
      b: string;
      c: boolean;
    }

    const object: TestObject = { a: 1, b: "test", c: true };

    const result = omit(object, "b");

    expect(result).toEqual({ a: 1, c: true });
    // TypeScript should infer the result type as { a: number; c: boolean; }
  });
});
