import { find } from "@/Pipe/find";
import { pipe } from "@/Tool/pipe";

describe("find", () => {
  it("returns the first element matching the predicate", () => {
    expect(
      pipe(
        [1, 2, 3],
        find((x) => x > 1),
      ),
    ).toBe(2);
  });

  it("returns undefined when no element matches", () => {
    expect(
      pipe(
        [1, 2, 3],
        find((x) => x > 5),
      ),
    ).toBeUndefined();
  });

  it("passes the index to the predicate", () => {
    expect(
      pipe(
        ["a", "b", "c"],
        find((_, index) => index === 2),
      ),
    ).toBe("c");
  });
});
