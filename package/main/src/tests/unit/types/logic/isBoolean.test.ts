import type { isBoolean } from "$/logic/isBoolean";

describe("isBoolean", () => {
  it("isBoolean<0>", () => {
    const _: isBoolean<0> = false;
    expect(_).toEqual(_);
  });
  it("isBoolean<1>", () => {
    const _: isBoolean<1> = true;
    expect(_).toEqual(_);
  });
  it('isBoolean<"">', () => {
    const _: isBoolean<""> = false;
    expect(_).toEqual(_);
  });
  it('isBoolean<"a">', () => {
    const _: isBoolean<"a"> = true;
    expect(_).toEqual(_);
  });
  it("isBoolean<true>", () => {
    const _: isBoolean<true> = true;
    expect(_).toEqual(_);
  });
  it("isBoolean<false>", () => {
    const _: isBoolean<false> = false;
    expect(_).toEqual(_);
  });
  it("isBoolean<undefined>", () => {
    const _: isBoolean<undefined> = false;
    expect(_).toEqual(_);
  });
  it("isBoolean<null>", () => {
    const _: isBoolean<null> = false;
    expect(_).toEqual(_);
  });
  it("isBoolean<[]>", () => {
    const _: isBoolean<[]> = false;
    expect(_).toEqual(_);
  });
  it("isBoolean<[1]>", () => {
    const _: isBoolean<[1]> = true;
    expect(_).toEqual(_);
  });
  it("isBoolean<{ a: 1 }>", () => {
    const _: isBoolean<{ a: 1 }> = true;
    expect(_).toEqual(_);
  });
  it("isBoolean<() => void>", () => {
    const _: isBoolean<() => void> = true;
    expect(_).toEqual(_);
  });
  it("isBoolean<symbol>", () => {
    const _: isBoolean<symbol> = true;
    expect(_).toEqual(_);
  });
});
