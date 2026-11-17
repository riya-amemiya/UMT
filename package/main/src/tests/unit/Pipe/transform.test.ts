import { transform } from "@/Pipe/transform";
import { pipe } from "@/Tool/pipe";

describe("transform", () => {
  it("applies the function to each element of an array", () => {
    expect(
      pipe(
        [1, 2, 3],
        transform((x) => x * 2),
      ),
    ).toEqual([2, 4, 6]);
  });

  it("applies the function to a non-array value", () => {
    expect(
      pipe(
        2,
        transform((x) => x * 2),
      ),
    ).toBe(4);
  });

  it("applies the function to an object value as a whole", () => {
    expect(
      pipe(
        { count: 1 },
        transform((x) => x.count + 1),
      ),
    ).toBe(2);
  });

  it("returns an empty array for an empty array", () => {
    expect(
      pipe(
        [] as number[],
        transform((x) => x * 2),
      ),
    ).toEqual([]);
  });

  it("passes only the element to the function", () => {
    expect(pipe(["10", "10", "10"], transform(Number.parseInt))).toEqual([
      10, 10, 10,
    ]);
  });

  it("does not modify the original array", () => {
    const original = [1, 2, 3];
    pipe(
      original,
      transform((x) => x * 2),
    );
    expect(original).toEqual([1, 2, 3]);
  });
});
