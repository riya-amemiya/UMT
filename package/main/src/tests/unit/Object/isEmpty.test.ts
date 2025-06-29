import { isEmpty } from "@/Object/isEmpty";

describe("isEmpty", () => {
  it("should return true for empty object", () => {
    const result = isEmpty({});

    expect(result).toBe(true);
  });

  it("should return false for object with properties", () => {
    const result = isEmpty({ a: 1 });

    expect(result).toBe(false);
  });

  it("should return false for object with multiple properties", () => {
    const object = { a: 1, b: 2, c: 3 };
    const result = isEmpty(object);

    expect(result).toBe(false);
  });

  it("should return false for object with null values", () => {
    const result = isEmpty({ a: null });

    expect(result).toBe(false);
  });

  it("should return false for object with undefined values", () => {
    const result = isEmpty({ a: undefined });

    expect(result).toBe(false);
  });

  it("should return false for object with false values", () => {
    const result = isEmpty({ a: false });

    expect(result).toBe(false);
  });

  it("should return false for object with zero values", () => {
    const result = isEmpty({ a: 0 });

    expect(result).toBe(false);
  });

  it("should return false for object with empty string values", () => {
    const result = isEmpty({ a: "" });

    expect(result).toBe(false);
  });

  it("should return false for object with empty array values", () => {
    const result = isEmpty({ a: [] });

    expect(result).toBe(false);
  });

  it("should return false for object with nested empty object values", () => {
    const result = isEmpty({ a: {} });

    expect(result).toBe(false);
  });

  it("should only check own properties", () => {
    const proto = { inherited: true };
    const object = Object.create(proto);

    const result = isEmpty(object);

    expect(result).toBe(true);
  });

  it("should handle object with symbol properties", () => {
    const symbol = Symbol("test");
    const object = { [symbol]: "value" };

    const result = isEmpty(object);

    expect(result).toBe(false);
  });
});
