import type { Length } from "$/logic/length";

describe("Length", () => {
  it("Length<[1, 2, 3]>", () => {
    const _: Length<[1, 2, 3]> = 3;
    expect(_).toEqual(_);
  });
  it("Length<[]>", () => {
    const _: Length<[]> = 0;
    expect(_).toEqual(_);
  });
});
