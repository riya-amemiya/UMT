import type { ArrayToUnion } from "$/logic/arrayToUnion";

describe("ArrayToUnion", () => {
  it('ArrayToUnion<[1, "a"]>', () => {
    const _: ArrayToUnion<[1, "a"]>[] = [1, "a"];
    expect(_).toEqual(_);
  });
  it('2 extends ArrayToUnion<[1, "a"]>', () => {
    const _: 2 extends ArrayToUnion<[1, "a"]> ? true : false = false;
    expect(_).toEqual(_);
  });
});
