import type { IsFloat } from "$/logic/isFloat";

describe("IsFloat", () => {
  it("IsFloat<1.5>", () => {
    const _: IsFloat<1.5> = true;
    expect(_).toEqual(_);
  });
  it("IsFloat<1>", () => {
    const _: IsFloat<1> = false;
    expect(_).toEqual(_);
  });
  it('IsFloat<"1.5">', () => {
    const _: IsFloat<"1.5"> = false;
    expect(_).toEqual(_);
  });
});
