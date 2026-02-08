import { mapValues } from "@/Object/mapValues";

describe("mapValues", () => {
  it("should transform values using the provided function", () => {
    const result = mapValues(
      { a: 1, b: 2, c: 3 },
      (value) => (value as number) * 2,
    );
    expect(result).toEqual({ a: 2, b: 4, c: 6 });
  });

  it("should pass value and key to the transformer", () => {
    const result = mapValues(
      { x: 10, y: 20 },
      (value, key) => `${key}=${value}`,
    );
    expect(result).toEqual({ x: "x=10", y: "y=20" });
  });

  it("should handle an empty object", () => {
    const result = mapValues({}, (value) => value);
    expect(result).toEqual({});
  });

  it("should not modify the original object", () => {
    const original = { a: 1, b: 2 };
    mapValues(original, (value) => (value as number) * 2);
    expect(original).toEqual({ a: 1, b: 2 });
  });

  it("should handle different return types", () => {
    const result = mapValues(
      { a: 1, b: 2, c: 3 },
      (value) => (value as number) > 1,
    );
    expect(result).toEqual({ a: false, b: true, c: true });
  });
});
