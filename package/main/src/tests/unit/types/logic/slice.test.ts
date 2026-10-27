import type { Slice } from "$/logic/slice";

describe("Slice", () => {
  it('Slice<"hello", 1, 3>', () => {
    const _: Slice<"hello", 1, 3> = "el";
    expect(_).toEqual(_);
  });
  it('Slice<"hello", 0, 0>', () => {
    const _: Slice<"hello", 0, 0> = "";
    expect(_).toEqual(_);
  });
  it('Slice<"ab", 0, 5>', () => {
    const _: Slice<"ab", 0, 5> = "ab";
    expect(_).toEqual(_);
  });
});
