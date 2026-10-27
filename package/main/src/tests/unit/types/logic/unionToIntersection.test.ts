import type { UnionToIntersection } from "$/logic/unionToIntersection";

describe("UnionToIntersection", () => {
  it("UnionToIntersection<{ a: 1 } | { b: 2 }>", () => {
    const _: UnionToIntersection<{ a: 1 } | { b: 2 }> = { a: 1, b: 2 };
    expect(_).toEqual(_);
  });
  it("{ a: 1 } extends UnionToIntersection<{ a: 1 } | { b: 2 }>", () => {
    const _: { a: 1 } extends UnionToIntersection<{ a: 1 } | { b: 2 }>
      ? true
      : false = false;
    expect(_).toEqual(_);
  });
});
