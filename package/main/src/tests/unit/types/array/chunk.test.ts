import type { Chunk, ChunkArrayType } from "$/array/chunk";

describe("Chunk", () => {
  it("Chunk<[1, 2, 3, 4, 5], 2>", () => {
    const _: Chunk<[1, 2, 3, 4, 5], 2> = [[1, 2], [3, 4], [5]];
    expect(_).toEqual(_);
  });
  it("Chunk<[1, 2, 3, 4], 2>", () => {
    const _: Chunk<[1, 2, 3, 4], 2> = [
      [1, 2],
      [3, 4],
    ];
    expect(_).toEqual(_);
  });
  it("Chunk<[], 2>", () => {
    const _: Chunk<[], 2> = [];
    expect(_).toEqual(_);
  });
  it("Chunk<[], 2, [], [[1]]>", () => {
    const _: Chunk<[], 2, [], [[1]]> = [[1]];
    expect(_).toEqual(_);
  });
  it("Chunk<number[], 2>", () => {
    const _: Chunk<number[], 2> = [[1, 2], [3]];
    expect(_).toEqual(_);
  });
  it("ChunkArrayType<[1, 2, 3], 2>", () => {
    const _: ChunkArrayType<[1, 2, 3], 2> = [[1, 2], [3]];
    expect(_).toEqual(_);
  });
});
