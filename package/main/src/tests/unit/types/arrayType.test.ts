import type { Chunk, ChunkArrayType } from "$/array/chunk";
import type { CompareFunction } from "$/array/compareFunction";
import type { ZIP, ZipArrayType } from "$/array/zip";

describe("arrayType", () => {
  it("ChunkArrayType", () => {
    const _: ChunkArrayType<[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3> = [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9],
      [10],
    ];
    expect(_).toEqual(_);
  });
  it("Chunk", () => {
    const _: [Chunk<[1, 2, 3, 4], 2>, Chunk<[], 2>, Chunk<number[], 2>] = [
      [
        [1, 2],
        [3, 4],
      ],
      [],
      [[1, 2], [3]],
    ];
    expect(_).toEqual(_);
  });
  it("ZipArrayType", () => {
    const _: ZipArrayType<[[1, 2], ["a", "b"]]> = [
      [1, "a"],
      [2, "b"],
    ];
    expect(_).toEqual(_);
  });
  it("ZIP", () => {
    const _: [
      ZIP<[[1], [true]]>,
      [ZIP<[1]>] extends [never] ? true : false,
      ZIP<number[][]>,
    ] = [[[1, true]], true, [[1]]];
    expect(_).toEqual(_);
  });
  it("CompareFunction", () => {
    const _: CompareFunction<number> = (a, b) => a - b;
    expect(_(2, 1)).toBe(1);
  });
});
