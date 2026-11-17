import { fold } from "@/Pipe/fold";
import { pipe } from "@/Tool/pipe";

describe("fold", () => {
  it("reduces the array to a single value", () => {
    expect(
      pipe(
        [1, 2, 3],
        fold((sum, x) => sum + x, 0),
      ),
    ).toBe(6);
  });

  it("returns the initial value for an empty array", () => {
    expect(
      pipe(
        [] as number[],
        fold((sum, x) => sum + x, 10),
      ),
    ).toBe(10);
  });

  it("accumulates into a different type", () => {
    expect(
      pipe(
        [1, 2, 3],
        fold((text, x) => `${text}${x}`, ""),
      ),
    ).toBe("123");
  });

  it("passes the index to the reducer", () => {
    expect(
      pipe(
        ["a", "b"],
        fold(
          (indexes: number[], _, index) => [...indexes, index],
          [] as number[],
        ),
      ),
    ).toEqual([0, 1]);
  });
});
