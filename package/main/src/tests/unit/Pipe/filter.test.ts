import { filter } from "@/Pipe/filter";
import { pipe } from "@/Tool/pipe";

describe("filter", () => {
  it("keeps the elements matching the predicate", () => {
    expect(
      pipe(
        [1, 2, 3, 4],
        filter((x) => x % 2 === 0),
      ),
    ).toEqual([2, 4]);
  });

  it("returns an empty array when no element matches", () => {
    expect(
      pipe(
        [1, 3],
        filter((x) => x % 2 === 0),
      ),
    ).toEqual([]);
  });

  it("passes the index to the predicate", () => {
    expect(
      pipe(
        ["a", "b", "c"],
        filter((_, index) => index > 0),
      ),
    ).toEqual(["b", "c"]);
  });

  it("does not modify the original array", () => {
    const original = [1, 2, 3, 4];
    pipe(
      original,
      filter((x) => x > 2),
    );
    expect(original).toEqual([1, 2, 3, 4]);
  });
});
