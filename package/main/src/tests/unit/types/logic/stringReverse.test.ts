import type { StringReverse } from "$/logic/stringReverse";

describe("StringReverse", () => {
  it('StringReverse<"abc">', () => {
    const _: StringReverse<"abc"> = "cba";
    expect(_).toEqual(_);
  });
  it('StringReverse<"">', () => {
    const _: StringReverse<""> = "";
    expect(_).toEqual(_);
  });
});
