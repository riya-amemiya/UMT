import type { StringToUnion } from "$/logic/stringToUnion";

describe("StringToUnion", () => {
  it('StringToUnion<"abc">', () => {
    const _: StringToUnion<"abc">[] = ["a", "b", "c"];
    expect(_).toEqual(_);
  });
  it('"d" extends StringToUnion<"abc">', () => {
    const _: "d" extends StringToUnion<"abc"> ? true : false = false;
    expect(_).toEqual(_);
  });
  it('StringToUnion<"">', () => {
    const _: [StringToUnion<"">] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
