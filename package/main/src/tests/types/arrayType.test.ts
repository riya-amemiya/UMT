import type { ChunkArrayType } from "@/types/arrayType";

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
});
