import { unzip } from "@/Array/unzip";
import { zip } from "@/Array/zip";

describe("unzip", () => {
  test("splits tuples into one array per position", () => {
    expect(
      unzip<[number, string]>([
        [1, "a"],
        [2, "b"],
        [3, "c"],
      ]),
    ).toEqual([
      [1, 2, 3],
      ["a", "b", "c"],
    ]);
  });

  test("is the inverse of zip", () => {
    const numbers = [1, 2, 3];
    const letters = ["a", "b", "c"];
    const flags = [true, false, true];
    expect(unzip(zip(numbers, letters, flags))).toEqual([
      numbers,
      letters,
      flags,
    ]);
  });

  test("truncates to the shortest tuple", () => {
    expect(
      unzip([
        [1, "a", true],
        [2, "b"],
        [3, "c", false],
      ]),
    ).toEqual([
      [1, 2, 3],
      ["a", "b", "c"],
    ]);
  });

  test("returns an empty array for an empty input", () => {
    expect(unzip([])).toEqual([]);
  });

  test("returns an empty array when a tuple is empty", () => {
    expect(unzip([[1], []])).toEqual([]);
  });

  test("keeps undefined and null values", () => {
    expect(
      unzip([
        [null, undefined],
        [1, 2],
      ]),
    ).toEqual([
      [null, 1],
      [undefined, 2],
    ]);
  });
});
