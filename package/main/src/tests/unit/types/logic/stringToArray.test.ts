import type { StringToArray } from "$/logic/stringToArray";

describe("StringToArray", () => {
  it('StringToArray<"abc">', () => {
    const _: StringToArray<"abc"> = ["a", "b", "c"];
    expect(_).toEqual(_);
  });
  it('StringToArray<"">', () => {
    const _: StringToArray<""> = [];
    expect(_).toEqual(_);
  });
});
