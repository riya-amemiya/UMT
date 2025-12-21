import { compact } from "@/Array/compact";
import _ from "lodash";

describe("compact function", () => {
  test("should handle array with mixed booleans and numbers", () => {
    expect(compact([0, 1, false, 2, "", 3])).toEqual([1, 2, 3]);
    expect(_.compact([0, 1, false, 2, "", 3])).toEqual([1, 2, 3]);
  });

  test("should handle array with all truthy values", () => {
    expect(compact([1, 2, 3, "a", true])).toEqual([1, 2, 3, "a", true]);
    expect(_.compact([1, 2, 3, "a", true])).toEqual([1, 2, 3, "a", true]);
  });

  test("should handle array with all falsy values", () => {
    expect(compact([0, false, "", null, undefined, Number.NaN])).toEqual([]);
    expect(_.compact([0, false, "", null, undefined, Number.NaN])).toEqual([]);
  });

  test("should handle empty array", () => {
    expect(compact([])).toEqual([]);
    expect(_.compact([])).toEqual([]);
  });

  test("should handle array with null and undefined", () => {
    expect(compact([null, undefined, "hello"])).toEqual(["hello"]);
    expect(_.compact([null, undefined, "hello"])).toEqual(["hello"]);
  });

  test("should handle array with falsy values like 0 and empty string", () => {
    expect(compact([0, 1, "", 2, " ", 3])).toEqual([1, 2, " ", 3]);
    expect(_.compact([0, 1, "", 2, " ", 3])).toEqual([1, 2, " ", 3]);
  });

  test("should handle array with truthy values like objects and arrays", () => {
    expect(compact([{}, [], 0, false, "text"])).toEqual([{}, [], "text"]);
    expect(_.compact([{}, [], 0, false, "text"])).toEqual([{}, [], "text"]);
  });

  test("should handle array with NaN values", () => {
    expect(compact([Number.NaN, 1, 2, Number.NaN])).toEqual([1, 2]);
    expect(_.compact([Number.NaN, 1, 2, Number.NaN])).toEqual([1, 2]);
  });

  test("should handle array with various data types", () => {
    expect(
      compact([
        0,
        1,
        "a",
        "",
        null,
        undefined,
        [],
        {},
        false,
        true,
        Number.NaN,
      ]),
    ).toEqual([1, "a", [], {}, true]);
    expect(
      _.compact([
        0,
        1,
        "a",
        "",
        null,
        undefined,
        [],
        {},
        false,
        true,
        Number.NaN,
      ]),
    ).toEqual([1, "a", [], {}, true]);
  });

  test("should return a new array without modifying the original", () => {
    const original = [0, 1, false, 2];
    const result = compact(original);
    expect(result).toEqual([1, 2]);
    expect(original).toEqual([0, 1, false, 2]);

    const lodashOriginal = [0, 1, false, 2];
    const lodashResult = _.compact(lodashOriginal);
    expect(lodashResult).toEqual([1, 2]);
    expect(lodashOriginal).toEqual([0, 1, false, 2]);
  });

  test("should treat -0 as falsy", () => {
    expect(compact([-0, 1, 2])).toEqual([1, 2]);
    expect(_.compact([-0, 1, 2])).toEqual([1, 2]);
  });

  test("should handle Infinity as truthy", () => {
    expect(
      compact([0, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]),
    ).toEqual([Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]);
    expect(
      _.compact([0, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]),
    ).toEqual([Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]);
  });
});
