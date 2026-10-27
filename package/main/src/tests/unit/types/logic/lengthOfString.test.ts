import type { LengthOfString } from "$/logic/lengthOfString";

describe("LengthOfString", () => {
  it('LengthOfString<"abc">', () => {
    const _: LengthOfString<"abc"> = 3;
    expect(_).toEqual(_);
  });
  it('LengthOfString<"">', () => {
    const _: LengthOfString<""> = 0;
    expect(_).toEqual(_);
  });
});
