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

  it("should return true for Object.create(null) with no own keys", () => {
    const object = Object.create(null);
    expect(isEmpty(object)).toBe(true);
  });

  it("should return false for Object.create(null) with own keys", () => {
    const object = Object.create(null);
    object.a = 1;
    expect(isEmpty(object)).toBe(false);
  });

  it("should return true when only non-enumerable own keys exist", () => {
    const object = {};
    Object.defineProperty(object, "hidden", {
      value: 1,
      enumerable: false,
    });
    expect(isEmpty(object)).toBe(true);
  });

  it("should return false when only a non-enumerable symbol property exists", () => {
    const object = {};
    const symbol = Symbol("hidden");
    Object.defineProperty(object, symbol, {
      value: 1,
      enumerable: false,
    });
    // getOwnPropertySymbols returns both enumerable and non-enumerable symbols.
    expect(isEmpty(object)).toBe(false);
  });

  it("should return false when a string key and a symbol key coexist", () => {
    const object = { a: 1, [Symbol("b")]: 2 };
    expect(isEmpty(object)).toBe(false);
  });

  it("should not be confused by a polluted Object.prototype", () => {
    try {
      // biome-ignore lint/suspicious/noExplicitAny: controlled pollution
      (Object.prototype as any).injected = "bad";
      expect(isEmpty({})).toBe(true);
    } finally {
      Reflect.deleteProperty(Object.prototype, "injected");
    }
  });

  it("should return false for an object with a single property after removing it and adding a new one", () => {
    const object: { a?: number; b?: number } = { a: 1 };
    Reflect.deleteProperty(object, "a");
    expect(isEmpty(object as Record<string, unknown>)).toBe(true);
    object.b = 2;
    expect(isEmpty(object as Record<string, unknown>)).toBe(false);
  });
});
