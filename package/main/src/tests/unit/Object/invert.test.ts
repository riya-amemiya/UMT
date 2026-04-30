import { invert } from "@/Object/invert";

describe("invert", () => {
  it("swaps keys and values", () => {
    expect(invert({ a: 1, b: 2 })).toEqual({ 1: "a", 2: "b" });
  });

  it("handles string-to-string swap", () => {
    expect(invert({ x: "X", y: "Y" })).toEqual({ X: "x", Y: "y" });
  });

  it("returns empty object for empty input", () => {
    expect(invert({})).toEqual({});
  });

  it("overwrites earlier keys when values collide", () => {
    expect(invert({ a: 1, b: 1 })).toEqual({ 1: "b" });
  });

  it("does not mutate the input", () => {
    const input = { a: 1, b: 2 };
    invert(input);
    expect(input).toEqual({ a: 1, b: 2 });
  });
});
