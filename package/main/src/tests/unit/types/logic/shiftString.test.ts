import type { ShiftString } from "$/logic/shiftString";

describe("ShiftString", () => {
  it('ShiftString<"abc">', () => {
    const _: ShiftString<"abc"> = "bc";
    expect(_).toEqual(_);
  });
  it('ShiftString<"">', () => {
    const _: [ShiftString<"">] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
