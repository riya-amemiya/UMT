// chunkArray.test.ts

import { chunk } from "@/Array/chunk";

describe("chunkArray function", () => {
  it("splits an array into chunks of size n", () => {
    const input = [0, 1, 2, 3, 4, 5, 6, 7];
    const n = 3;
    const expected = [
      [0, 1, 2],
      [3, 4, 5],
      [6, 7],
    ];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
  });

  it("handles empty arrays", () => {
    const input: number[] = [];
    const n = 3;
    const expected: number[][] = [];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
  });

  it("handles n larger than array length", () => {
    const input = [0, 1, 2];
    const n = 5;
    const expected = [[0, 1, 2]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
  });
});
