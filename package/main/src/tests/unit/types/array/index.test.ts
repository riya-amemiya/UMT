import type { ChunkArrayType, ZipArrayType } from "$/array/index";

describe("array index", () => {
  it("ChunkArrayType<[1, 2, 3], 2>", () => {
    const _: ChunkArrayType<[1, 2, 3], 2> = [[1, 2], [3]];
    expect(_).toEqual(_);
  });
  it("ChunkArrayType<[], 2>", () => {
    const _: ChunkArrayType<[], 2> = [];
    expect(_).toEqual(_);
  });
  it("ZipArrayType<[[1], [true]]>", () => {
    const _: ZipArrayType<[[1], [true]]> = [[1, true]];
    expect(_).toEqual(_);
  });
});
