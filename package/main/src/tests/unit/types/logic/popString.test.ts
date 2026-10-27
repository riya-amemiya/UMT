import type { PopString } from "$/logic/popString";

describe("PopString", () => {
  it('PopString<"abc">', () => {
    const _: PopString<"abc"> = "ab";
    expect(_).toEqual(_);
  });
  it('PopString<"a">', () => {
    const _: PopString<"a"> = "";
    expect(_).toEqual(_);
  });
  it('PopString<"">', () => {
    const _: [PopString<"">] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
