import { slice } from "@/Pipe/slice";
import { pipe } from "@/Tool/pipe";

describe("slice", () => {
  it("returns the elements between start and end", () => {
    expect(pipe([1, 2, 3, 4], slice(1, 3))).toEqual([2, 3]);
  });

  it("counts a negative start from the end", () => {
    expect(pipe([1, 2, 3, 4], slice(-2))).toEqual([3, 4]);
  });

  it("returns a copy of the whole array when no index is given", () => {
    const original = [1, 2, 3];
    const result = pipe(original, slice());
    expect(result).toEqual([1, 2, 3]);
    expect(result).not.toBe(original);
  });
});
