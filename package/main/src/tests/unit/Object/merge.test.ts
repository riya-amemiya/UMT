import { merge } from "@/Object/merge";

describe("merge", () => {
  it("should merge multiple objects", () => {
    const target = { a: 1, b: 2 };
    const source1 = { b: 3, c: 4 };
    const source2 = { c: 5, d: 6 };

    const result = merge(target, source1, source2);
    expect(result).toEqual({ a: 1, b: 3, c: 5, d: 6 });
  });

  it("should not modify original objects", () => {
    const target = { a: 1, b: 2 };
    const source = { b: 3, c: 4 };

    const result = merge(target, source);

    expect(target).toEqual({ a: 1, b: 2 });
    expect(source).toEqual({ b: 3, c: 4 });
    expect(result).toEqual({ a: 1, b: 3, c: 4 });
  });

  it("should handle empty target", () => {
    const target = {};
    const source = { a: 1, b: 2 };

    const result = merge(target, source);

    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should handle no sources", () => {
    const target = { a: 1, b: 2 };

    const result = merge(target);

    expect(result).toEqual({ a: 1, b: 2 });
    expect(result).not.toBe(target);
  });

  it("should handle empty sources", () => {
    const target = { a: 1, b: 2 };
    const source = {};

    const result = merge(target, source);

    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should override properties with later sources", () => {
    const target = { a: 1 };
    const source1 = { a: 2 };
    const source2 = { a: 3 };

    const result = merge(target, source1, source2);

    expect(result).toEqual({ a: 3 });
  });

  it("should handle null and undefined values", () => {
    const target = { a: 1, b: null };
    const source = { b: undefined, c: null };

    const result = merge(target, source);

    expect(result).toEqual({ a: 1, b: undefined, c: null });
  });
});
