import { flatMap } from "@/Pipe/flatMap";
import { pipe } from "@/Tool/pipe";

describe("flatMap", () => {
  it("flattens the arrays returned for each element", () => {
    expect(
      pipe(
        [1, 2],
        flatMap((x) => [x, x * 10]),
      ),
    ).toEqual([1, 10, 2, 20]);
  });

  it("keeps non-array return values as elements", () => {
    expect(
      pipe(
        [1, 2],
        flatMap((x) => x * 2),
      ),
    ).toEqual([2, 4]);
  });

  it("flattens only one level", () => {
    expect(
      pipe(
        [1],
        flatMap((x) => [[x]]),
      ),
    ).toEqual([[1]]);
  });

  it("passes the index to the function", () => {
    expect(
      pipe(
        ["a", "b"],
        flatMap((x, index) => [x, index]),
      ),
    ).toEqual(["a", 0, "b", 1]);
  });
});
