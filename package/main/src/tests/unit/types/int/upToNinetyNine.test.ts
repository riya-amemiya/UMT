import type { UpToNinetyNine } from "$/int/upToNinetyNine";

describe("UpToNinetyNine", () => {
  it("UpToNinetyNine", () => {
    const _: UpToNinetyNine[] = [90, 91, 92, 93, 94, 95, 96, 97, 98, 99];
    expect(_).toEqual(_);
  });
  it("89 extends UpToNinetyNine", () => {
    const _: 89 extends UpToNinetyNine ? true : false = false;
    expect(_).toEqual(_);
  });
  it("100 extends UpToNinetyNine", () => {
    const _: 100 extends UpToNinetyNine ? true : false = false;
    expect(_).toEqual(_);
  });
});
