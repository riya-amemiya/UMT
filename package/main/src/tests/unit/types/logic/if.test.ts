import type { IF } from "$/logic/if";

describe("IF", () => {
  it("IF<true, 1, 2>", () => {
    const _: IF<true, 1, 2> = 1;
    expect(_).toEqual(_);
  });
  it("IF<false, 1, 2>", () => {
    const _: IF<false, 1, 2> = 2;
    expect(_).toEqual(_);
  });
});
