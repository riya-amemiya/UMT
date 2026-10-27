import type { ToNumber } from "$/logic/toNumber";

describe("ToNumber", () => {
  it('ToNumber<"42">', () => {
    const _: ToNumber<"42"> = 42;
    expect(_).toEqual(_);
  });
  it('ToNumber<"a">', () => {
    const _: [ToNumber<"a">] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
