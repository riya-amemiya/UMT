import { has } from "@/Object/has";

describe("has", () => {
  it("should return true for existing nested path (string)", () => {
    expect(has({ a: { b: 1 } }, "a.b")).toBe(true);
  });

  it("should return true for existing nested path (array)", () => {
    expect(has({ a: { b: 1 } }, ["a", "b"])).toBe(true);
  });

  it("should return false for non-existing nested path (string)", () => {
    expect(has({ a: { b: 1 } }, "a.c")).toBe(false);
  });

  it("should return false for non-existing nested path (array)", () => {
    expect(has({ a: { b: 1 } }, ["a", "c"])).toBe(false);
  });

  it("should return false for non-existing top-level path", () => {
    expect(has({ a: { b: 1 } }, "b")).toBe(false);
  });

  it("should handle empty path (string)", () => {
    expect(has({ a: { b: 1 } }, "")).toBe(false);
  });

  it("should handle empty path (array)", () => {
    expect(has({ a: { b: 1 } }, [])).toBe(true);
  });

  it("should handle null or undefined object", () => {
    // @ts-expect-error
    expect(has(null, "a.b")).toBe(false);
    // @ts-expect-error
    expect(has(undefined, "a.b")).toBe(false);
  });

  it("should block __proto__ key to prevent prototype pollution", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, "__proto__")).toBe(false);
    expect(has(obj, "__proto__.polluted")).toBe(false);
    expect(has(obj, "a.__proto__")).toBe(false);
  });

  it("should block constructor key to prevent prototype pollution", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, "constructor")).toBe(false);
    expect(has(obj, "constructor.prototype")).toBe(false);
  });

  it("should block prototype key to prevent prototype pollution", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, "prototype")).toBe(false);
  });

  it("should block dangerous keys passed as array path", () => {
    const obj = { a: { b: 1 } };
    expect(has(obj, ["__proto__"])).toBe(false);
    expect(has(obj, ["constructor"])).toBe(false);
    expect(has(obj, ["prototype"])).toBe(false);
    expect(has(obj, ["a", "__proto__"])).toBe(false);
  });
});
