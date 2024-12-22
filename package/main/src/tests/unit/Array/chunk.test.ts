import { chunk } from "@/Array/chunk";
import _ from "lodash";

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
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles empty arrays", () => {
    const input: number[] = [];
    const n = 3;
    const expected: number[][] = [];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("handles n larger than array length", () => {
    const input = [0, 1, 2];
    const n = 5;
    const expected = [[0, 1, 2]];
    const output = chunk(input, n);
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("type checks", () => {
    const input: [0, 1, 2, 3, 4, 5, 6, 7] = [0, 1, 2, 3, 4, 5, 6, 7];
    const n = 3;
    const output = chunk(input, n);
    const expected: typeof output = [
      [0, 1, 2],
      [3, 4, 5],
      [6, 7],
    ];
    expect(output).toEqual(expected);
    const lodashOutput = _.chunk(input, n);
    expect(lodashOutput).toEqual(expected);
  });

  it("does not mutate the input array", () => {
    const input = [0, 1, 2, 3, 4, 5, 6, 7];
    const n = 3;
    chunk(input, n);
    expect(input).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
    _.chunk(input, n);
    expect(input).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
  });
});
