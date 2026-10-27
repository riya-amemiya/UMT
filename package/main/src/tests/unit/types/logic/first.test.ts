import type { First } from "$/logic/first";

describe("First", () => {
  it("First<[1, 2]>", () => {
    const _: First<[1, 2]> = 1;
    expect(_).toEqual(_);
  });
  it("First<[]>", () => {
    const _: [First<[]>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
