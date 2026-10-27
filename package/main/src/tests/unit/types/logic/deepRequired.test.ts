import type { DeepRequired } from "$/logic/deepRequired";

describe("DeepRequired", () => {
  it("DeepRequired<{ a?: { b?: number } }>", () => {
    const _: DeepRequired<{ a?: { b?: number } }> = { a: { b: 1 } };
    expect(_).toEqual(_);
  });
  it("{ a: object } extends DeepRequired<{ a?: { b?: number } }>", () => {
    const _: { a: object } extends DeepRequired<{ a?: { b?: number } }>
      ? true
      : false = false;
    expect(_).toEqual(_);
  });
  it("DeepRequired<number>", () => {
    const _: DeepRequired<number> = 1;
    expect(_).toEqual(_);
  });
});
