import type { Pop } from "$/logic/pop";

describe("Pop", () => {
  it("Pop<[1, 2, 3]>", () => {
    const _: Pop<[1, 2, 3]> = [1, 2];
    expect(_).toEqual(_);
  });
  it("Pop<[]>", () => {
    const _: [Pop<[]>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});
