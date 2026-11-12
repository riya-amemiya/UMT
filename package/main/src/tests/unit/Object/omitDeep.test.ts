import { omitDeep } from "@/Object/omitDeep";

describe("omitDeep", () => {
  it("removes simple keys", () => {
    expect(omitDeep({ a: 1, b: 2, c: 3 }, "a", "c")).toEqual({ b: 2 });
  });

  it("removes nested keys", () => {
    const obj = { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 };
    expect(omitDeep(obj, "a.b.c", "f")).toEqual({
      a: { b: { d: 2 }, e: 3 },
    });
  });

  it("does not mutate the input", () => {
    const obj = { a: { b: 1, c: 2 }, d: { e: 3 } };
    const result = omitDeep(obj, "a.b");
    expect(obj).toEqual({ a: { b: 1, c: 2 }, d: { e: 3 } });
    expect(result.d).toBe(obj.d);
  });

  it("returns a shallow copy when no keys are given", () => {
    const obj = { a: 1 };
    const result = omitDeep(obj);
    expect(result).toEqual({ a: 1 });
    expect(result).not.toBe(obj);
  });

  it("ignores missing keys", () => {
    const obj = { a: { b: 1 } };
    // @ts-expect-error
    expect(omitDeep(obj, "a.x", "z")).toEqual({ a: { b: 1 } });
  });

  it("removes a whole nested object", () => {
    expect(omitDeep({ a: { b: 1 }, c: 2 }, "a")).toEqual({ c: 2 });
  });

  it("removes keys from objects inside arrays by index", () => {
    const obj = { a: [{ b: 1, c: 2 }] };
    // @ts-expect-error
    expect(omitDeep(obj, "a.0.b")).toEqual({ a: [{ c: 2 }] });
  });
});
