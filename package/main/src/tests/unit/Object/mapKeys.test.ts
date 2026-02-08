import { mapKeys } from "@/Object/mapKeys";

describe("mapKeys", () => {
  it("should transform keys using the provided function", () => {
    const result = mapKeys({ a: 1, b: 2, c: 3 }, (_value, key) =>
      key.toUpperCase(),
    );
    expect(result).toEqual({ A: 1, B: 2, C: 3 });
  });

  it("should pass value and key to the transformer", () => {
    const result = mapKeys({ x: 10, y: 20 }, (value, key) => `${key}_${value}`);
    expect(result).toEqual({ x_10: 10, y_20: 20 });
  });

  it("should handle an empty object", () => {
    const result = mapKeys({}, (_value, key) => key);
    expect(result).toEqual({});
  });

  it("should handle keys that map to the same value", () => {
    const result = mapKeys({ a: 1, b: 2 }, () => "same");
    expect(result).toEqual({ same: 2 });
  });

  it("should not modify the original object", () => {
    const original = { a: 1, b: 2 };
    mapKeys(original, (_value, key) => key.toUpperCase());
    expect(original).toEqual({ a: 1, b: 2 });
  });
});
