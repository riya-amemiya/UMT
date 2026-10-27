import type { Shift } from "$/logic/shift";

describe("Shift", () => {
  it("Shift<[1, 2, 3]>", () => {
    const _: Shift<[1, 2, 3]> = [2, 3];
    expect(_).toEqual(_);
  });
  it("Shift<[]>", () => {
    const _: [Shift<[]>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
