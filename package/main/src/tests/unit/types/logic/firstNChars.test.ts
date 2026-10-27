import type { FirstNChars } from "$/logic/firstNChars";

describe("FirstNChars", () => {
  it('FirstNChars<"hello", 2>', () => {
    const _: FirstNChars<"hello", 2> = "he";
    expect(_).toEqual(_);
  });
  it('FirstNChars<"hello", 0>', () => {
    const _: FirstNChars<"hello", 0> = "";
    expect(_).toEqual(_);
  });
  it('FirstNChars<"he", 3>', () => {
    const _: [FirstNChars<"he", 3>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
