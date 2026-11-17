import { defaultTo } from "@/Pipe/defaultTo";
import { filter } from "@/Pipe/filter";
import { fold } from "@/Pipe/fold";
import { join } from "@/Pipe/join";
import { split } from "@/Pipe/split";
import { toUpperCase } from "@/Pipe/toUpperCase";
import { transform } from "@/Pipe/transform";
import { trim } from "@/Pipe/trim";
import { pipe } from "@/Tool/pipe";

describe("Integration test for pipe with Pipe operators", () => {
  const splitList = (value: string | undefined): string[] =>
    pipe(
      value,
      defaultTo(""),
      split(","),
      transform((item) => item.trim()),
      filter((item) => item.length > 0),
    );

  it("splits a comma separated list into trimmed items", () => {
    expect(splitList(" a, b ,,c ")).toEqual(["a", "b", "c"]);
  });

  it("returns an empty array for undefined", () => {
    expect(splitList(undefined)).toEqual([]);
  });

  it("transforms a single value", () => {
    expect(
      pipe(
        2,
        transform((x) => x * 2),
      ),
    ).toBe(4);
  });

  it("chains array and string operators", () => {
    const result: string = pipe(
      " a , b , c ",
      split(","),
      transform(trim()),
      transform(toUpperCase()),
      join("-"),
    );
    expect(result).toBe("A-B-C");
  });

  it("folds the transformed items", () => {
    const total: number = pipe(
      "1,2,3",
      split(","),
      transform(Number),
      fold((sum, x) => sum + x, 0),
    );
    expect(total).toBe(6);
  });
});
