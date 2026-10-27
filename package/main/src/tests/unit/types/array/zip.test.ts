import type { ZIP, ZipArrayType } from "$/array/zip";

describe("ZIP", () => {
  it("ZIP<[[1], [true]]>", () => {
    const _: ZIP<[[1], [true]]> = [[1, true]];
    expect(_).toEqual(_);
  });
  it("ZIP<[]>", () => {
    const _: ZIP<[]> = [];
    expect(_).toEqual(_);
  });
  it("ZIP<[1]>", () => {
    const _: [ZIP<[1]>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
  it("ZIP<number[][]>", () => {
    const _: ZIP<number[][]> = [[1]];
    expect(_).toEqual(_);
  });
  it('ZipArrayType<[[1, 2], ["a", "b"]]>', () => {
    const _: ZipArrayType<[[1, 2], ["a", "b"]]> = [
      [1, "a"],
      [2, "b"],
    ];
    expect(_).toEqual(_);
  });
});
